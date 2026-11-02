// Waymap: client-side waypoints, in the world and on the minimap.
//
// A waypoint is a named point (`name`, `x`, `y`, `z`) in one dimension, with a
// colour. Waypoints are stored per context (server or singleplayer world) in
// the mod's own folder (`horizon-mods/.horizon-mod-data/waymap/`), so they
// survive restarts, world changes, and updates.
//
// The markers are drawn from `locator_camera`, the camera Horizon used for the
// locator bar (position, yaw, pitch and field of view), so they line up with
// the world without reading the renderer's matrices. The minimap dots are
// drawn in `extras_hud::waymap_minimap` from the view the minimap was
// composed from; this module hands it the waypoints once a frame.
//
// Commands: `/wp add [name]`, `/wp del <name>`, `/wp list`, `/wp rename <old>
// <new>`, `/wp color <name> <colour>`, `/wp clear`, `/wp help`.
// Keys: N adds a waypoint here, B shows the list panel (rebindable in
// Options > Controls > Waymap).

use super::Game;
use horizon_client::extras::Corner;
use horizon_client::extras_hud::Corners;
use horizon_client::hud::{HudAtlas, HudBatch, rgba};
use std::cell::RefCell;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// The mod's id: its folder, its settings files and its keys.
pub(super) const MOD_ID: &str = "waymap";
const NAME: &str = "Waymap";
const VERSION: &str = "0.1.2";
const DESCRIPTION: &str =
    "Waypoints in the world and on the minimap: /wp add, /wp list, /wp del.";
/// Maximum waypoints per context; more would only clutter the map.
const MAX_WAYPOINTS: usize = 512;
/// Player eye height while standing: a waypoint is kept at eye level, so a
/// marker sits on the crosshair when the player stands where it was set.
const EYE: f64 = 1.62;
/// The colours `/wp color` knows, cycled by the waypoint's index.
const COLORS: [(&str, u32); 12] = [
    ("white", 0xFF_FFFF),
    ("red", 0xFF_5555),
    ("orange", 0xFF_A033),
    ("yellow", 0xFF_EE55),
    ("green", 0x66_EE66),
    ("cyan", 0x55_EEEE),
    ("blue", 0x66_99_FF),
    ("purple", 0xBB_88_FF),
    ("pink", 0xFF_88_CC),
    ("gray", 0xBB_BB_BB),
    ("brown", 0xC8_8A_66),
    ("lime", 0xAA_FF_55),
];

#[derive(Clone, Debug)]
struct Waypoint {
    name: String,
    /// Eye-level position, so a marker on a block lands on the crosshair.
    x: f64,
    y: f64,
    z: f64,
    /// `minecraft:overworld`, `minecraft:the_nether`, `minecraft:the_end`.
    dim: String,
    /// `0xRRGGBB`.
    color: u32,
}

/// Context key identifying the current world/server.
/// Multiplayer: "server:<host:port>"
/// Singleplayer: "world:<folder_name>"
#[derive(Clone, Debug, PartialEq, Eq)]
enum ContextKey {
    Multiplayer(String),
    Singleplayer(String),
}

impl ContextKey {
    fn file_name(&self) -> String {
        match self {
            ContextKey::Multiplayer(addr) => {
                // Sanitize server address for filename
                let safe = addr.replace(':', "_").replace('/', "_").replace('\\', "_");
                format!("server_{}.txt", safe)
            }
            ContextKey::Singleplayer(folder) => {
                // Sanitize folder name for filename
                let safe = folder.replace('/', "_").replace('\\', "_");
                format!("world_{}.txt", safe)
            }
        }
    }

    fn display_name(&self) -> String {
        match self {
            ContextKey::Multiplayer(addr) => format!("Server: {}", addr),
            ContextKey::Singleplayer(folder) => format!("World: {}", folder),
        }
    }
}

/// The minimap's view type from the render crate.
type MiniView = horizon_client::minimap::View;

