// Waymap: client-side waypoints, in the world and on the minimap.
//
// A waypoint is a named point (`name`, `x`, `y`, `z`) in one dimension, with a
// colour. Waypoints live in the mod's own folder
// (`horizon-mods/.horizon-mod-data/waymap/waypoints.txt`), so they survive a
// restart, a world change and an update.
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
use std::path::{Path, PathBuf};

/// The mod's id: its folder, its settings files and its keys.
pub(super) const MOD_ID: &str = "waymap";
const NAME: &str = "Waymap";
const VERSION: &str = "0.1.0";
const DESCRIPTION: &str =
    "Waypoints in the world and on the minimap: /wp add, /wp list, /wp del.";
/// The waypoints file, one line per waypoint, tab separated.
const FILE: &str = "waypoints.txt";
/// `Player.getEyeHeight` while standing: a waypoint is kept at eye level, so a
/// marker sits on the crosshair when the player stands where it was set.
const EYE: f64 = 1.62;
/// Waypoints per world; more would only clutter the map.
const MAX_WAYPOINTS: usize = 512;
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

/// The minimap's view, its dots and their drawing live in the client library
/// (`extras_hud::waymap_minimap`), next to `draw_minimap`; the game pushes
/// this frame's waypoints there.

/// The minimap's view type from the render crate.
type MiniView = horizon_client::minimap::View;

struct State {
    /// The waypoints were read (or there was nothing to read).
    loaded: bool,
    dir: Option<PathBuf>,
    waypoints: Vec<Waypoint>,
    markers: bool,
    labels: bool,
    panel: bool,
    edit_panel: bool,
    /// This frame's minimap view, from `update_minimap`.
    mini: Option<MiniView>,
    /// The dimension of the last frame, for the minimap dots (drawn from a
    /// hook that has no `Game`).
    dim: String,
}
impl Default for State {
    fn default() -> Self {
        Self {
            loaded: false,
            dir: None,
            waypoints: Vec::new(),
            markers: true,
            labels: true,
            panel: true,
            edit_panel: false,
            mini: None,
            dim: String::new(),
        }
    }
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with_borrow_mut(f)
}

/// Reads the waypoints once, from the folder the game gives this mod.
fn ensure_loaded(game: &Game<'_>) {
    if with(|s| s.loaded) {
        return;
    }
    let dir = game.native_data_dir(MOD_ID);
    let waypoints = load(&dir);
    let on = |key: &str| game.native_setting(MOD_ID, key).as_deref() != Some("off");
    let (markers, labels, panel) = (on("markers"), on("labels"), on("panel"));
    with(|s| {
        if s.loaded {
            return;
        }
        s.loaded = true;
        s.dir = Some(dir);
        s.waypoints = waypoints;
        s.markers = markers;
        s.labels = labels;
        s.panel = panel;
    });
}

fn save(dir: &Path, waypoints: &[Waypoint]) {
    let mut text = String::new();
    for w in waypoints {
        let _ = writeln!(
            text,
            "{}\t{:.3}\t{:.3}\t{:.3}\t{:06x}\t{}",
            w.dim, w.x, w.y, w.z, w.color, w.name
        );
    }
    let _ = std::fs::create_dir_all(dir);
    let _ = std::fs::write(dir.join(FILE), text);
}

fn load(dir: &Path) -> Vec<Waypoint> {
    let Ok(text) = std::fs::read_to_string(dir.join(FILE)) else {
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

/// Called from `native_frame` (once a frame, in a world or not): hands this
/// frame's minimap dots to the library overlay.
pub(super) fn frame(game: &mut Game<'_>) {
    if !game.native_window() {
        return;
    }
    ensure_loaded(game);
    
    // When chat opens, don't hide markers - show a note that they're available.
    // The chat screen takes over; markers are behind it anyway.
    if game.screen == crate::game::Screen::Chat {
        // Chat open: markers invisible because chat overlay covers them
        // We could show a brief message or just rely on existing behavior
        return; // Skip marker drawing while chat is open (they'd be hidden anyway)
    }
    
    with(|s| {
        s.mini = None;
        if let Some(p) = game.native_player() {
            s.dim = p.dimension.to_owned();
        }
    });
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
            [
                "edit".into(),
                "Waypoint edit panel".into(),
                "key.keyboard.m".into(),
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
    buttons.push(format!("Edit panel (M): {}", on_off(edit_panel)));
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
        3 => {
            s.edit_panel = !s.edit_panel;
            message = format!("Edit panel: {}", on_off(s.edit_panel));
        }
        _ => return,
    });
    let (_key, on) = match button {
        0 => ("markers", with(|s| s.markers)),
        1 => ("labels", with(|s| s.labels)),
        2 => ("panel", with(|s| s.panel)),
        3 => ("management", with(|_| true)),
        _ => return,
    };
    if button < 4 {
        let key = match button {
            0 => "markers",
            1 => "labels",
            2 => "panel",
            3 => "edit_panel",
            _ => "",
        };
        if !key.is_empty() {
            game.native_set_setting(MOD_ID, key, on_off(on));
        }
    }
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
        "del" | "delete" | "remove" => remove(args),
        "list" => list(),
        "rename" => rename(args),
        "color" | "colour" => recolor(args),
        "clear" => clear(),
        "set" | "edit" => edit(args),
        other => format!("Unbekannt: /wp {other}. /wp help zeigt die Befehle."),
    };
    for line in message.lines() {
        game.native_chat(line);
    }
}

