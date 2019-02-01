extern crate i3ipc;

mod layout;
mod workspaces;

use self::layout::{rhombus::*, LayoutGenerator};
use i3ipc::*;

fn main() {
    // let mut i3 = I3Connection::connect().unwrap();
    // let wss = i3.get_workspaces().unwrap();

    for i in 1..60 {
        let (x, _) = Rhombus::rel_position(i);
        // let generation = i as u64 / gen as u64;
        // println!("For {} x: {}", i, x);
    }
}