struct State {
    /// The current context key (None = not in a world).
    context: Option<ContextKey>,
    /// The waypoints for the current context.
    waypoints: Vec<Waypoint>,
    /// The waypoints were loaded for the current context.
    loaded: bool,
    /// Mod settings.
    markers: bool,
    labels: bool,
    panel: bool,
    /// This frame's minimap view, from `update_minimap`.
    mini: Option<MiniView>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            context: None,
            waypoints: Vec::new(),
            loaded: false,
            markers: true,
            labels: true,
            panel: true,
            mini: None,
        }
    }
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with_borrow_mut(f)
}

/// Determine the current context key from the game state.
fn current_context(game: &Game<'_>) -> Option<ContextKey> {
    if game.native_is_multiplayer() {
        game.native_server_address().map(ContextKey::Multiplayer)
    } else {
        game.native_world_folder().map(ContextKey::Singleplayer)
    }
}

/// Get the data directory for a specific context.
fn context_dir(base: &Path, ctx: &ContextKey) -> PathBuf {
    base.join(ctx.file_name())
}

/// Safely write waypoints to a file using atomic rename.
fn save_waypoints(dir: &Path, file_name: &str, waypoints: &[Waypoint]) -> Result<(), String> {
    let path = dir.join(file_name);
    let mut text = String::new();
    for w in waypoints {
        writeln!(
            text,
            "{}\t{:.3}\t{:.3}\t{:.3}\t{:06x}\t{}",
            w.dim, w.x, w.y, w.z, w.color, w.name
        ).map_err(|e| format!("format error: {}", e))?;
    }
    fs::create_dir_all(dir).map_err(|e| format!("create dir: {}", e))?;
    
    // Write to temp file first, then atomically rename
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, text).map_err(|e| format!("write temp: {}", e))?;
    fs::rename(&temp_path, &path).map_err(|e| format!("rename: {}", e))?;
    Ok(())
}

