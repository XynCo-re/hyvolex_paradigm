// System modules
pub mod camera;
pub mod generation;
pub mod intersections;
pub mod magnetic;
pub mod mesh_generator;
pub mod node_visuals;
pub mod particles;
pub mod rendering;
pub mod setup;

// Re-exports for commonly used functionality
pub use self::{
    camera::{camera_controls, camera_setup},
    generation::generate_helix,
    intersections::{check_intersections, setup_intersection_effects},
    magnetic::{setup_magnetic_effects, update_magnetic_fields},
    mesh_generator::create_tridecahedron,
    node_visuals::{setup_node_effects, update_node_visuals},
    particles::{update_particles, setup_particle_system},
    rendering::update_rendering_visuals,
    setup::{setup_camera, setup_materials, setup_scene, setup_window_border, animate_window_border},
}; 