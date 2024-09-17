// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use lumina::AppPlugin;

fn main() -> AppExit {
    println!("Entered main.");
    App::new().add_plugins(AppPlugin).run()
}