/// Load waypoints from a file.
fn load_waypoints(dir: &Path, file_name: &str) -> Vec<Waypoint> {
    let path = dir.join(file_name);
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Waypoint> {
    let mut parts = line.splitn(6, '\t');
    let dim = parts.next()?.to_owned();
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    let z = parts.next()?.parse().ok()?;
    let color = u32::from_str_radix(parts.next()?, 16).ok()?;
    let name = parts.next()?.to_owned();
    Some(Waypoint {
        name,
        x,
        y,
        z,
        dim,
        color,
    })
}

/// Migrate legacy waypoints.txt to new per-context files.
fn migrate_legacy(game: &Game<'_>) {
    let dir = game.native_data_dir(MOD_ID);
    let legacy_path = dir.join("waypoints.txt");
    if !legacy_path.exists() {
        return;
    }

    // Read legacy waypoints
    let Ok(text) = fs::read_to_string(&legacy_path) else {
        return;
    };
    let waypoints: Vec<Waypoint> = text.lines().filter_map(parse_line).collect();
    if waypoints.is_empty() {
        // Nothing to migrate, just remove empty file
        let _ = fs::remove_file(&legacy_path);
        return;
    }

    // Try to determine context from first waypoint's dimension
    // This is a best-effort migration
    let ctx = if let Some(first) = waypoints.first() {
        // We can't know the exact context, so we'll use a generic name
        // The player will need to manually reorganize if needed
        ContextKey::Singleplayer("legacy".to_string())
    } else {
        return;
    };

    // Save to new context file
    let dir = game.native_data_dir(MOD_ID);
    let file_name = ctx.file_name();
    let _ = save_waypoints(&dir, &file_name, &waypoints);

    // Rename legacy file to backup (don't delete in case migration failed)
    let backup = dir.join("waypoints.txt.legacy_backup");
    let _ = fs::rename(&legacy_path, &backup);
}

/// Get the current context and handle context switching.
fn update_context(game: &Game<'_>) {
    let new_ctx = current_context(game);
    with(|s| {
        // Check if context changed
        if s.context.as_ref() != new_ctx.as_ref() {
            // Save old context
            if let Some(old_ctx) = s.context.take() {
                if s.loaded && !s.waypoints.is_empty() {
                    let dir = game.native_data_dir(MOD_ID);
                    let file_name = old_ctx.file_name();
                    let _ = save_waypoints(&game.native_data_dir(MOD_ID), &file_name, &s.waypoints);
                }
            }
            // Load new context
            s.context = new_ctx.clone();
            s.loaded = false;
            s.waypoints.clear();
            if let Some(ctx) = new_ctx {
                let dir = game.native_data_dir(MOD_ID);
                s.waypoints = load_waypoints(&dir, &ctx.file_name());
                s.loaded = true;
            }
        }
    });
}

/// Called from `native_frame` (once a frame, in a world or not): handles
/// context switching and hands this frame's minimap dots to the library overlay.
pub(super) fn frame(game: &mut Game<'_>) {
    if !game.native_window() {
        return;
    }
    
    // Handle context switching
    update_context(game);
    
    // Skip minimap dots when chat is open (they'd be hidden anyway)
    if game.screen == crate::game::Screen::Chat {
        with(|s| {
            s.mini = None;
        });
        return;
    }

    ensure_loaded(game);
    
    with(|s| {
        s.mini = None;
        if let Some(p) = game.native_player() {
            // Update dimension for minimap dots
            // (kept for minimap dots)
        }
    });

    // Send minimap dots for current context
    let dots = match game.native_player() {
        Some(p) if game.native_on(MOD_ID) => {
            let dim = p.dimension;
            with(|s| {
                s.waypoints
                    .iter()
                    .filter(|w| w.dim == dim)
                    .map(|w| ([w.x, w.z], w.color))
                    .collect()
            })
        }
        _ => Vec::new(),
    };
    horizon_client::extras_hud::waymap_minimap::set_dots(dots);
}

/// Ensure waypoints are loaded for the current context.
fn ensure_loaded(game: &Game<'_>) {
    with(|s| {
        if s.loaded {
            return;
        }
        let ctx = match current_context(game) {
            Some(ctx) => ctx,
            None => return,
        };
        let dir = game.native_data_dir(MOD_ID);
        s.waypoints = load_waypoints(&dir, &ctx.file_name());
        s.loaded = true;
    });
}

/// The mod's row in Options > Mods (from `native_mods`).
pub(super) fn listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: MOD_ID.into(),
        title: format!("{NAME} {VERSION}"),
        description: DESCRIPTION.into(),
        keys: vec![
            [
                "add".into(),
                "Waypoint here".into(),
                "key.keyboard.n".into(),
            ],
            [
                "panel".into(),
                "Waypoint list".into(),
                "key.keyboard.b".into(),
            ],
        ],
    });
}

/// The buttons of the mod's settings page (from `native_settings`).
pub(super) fn settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != MOD_ID {
        return;
    }
    ensure_loaded(game);
    let (markers, labels, panel) = with(|s| (s.markers, s.labels, s.panel));
    buttons.push(format!("Markers: {}", on_off(markers)));
    buttons.push(format!("Labels: {}", on_off(labels)));
    buttons.push(format!("List panel: {}", on_off(panel)));
}

fn on_off(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

/// A click on the settings page (from `native_settings_click`).
pub(super) fn click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != MOD_ID {
        return;
    }
    ensure_loaded(game);
    let mut message = String::new();
    with(|s| match button {
        0 => {
            s.markers = !s.markers;
            message = format!("Markers: {}", on_off(s.markers));
        }
        1 => {
            s.labels = !s.labels;
            message = format!("Labels: {}", on_off(s.labels));
        }
        2 => {
            s.panel = !s.panel;
            message = format!("List panel: {}", on_off(s.panel));
        }
        _ => return,
    });
    let (key, on) = match button {
        0 => ("markers", with(|s| s.markers)),
        1 => ("labels", with(|s| s.labels)),
        2 => ("panel", with(|s| s.panel)),
        _ => return,
    };
    game.native_set_setting(MOD_ID, key, on_off(on));
    game.native_chat(&format!("{NAME}: {message}"));
}

