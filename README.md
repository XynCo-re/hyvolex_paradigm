# HyvoLex Paradigm

HyvoLex Paradigm is a sophisticated physics simulation engine implemented in Rust using the Bevy game engine. It specializes in modeling electromagnetic interactions within a three-dimensional helical structure, incorporating advanced rendering techniques and real-time particle dynamics.

## Core Features

### Physics Simulation
- Magnetic field interaction modeling with polarity-based attractions and repulsions
- Dynamic node generation within a triple-helix configuration
- Spatiotemporal phase calculations for field strength modulation
- Inverse square law implementation for distance-based field interactions

### Visualization Components
- Real-time 3D rendering with custom shaders and materials
- Adaptive mesh generation for various node types (Alpha, Beta, Gamma)
- Interactive camera system with orbital controls
- Dynamic particle effects using the Hanabi particle system
- Outline rendering for enhanced visual feedback

### Technical Architecture
- Entity-Component-System (ECS) architecture via Bevy
- Multi-threaded physics computations
- Comprehensive error handling system with circuit breaker pattern
- Resource management with automatic cleanup
- Platform-specific optimizations for Windows and Linux

## Installation

### Prerequisites
- Rust toolchain (2021 edition or later)
- GPU with Vulkan/Metal/DirectX 11+ support
- CMake (for building dependencies)

# Clone the repository
git clone https://github.com/yourusername/hyvolex_paradigm.git
cd hyvolex_paradigm

# Build the project
cargo build --release

### Development Dependencies
bevy = "0.15.0"
bevy_hanabi = "0.14.0"
bevy_tweening = "0.12.0"
bevy_mod_outline = "0.9.0"
bevy_panorbit_camera = "0.21.1"

## Usage

### Basic Execution
cargo run --release

### Camera Controls
- **Middle Mouse Button**: Orbit camera
- **Shift + Middle Mouse Button**: Pan camera
- **Z/X Keys**: Zoom in/out
- **Arrow Keys**: 
 - Normal: Rotate camera
 - With Ctrl: Jump 45 degrees
 - With Shift: Pan view

### Simulation Controls
- **Space**: Pause/Resume simulation
- **Up/Down Arrows**: Adjust simulation speed
- **ESC**: Exit application

## Architecture

### Component Structure
Node
├── ShapeType (Alpha/Beta/Gamma)
├── MagneticField
│   ├── Strength
│   ├── Polarity
│   └── Orientation
└── TemporalPhase

### System Pipeline
1. Input Processing (HyvoGridSet::Input)
2. Physics Simulation (HyvoGridSet::Simulation)
3. Visual Rendering (HyvoGridSet::Rendering)

## Error Handling

The project implements a sophisticated error handling system with:
- Circuit breaker pattern for failure isolation
- Platform-specific error recovery strategies
- Comprehensive error type hierarchy
- Fallback mechanisms for graceful degradation

## Configuration

### Material Properties
MaterialHandles {
   node_material: Handle<StandardMaterial>,
   connection_material: Handle<StandardMaterial>,
   highlight_material: Handle<StandardMaterial>,
   noise_texture: Handle<Image>,
   ramp_texture: Handle<Image>,
}

### Performance Tuning
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
opt-level = 3
lto = "thin"

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Submit a Pull Request

Ensure all commits follow the conventional commits specification and include appropriate test coverage.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Technical Notes

### Performance Considerations
- Mesh generation is optimized for vertex buffer reuse
- Magnetic field calculations use spatial partitioning
- Material system implements automatic texture management
- Plugin system allows selective feature enablement

### Known Limitations
- Particle system performance may degrade with >10000 concurrent particles
- Magnetic field calculations are approximated for distant interactions
- Camera controls may exhibit quaternion gimbal lock at extreme angles

## Acknowledgments

- Bevy Engine Community
- Rust Graphics Working Group
- Physics Simulation Reference Papers