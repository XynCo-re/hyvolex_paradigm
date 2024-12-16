use bevy::prelude::*;
use hyvolex_paradigm::run;

fn main() {
    // Configure logging settings
    std::env::set_var("RUST_LOG", "wgpu=error,hyvolex_paradigm=debug,bevy_render=warn");
    
    // Run the application with error handling
    if let Err(e) = run() {
        error!("Application error: {e}");
        std::process::exit(1);
    }
}