/// `/wp ...` (from `native_command`).
pub(super) fn command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
    if *ran || !game.native_window() || !game.native_on(MOD_ID) {
        return;
    }
    let Some(rest) = command
        .strip_prefix("wp")
        .filter(|rest| rest.is_empty() || rest.starts_with(' '))
    else {
        return;
    };
    ensure_loaded(game);
    *ran = true;
    let rest = rest.trim();
    let (sub, args) = rest.split_once(' ').unwrap_or((rest, ""));
    let args = args.trim();
    let message = match sub {
        "" | "help" => help(),
        "add" => add_here(game, (!args.is_empty()).then(|| clean_name(args))),
        "del" | "delete" | "remove" => remove(game, args),
        "list" => list(),
        "rename" => rename(game, args),
        "color" | "colour" => recolor(game, args),
        "clear" => clear(game),
        "set" | "edit" => edit(game, args),
        other => format!("Unknown: /wp {other}. /wp help shows commands."),
    };
    for line in message.lines() {
        game.native_chat(line);
    }
}

/// Save current context's waypoints to disk.
fn save_context(game: &Game<'_>) {
    with(|s| {
        if let (Some(ctx), true) = (s.context.as_ref(), s.loaded) {
            if !s.waypoints.is_empty() {
                let dir = game.native_data_dir(MOD_ID);
                let file_name = ctx.file_name();
                let _ = save_waypoints(&dir, &file_name, &s.waypoints);
            }
        }
    });
}

/// Help text for the /wp command.
fn help() -> String {
    format!(
        "{NAME} {VERSION}:\n\
         /wp add [name]          - Add waypoint at current position\n\
         /wp del <name>          - Delete waypoint\n\
         /wp list                - List all waypoints\n\
         /wp rename <old> <new>  - Rename waypoint (supports quoted names)\n\
         /wp color <name> <color> - Change waypoint color (name or #rrggbb)\n\
         /wp clear               - Clear all waypoints in current context\n\
         /wp edit <name>         - Show waypoint info\n\
         /wp help                - Show this help"
    )
}

/// Adds a waypoint at the player's eyes, or reports why not.
fn add_here(game: &mut Game<'_>, name: Option<String>) -> String {
    let Some(p) = game.native_player() else {
        return "No player in a world.".to_owned();
    };
    let mut message = String::new();
    with(|s| {
        if s.waypoints.len() >= MAX_WAYPOINTS {
            message = format!("Cannot have more than {MAX_WAYPOINTS} waypoints.");
            return;
        }
        let name = name.unwrap_or_else(|| auto_name(&s.waypoints));
        if let Some(old) = s.waypoints.iter().find(|w| w.name.eq_ignore_ascii_case(&name)) {
            message = format!(
                "'{}' already exists ({}, {}, {}). Use /wp del {} first.",
                old.name,
                old.x.floor(),
                old.y.floor(),
                old.z.floor(),
                old.name
            );
            return;
        }
        let color = default_color(s.waypoints.len());
        s.waypoints.push(Waypoint {
            name: name.clone(),
            x: p.x,
            y: p.y + EYE,
            z: p.z,
            dim: p.dimension.to_owned(),
            color,
        });
        // Save immediately
        if let Some(ctx) = s.context.as_ref() {
            let dir = game.native_data_dir(MOD_ID);
            let file_name = ctx.file_name();
            let _ = save_waypoints(&dir, &file_name, &s.waypoints);
        }
        message = format!(
            "Waypoint '{name}' set: {}, {}, {} ({}).",
            p.x.floor(),
            (p.y + EYE).floor(),
            p.z.floor(),
            color_name(color)
        );
    });
    message
}