fn help() -> String {
    format!(
        "{NAME} {VERSION}: /wp add [Name] · /wp del <Name> · /wp list\n\
         /wp rename <Alt> <Neu> · /wp color <Name> <Farbe> · /wp clear\n\
         /wp edit <Name> (öffnet das Management-Menü)"
    )
}

/// Adds a waypoint at the player's eyes, or reports why not.
fn add_here(game: &mut Game<'_>, name: Option<String>) -> String {
    let Some(p) = game.native_player() else {
        return "Kein Spieler in einer Welt.".to_owned();
    };
    let mut message = String::new();
    with(|s| {
        if s.waypoints.len() >= MAX_WAYPOINTS {
            message = format!("Mehr als {MAX_WAYPOINTS} Wegpunkte gehen nicht.");
            return;
        }
        let name = name.unwrap_or_else(|| auto_name(&s.waypoints));
        if let Some(old) = s.waypoints.iter().find(|w| w.name.eq_ignore_ascii_case(&name)) {
            message = format!(
                "'{}' gibt es schon ({}, {}, {}). /wp del {} zuerst.",
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
        if let Some(dir) = &s.dir {
            save(dir, &s.waypoints);
        }
        message = format!(
            "Wegpunkt '{name}' gesetzt: {}, {}, {} ({}).",
            p.x.floor(),
            (p.y + EYE).floor(),
            p.z.floor(),
            color_name(color)
        );
    });
    message
}

fn remove(name: &str) -> String {
    // Handle quoted names and multi-word names
    let name = name.trim();
    if name.is_empty() {
        return "Welchen Wegpunkt? /wp del <Name>".to_owned();
    }
    // If quoted, extract content
    let name_trim = name.trim();
    let name_clean = if name_trim.starts_with('"') && name_trim.ends_with('"') && name_trim.len() > 1 {
        &name_trim[1..name_trim.len()-1]
    } else {
        name
    };
    let mut message = String::new();
    with(|s| {
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(name_clean)) else {
            message = format!("Kein Wegpunkt '{name_clean}'. /wp list zeigt alle.");
            return;
        };
        let w = s.waypoints.remove(i);
        if let Some(dir) = &s.dir {
            save(dir, &s.waypoints);
        }
        message = format!("Wegpunkt '{}' gelöscht.", w.name);
    });
    message
}

fn list() -> String {
    with(|s| {
        if s.waypoints.is_empty() {
            return format!("{NAME}: keine Wegpunkte. /wp add [Name] setzt einen.");
        }
        let mut text = format!("{NAME}: {} Wegpunkte", s.waypoints.len());
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
            let _ = write!(text, "\n… und {} mehr", s.waypoints.len() - 20);
        }
        text
    })
}

fn rename(args: &str) -> String {
    let args_trim = args.trim();
    if args_trim.is_empty() {
        return "Nutzung: /wp rename <Alt> <Neu>".to_owned();
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
            return "Nutzung: /wp rename <Alt> <Neu>".to_owned();
        }
        return do_rename(old, new);
    }
    if let Some(pos) = args_trim.find(' ') {
        let old = args_trim[..pos].trim();
        let new = args_trim[pos+1..].trim();
        if !old.is_empty() && !new.is_empty() {
            return do_rename(old, new);
        }
    }
    "Nutzung: /wp rename <Alt> <Neu>".to_owned()
}

fn do_rename(old: &str, new: &str) -> String {
    let new = clean_name(new);
    let mut message = String::new();
    with(|s| {
        if s.waypoints.iter().any(|w| w.name.eq_ignore_ascii_case(&new)) {
            message = format!("'{new}' gibt es schon.");
            return;
        }
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(old)) else {
            message = format!("Kein Wegpunkt '{old}'.");
            return;
        };
        s.waypoints[i].name = new.clone();
        if let Some(dir) = &s.dir {
            save(dir, &s.waypoints);
        }
        message = format!("'{old}' heißt jetzt '{new}'.");
    });
    message
}

