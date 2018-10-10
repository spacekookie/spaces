use i3ipc::I3Connection;

/// Get a list of all workspaces as a normal string vector
pub fn get_workspaces(conn: &mut I3Connection) -> Vec<String> {
    conn.get_workspaces()
        .unwrap()
        .workspaces
        .into_iter()
        .map(|ws| ws.name.clone())
        .collect()
}

pub fn set_workspace(conn: &mut I3Connection, name: &String) {
    conn.run_command("foo").unwrap();
}