fn remove(game: &Game<'_>, name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        return "Which waypoint? /wp del <name>".to_owned();
    }
    let name_trim = name.trim();
    let name_clean = if name_trim.starts_with('"') && name_trim.ends_with('"') && name_trim.len() > 1 {
        &name_trim[1..name_trim.len()-1]
    } else {
        name
    };
    let mut message = String::new();
    let mut should_save = false;
    with(|s| {
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(name_clean)) else {
            message = format!("No waypoint '{name_clean}'. Use /wp list to see all.");
            return;
        };
        let w = s.waypoints.remove(i);
        message = format!("Waypoint '{}' deleted.", w.name);
        should_save = s.context.is_some();
    });
    if should_save {
        save_context(game);
    }
    message
}

fn list() -> String {
    with(|s| {
        if s.waypoints.is_empty() {
            return format!("{NAME}: no waypoints. Use /wp add [name] to add one.");
        }
        let mut text = format!("{NAME}: {} waypoints", s.waypoints.len());
        if let Some(ctx) = s.context.as_ref() {
            text.push_str(&format!(" ({})", ctx.display_name()));
        }
        for (i, w) in s.waypoints.iter().enumerate().take(20) {
            let _ = write!(
                text,
                "\n{}. {} · {}, {}, {} · {} · {}",
                i + 1,
                w.name,
                w.x.floor(),
                w.y.floor(),
                w.z.floor(),
                short_dim(&w.dim),
                color_name(w.color)
            );
        }
        if s.waypoints.len() > 20 {
            let _ = write!(text, "\n... and {} more", s.waypoints.len() - 20);
        }
        text
    })
}

fn rename(game: &Game<'_>, args: &str) -> String {
    let args_trim = args.trim();
    if args_trim.is_empty() {
        return "Usage: /wp rename <old> <new>".to_owned();
    }
    if args_trim.starts_with('"') {
        let end_quote = args_trim.find('"').unwrap_or(args_trim.len());
        let old = args_trim[1..end_quote].trim();
        let rest = args_trim[end_quote+1..].trim();
        let new = if rest.starts_with('"') {
            rest[1..].trim().trim_end_matches('"').trim()
        } else {
            rest.trim()
        };
        if new.is_empty() || old.is_empty() {
            return "Usage: /wp rename <old> <new>".to_owned();
        }
        return do_rename(game, old, new);
    }
    if let Some(pos) = args_trim.find(' ') {
        let old = args_trim[..pos].trim();
        let new = args_trim[pos+1..].trim();
        if !old.is_empty() && !new.is_empty() {
            return do_rename(game, old, new);
        }
    }
    "Usage: /wp rename <old> <new>".to_owned()
}

fn do_rename(game: &Game<'_>, old: &str, new: &str) -> String {
    let new = clean_name(new);
    let mut message = String::new();
    let mut should_save = false;
    with(|s| {
        if s.waypoints.iter().any(|w| w.name.eq_ignore_ascii_case(&new)) {
            message = format!("'{new}' already exists.");
            return;
        }
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(old)) else {
            message = format!("No waypoint '{old}'.");
            return;
        };
        s.waypoints[i].name = new.clone();
        should_save = true;
        message = format!("'{old}' renamed to '{new}'.");
    });
    if should_save {
        save_context(game);
    }
    message
}

