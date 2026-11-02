#[cfg(test)]
mod tests {
    include!("mixins/waymap/commands.rs");

    #[test]
    fn registers_waypoint_command() {
        let mut commands = Vec::new();
        waymap_commands(&mut commands);
        assert_eq!(commands, ["wp"]);
    }
}