fn recolor(args: &str) -> String {
    // Handle quoted names and multi-word names
    let args_trim = args.trim();
    if args_trim.is_empty() {
        return colors_help();
    }
    
    // Find the last word as color (color names are single words: red, green, etc.)
    // Or find quoted sections
    let parts: Vec<&str> = args_trim.split(' ').collect();
    if parts.len() < 2 {
        return colors_help();
    }
    
    // Last word is color, everything before is name
    let color_str = parts[parts.len()-1].trim();
    let name = parts[..parts.len()-1].join(" ").trim().to_string();
    
    if name.is_empty() || color_str.is_empty() {
        return colors_help();
    }
    
    let Some(color) = parse_color(color_str) else {
        return format!("Unbekannte Farbe '{}'. {}", color_str, colors_help());
    };
    
    let mut message = String::new();
    with(|s| {
        let Some(w) = s.waypoints.iter_mut().find(|w| w.name.eq_ignore_ascii_case(&name)) else {
            message = format!("Kein Wegpunkt '{}'.", name);
            return;
        };
        w.color = color;
        let wp_name = w.name.clone();
        if let Some(dir) = &s.dir {
            save(dir, &s.waypoints);
        }
        message = format!("'{wp_name}' ist jetzt {}.", color_name(color));
    });
    message
}

fn colors_help() -> String {
    let names: Vec<&str> = COLORS.iter().map(|(name, _)| *name).collect();
    format!("Farben: {} oder #rrggbb", names.join(", "))
}

fn clear() -> String {
    let mut message = String::new();
    with(|s| {
        let n = s.waypoints.len();
        s.waypoints.clear();
        if let Some(dir) = &s.dir {
            save(dir, &s.waypoints);
        }
        message = if n == 0 {
            "Es gab keine Wegpunkte.".to_owned()
        } else {
            format!("{n} Wegpunkte gelöscht.")
        };
    });
    message
}

fn edit(args: &str) -> String {
    let mut message = String::new();
    with(|s| {
        let Some(i) = s.waypoints.iter().position(|w| w.name.eq_ignore_ascii_case(args.trim())) else {
            message = format!("Kein Wegpunkt '{}'. /wp list zeigt alle.", args.trim());
            return;
        };
        let w = &mut s.waypoints[i];
        message = format!(
            "Waypoint '{name}' bearbeiten: /wp edit {name} [new_name] [color] · /wp del {name}",
            name = w.name
        );
    });
    message
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
    } else if key == game.native_mod_key(MOD_ID, "edit", "key.keyboard.m") {
        let on = with(|s| {
            s.edit_panel = !s.edit_panel;
            s.edit_panel
        });
        game.native_set_setting(MOD_ID, "edit_panel", on_off(on));
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
    if with(|s| s.edit_panel) {
        draw_edit_panel(game, batch, corners, &dim);
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
    // atlas referenced via dot() params; no standalone unused variable
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



/// The edit panel: a full list of all waypoints with delete/edit actions.
fn draw_edit_panel(game: &Game<'_>, batch: &mut HudBatch, _corners: &mut Corners, dim: &str) {
    if !game.native_playing() {
        return;
    }
    let waypoints = in_world(dim);
    if waypoints.is_empty() {
        return;
    }
    let (w, h) = (batch.width, batch.height);
    // Large overlay panel in center-top area
    let pw = (w * 0.6).min(600.0);
    let ph = (h * 0.7).min(500.0);
    let px = (w - pw) / 2.0;
    let py = (h - ph) / 2.0;
    
    // Background
    game.native_fill(batch, [px, py, pw, ph], 0x7800_0000);
    
    // Title
    game.native_text(batch, "Waymap - Wegpunkte verwalten", [px + 10.0, py + 10.0], 0xFF_EEAA, true);
    
    // Instruction
    game.native_text(batch, "Rechtsklick = Löschen | Links = Info (noch in Entwicklung)", [px + 10.0, py + 30.0], 0x88_8888, true);
    
    // List waypoints
    let max_items = waypoints.len().min(15);
    for (i, wp) in waypoints.iter().enumerate().take(max_items) {
        let y = py + 55.0 + i as f32 * 22.0;
        let text = format!("{}. {} | {}, {}, {} | {}", i + 1, wp.name, wp.x.round() as i32, wp.y.round() as i32, wp.z.round() as i32, color_name(wp.color));
        game.native_text(batch, &text, [px + 10.0, y], 0xFF_FFFF, true);
    }
}