fn recolor(game: &Game<'_>, args: &str) -> String {
    let args_trim = args.trim();
    if args_trim.is_empty() {
        return colors_help();
    }
    let parts: Vec<&str> = args_trim.split(' ').collect();
    if parts.len() < 2 {
        return colors_help();
    }
    let color_str = parts[parts.len()-1].trim();
    let name = parts[..parts.len()-1].join(" ").trim().to_string();
    
    if name.is_empty() || color_str.is_empty() {
        return colors_help();
    }
    
    let Some(color) = parse_color(color_str) else {
        return format!("Unknown color '{}'. {}", color_str, colors_help());
    };
    
    let mut message = String::new();
    let mut should_save = false;
    with(|s| {
        let Some(w) = s.waypoints.iter_mut().find(|w| w.name.eq_ignore_ascii_case(&name)) else {
            message = format!("No waypoint '{}'.", name);
            return;
        };
        w.color = color;
        let wp_name = w.name.clone();
        should_save = true;
        message = format!("'{wp_name}' is now {}.", color_name(color));
    });
    if should_save {
        save_context(game);
    }
    message
}

fn colors_help() -> String {
    let names: Vec<&str> = COLORS.iter().map(|(name, _)| *name).collect();
    format!("Colors: {} or #rrggbb", names.join(", "))
}

fn clear(game: &Game<'_>) -> String {
    let mut message = String::new();
    let mut should_save = false;
    with(|s| {
        let n = s.waypoints.len();
        s.waypoints.clear();
        should_save = n > 0;
        message = if n == 0 {
            "No waypoints to clear.".to_owned()
        } else {
            format!("{n} waypoints cleared.")
        };
    });
    if should_save {
        save_context(game);
    }
    message
}

fn edit(game: &Game<'_>, args: &str) -> String {
    let mut message = String::new();
    with(|s| {
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(args.trim())) else {
            message = format!("No waypoint '{}'. Use /wp list to see all.", args.trim());
            return;
        };
        let w = &mut s.waypoints[i];
        message = format!(
            "Waypoint '{name}' info: /wp edit {name} [new_name] [color] · /wp del {name}",
            name = w.name
        );
    });
    message
}

fn clean_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    let cleaned = cleaned.trim();
    if cleaned.is_empty() {
        "waypoint".to_owned()
    } else {
        cleaned.chars().take(48).collect()
    }
}

fn auto_name(waypoints: &[Waypoint]) -> String {
    let mut next = 1;
    for w in waypoints {
        if let Some(n) = w.name.strip_prefix("WP ").and_then(|n| n.parse::<u32>().ok()) {
            next = next.max(n + 1);
        }
    }
    format!("WP {next}")
}

fn parse_color(text: &str) -> Option<u32> {
    let lower = text.to_ascii_lowercase();
    if let Some((_, color)) = COLORS.iter().find(|(name, _)| *name == lower) {
        return Some(*color);
    }
    let hex = lower.strip_prefix('#').unwrap_or(&lower);
    if hex.len() == 6 {
        return u32::from_str_radix(hex, 16).ok();
    }
    None
}

fn color_name(color: u32) -> String {
    COLORS
        .iter()
        .find(|(_, c)| *c == color)
        .map_or_else(|| format!("#{color:06x}"), |(name, _)| (*name).to_owned())
}

fn default_color(index: usize) -> u32 {
    COLORS[index % COLORS.len()].1
}

fn short_dim(dim: &str) -> &str {
    dim.strip_prefix("minecraft:").unwrap_or(dim)
}

/// Keys (from `native_key`): add here, toggle the list panel.
pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_window() || !game.native_on(MOD_ID) || !game.native_playing() {
        return;
    }
    ensure_loaded(game);
    if key == game.native_mod_key(MOD_ID, "add", "key.keyboard.n") {
        let message = add_here(game, None);
        game.native_chat(&message);
    } else if key == game.native_mod_key(MOD_ID, "panel", "key.keyboard.b") {
        let on = with(|s| {
            s.panel = !s.panel;
            s.panel
        });
        game.native_set_setting(MOD_ID, "panel", on_off(on));
    }
}

