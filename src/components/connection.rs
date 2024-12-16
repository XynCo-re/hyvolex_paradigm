use bevy::prelude::*;

#[derive(Component)]
pub struct Connection {
    pub start: Entity,
    pub end: Entity,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    // Cardinal and Ordinal
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    
    // Vertical
    Up,
    Down,
    
    // Spatiotemporal
    Forward,  // Temporal progression
    Backward, // Temporal regression
    Inward,   // Gravitational approach
    Outward,  // Gravitational retreat
    
    // Reference
    Center,   // Node's intrinsic center point
} 