/// The HUD (from `native_hud`): markers in the world and the list panel. The
/// minimap dots are drawn by the library overlay.
pub(super) fn hud(game: &mut Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    if !game.native_on(MOD_ID) {
        return;
    }
    ensure_loaded(game);
    let Some(player) = game.native_player() else {
        return;
    };
    let dim = player.dimension.to_owned();
    let (markers, labels, panel) = with(|s| (s.markers, s.labels, s.panel));
    if !game.native_playing() {
        return;
    }
    if markers {
        draw_world_markers(game, batch, &dim, labels);
    }
    if panel {
        draw_panel(game, batch, corners, &dim);
    }
}

/// The waypoints of one dimension, nearest copy.
fn in_world(dim: &str) -> Vec<Waypoint> {
    with(|s| {
        s.waypoints
            .iter()
            .filter(|w| w.dim == dim)
            .cloned()
            .collect()
    })
}

/// The camera of the last frame, as the locator bar uses it.
#[derive(Clone, Copy)]
struct Camera {
    pos: [f64; 3],
    forward: [f64; 3],
    right: [f64; 3],
    up: [f64; 3],
    /// `1 / tan(fov / 2)`, the vertical FOV in degrees.
    focal: f64,
    half: [f64; 2],
}

fn camera(game: &Game<'_>, width: f32, height: f32) -> Option<Camera> {
    let (pos, yaw, pitch, fov) = game.locator_camera?;
    let (sy, cy) = (f64::from(yaw).to_radians()).sin_cos();
    let (sp, cp) = (f64::from(pitch).to_radians()).sin_cos();
    Some(Camera {
        pos,
        forward: [-sy * cp, -sp, cy * cp],
        right: [-cy, 0.0, -sy],
        up: [-sy * sp, cp, cy * sp],
        focal: 1.0 / (f64::from(fov) / 2.0).to_radians().tan(),
        half: [f64::from(width) / 2.0, f64::from(height) / 2.0],
    })
}

impl Camera {
    fn offset(&self, p: &Waypoint) -> [f64; 3] {
        [p.x - self.pos[0], p.y - self.pos[1], p.z - self.pos[2]]
    }

    fn distance(&self, p: &Waypoint) -> f64 {
        let o = self.offset(p);
        (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt()
    }

    /// Screen pixels, when the point is in front of the camera.
    fn project(&self, p: &Waypoint) -> Option<[f32; 2]> {
        let o = self.offset(p);
        let dot = |a: [f64; 3]| a[0] * o[0] + a[1] * o[1] + a[2] * o[2];
        let depth = dot(self.forward);
        if depth <= 0.05 {
            return None;
        }
        let aspect = self.half[0] / self.half[1];
        let x = self.half[0] + dot(self.right) / depth * self.focal / aspect * self.half[0];
        let y = self.half[1] - dot(self.up) / depth * self.focal * self.half[1];
        Some([x as f32, y as f32])
    }

    /// Where a point away from the screen points: on the border, 16 pixels in.
    /// The direction is the point's bearing on the screen: ahead is up, right
    /// is right, behind is down (a point behind and to the right sits at the
    /// bottom right), raised or lowered by its height above or below the eye.
    fn edge(&self, p: &Waypoint) -> [f32; 2] {
        let o = self.offset(p);
        let dot = |a: [f64; 3]| a[0] * o[0] + a[1] * o[1] + a[2] * o[2];
        let (right, up, forward) = (dot(self.right), dot(self.up), dot(self.forward));
        // Bearing around the player: 0 ahead, positive to the right.
        let bearing = right.atan2(forward);
        let flat = (right * right + forward * forward).sqrt();
        // Elevation: positive above the horizon.
        let elevation = if flat > 1e-9 {
            up.atan2(flat)
        } else {
            0.0
        };
        let (sb, cb) = bearing.sin_cos();
        let (se, ce) = elevation.sin_cos();
        // Screen direction (x right, y down), from the two angles.
        let dx = sb * ce;
        let dy = -cb * ce - se;
        let mx = (self.half[0] - 16.0) / self.half[0];
        let my = (self.half[1] - 16.0) / self.half[1];
        let s = (mx / dx.abs().max(1e-9)).min(my / dy.abs().max(1e-9));
        [
            (self.half[0] + dx * s * self.half[0]) as f32,
            (self.half[1] + dy * s * self.half[1]) as f32,
        ]
    }
}

fn metres(d: f64) -> String {
    if d >= 1000.0 {
        format!("{:.1} km", d / 1000.0)
    } else {
        format!("{} m", d.round() as i64)
    }
}

fn lighter(color: u32) -> u32 {
    let [_, r, g, b] = color.to_be_bytes();
    let up = |v: u8| v.saturating_add((255 - v) / 3);
    (u32::from(up(r)) << 16) | (u32::from(up(g)) << 8) | u32::from(up(b))
}

fn dot(batch: &mut HudBatch, atlas: &HudAtlas, [x, y]: [f32; 2], color: u32, size: f32) {
    let [_, r, g, b] = color.to_be_bytes();
    batch.fill(
        atlas,
        x - size - 1.0,
        y - size - 1.0,
        size * 2.0 + 2.0,
        size * 2.0 + 2.0,
        rgba(0, 0, 0, 150),
    );
    batch.fill(
        atlas,
        x - size,
        y - size,
        size * 2.0,
        size * 2.0,
        rgba(r, g, b, 255),
    );
}

fn draw_world_markers(game: &Game<'_>, batch: &mut HudBatch, dim: &str, labels: bool) {
    let Some(cam) = camera(game, batch.width, batch.height) else {
        return;
    };
    let mut waypoints = in_world(dim);
    waypoints.sort_by(|a, b| cam.distance(b).total_cmp(&cam.distance(a)));
    let (w, h) = (batch.width, batch.height);
    let atlas = &game.gui.atlas;
    for wp in &waypoints {
        let projected = cam.project(wp);
        let (at, on_screen) = match projected {
            Some(at) if at[0] >= 0.0 && at[0] <= w && at[1] >= 0.0 && at[1] <= h => (at, true),
            _ => (cam.edge(wp), false),
        };
        dot(batch, atlas, at, wp.color, if on_screen { 3.0 } else { 2.5 });
        if !labels {
            continue;
        }
        let text = format!("{} · {}", wp.name, metres(cam.distance(wp)));
        let width = game.native_text_width(&text);
        let x = (at[0] - width / 2.0).clamp(2.0, (w - width - 2.0).max(2.0));
        let y = if on_screen {
            at[1] - 13.0
        } else {
            at[1] + 6.0
        };
        let y = y.clamp(2.0, h - 12.0);
        game.native_text(batch, &text, [x, y], lighter(wp.color), true);
    }
}

fn draw_panel(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners, dim: &str) {
    // F3's debug screen draws a chart in the bottom left while it is open.
    if !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    let Some(cam) = camera(game, batch.width, batch.height) else {
        return;
    };
    let mut lines: Vec<(String, f64)> = in_world(dim)
        .iter()
        .map(|wp| (format!("{}  {}", wp.name, metres(cam.distance(wp))), cam.distance(wp)))
        .collect();
    if lines.is_empty() {
        return;
    }
    lines.sort_by(|a, b| a.1.total_cmp(&b.1));
    lines.truncate(6);
    let width = lines
        .iter()
        .map(|(text, _)| game.native_text_width(text))
        .fold(0.0, f32::max)
        + 8.0;
    let height = 10.0 * lines.len() as f32 + 5.0;
    let (x, y) = corners.place(Corner::BottomLeft, width, height);
    game.native_fill(batch, [x, y, width, height], 0x7800_0000);
    for (i, (text, _)) in lines.iter().enumerate() {
        game.native_text(
            batch,
            text,
            [x + 4.0, y + 3.0 + 10.0 * i as f32],
            0xFF_FFFF,
            true,
        );
    }
}