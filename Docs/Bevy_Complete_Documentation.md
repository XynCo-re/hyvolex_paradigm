# Bevy Engine Complete Documentation
Version: 0.15.0

## Table of Contents
1. [Glossary](#glossary)
2. [Core Concepts](#core-concepts)
   - [Components](#components)
   - [Resources](#resources)
   - [Systems](#systems)
   - [Assets](#assets)
3. [Camera Systems](#camera-systems)
   - [Core Camera](#core-camera)
   - [PanOrbit Camera](#panorbit-camera)
   - [Editor Camera](#editor-camera)
4. [Input Management](#input-management)
   - [Core Input](#core-input)
   - [Pan Camera Input](#pan-camera-input)
   - [Leafwing Input Manager](#leafwing-input-manager)
5. [Animation and Effects](#animation-and-effects)
   - [Tweening System](#tweening-system)
   - [Particle System (Hanabi)](#particle-system)
6. [Best Practices](#best-practices)
   - [Component Patterns](#component-patterns)
   - [Resource Management](#resource-management)
   - [System Organization](#system-organization)
7. [Advanced ECS Architecture](#advanced-ecs-architecture)
   - [Entity Component System Deep Dive](#entity-component-system-deep-dive)
   - [Advanced Query Patterns](#advanced-query-patterns)
   - [Component Storage Patterns](#component-storage-patterns)
8. [State Management](#state-management)
   - [Game States](#game-states)
   - [Resource States](#resource-states)
   - [Local State](#local-state)
   - [Complex State Patterns](#complex-state-patterns)
9. [Event System Patterns](#event-system-patterns)

## Glossary

- **Asset**: Content loaded from disk or generated at runtime (textures, models, sounds, etc.)
- **Component**: Data attached to entities in the ECS system
- **Entity**: A unique ID that components can be attached to
- **Resource**: Global singleton state accessible across systems
- **System**: Function that operates on components and resources
- **Bundle**: Collection of components that can be added to an entity together
- **Query**: Method to access entities and their components in systems
- **Plugin**: Collection of systems, resources, and other functionality
- **State**: Enum representing different states of the application
- **Handle**: Reference-counted pointer to an asset
- **Transform**: Component representing position, rotation, and scale
- **Lens**: Interface for tweening between two values
- **Spawner**: Configuration for particle emission
- **InputMap**: Configuration mapping inputs to actions

## Core Concepts

### Components
```rust
// Modern component pattern
#[derive(Component)]
struct Player {
    health: f32,
    speed: f32,
}

// Bundle pattern
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: Transform,
    sprite: Sprite,
}
```

Components are the fundamental building blocks of entities in Bevy. They should be:
- Small and focused
- Implement Default when appropriate
- Use derive macros for common traits
- Follow Rust naming conventions

### Resources
```rust
#[derive(Resource)]
struct GameState {
    score: u32,
    is_paused: bool,
}

// Resource initialization
app.insert_resource(GameState {
    score: 0,
    is_paused: false,
});
```

Resources are global singleton states that can be accessed by systems. Best practices:
- Use for global state
- Keep data organized
- Consider using Events for updates
- Implement Default when possible

### Systems
```rust
// Basic system
fn update_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query {
        transform.translation.x += player.speed * time.delta_seconds();
    }
}

// System sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    Input,
    Movement,
    Collision,
}
```

Systems are functions that operate on components and resources. Key concepts:
- Run in parallel when possible
- Can be ordered using SystemSets
- Can have run conditions
- Should be focused and modular

### Assets
```rust
// Asset loading
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/player.png");
    commands.spawn(SpriteBundle {
        texture,
        ..default()
    });
}
```

Assets are external resources loaded at runtime. Important concepts:
- Loaded asynchronously
- Reference counted via Handles
- Can be hot-reloaded
- Support custom asset types

## Camera Systems

### Core Camera
```rust
// Modern camera setup (3D)
commands.spawn((
    Camera3d,
    Camera,
    Transform::from_xyz(0.0, 5.0, 15.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
));

// 2D camera setup
commands.spawn((
    Camera2dBundle::default(),
));
```

Core camera features:
- Support for 2D and 3D
- Orthographic and perspective projection
- Clear color configuration
- MSAA support
- HDR and tonemapping

### PanOrbit Camera
```rust
// PanOrbit camera setup
commands.spawn((
    Camera3d,
    Camera,
    PanOrbitCamera {
        focus: Vec3::ZERO,
        radius: 5.0,
        alpha: std::f32::consts::FRAC_PI_4,
        beta: std::f32::consts::FRAC_PI_4,
        ..default()
    },
));
```

PanOrbit camera features:
- Orbit around focus point
- Pan view control
- Zoom in/out
- Touch input support
- Configurable buttons
- Smooth transitions

### Editor Camera
```rust
// Editor camera setup
commands.spawn((
    Camera3d,
    Camera,
    EditorCamera::default(),
));
```

Editor camera features:
- Production-ready for 3D editors
- Responsive controls
- First-order input
- Pixel-perfect panning
- Intuitive zoom
- Predictable rotation

## Input Management

### Core Input
```rust
// Basic input handling
fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 1.0;
        }
    }
}
```

Core input features:
- Keyboard support
- Mouse input
- Touch input
- Gamepad support
- Input state tracking

### Pan Camera Input
```rust
// Pan camera configuration
commands.spawn((
    Camera2d,
    PanCam {
        grab_buttons: vec![MouseButton::Left],
        enabled: true,
        zoom_to_cursor: true,
        min_scale: 1.0,
        max_scale: 40.0,
        ..default()
    },
));
```

Pan camera input features:
- Click and drag movement
- Scroll wheel zoom
- Keyboard movement
- Touch support
- Configurable controls

### Leafwing Input Manager
```rust
// Action definition
#[derive(Actionlike, Clone, Copy, Debug)]
enum Action {
    Move,
    Jump,
    Attack,
}

// Input map setup
let input_map = InputMap::new([
    (Action::Jump, KeyCode::Space),
    (Action::Attack, MouseButton::Left),
]);

// Player setup
commands.spawn((
    PlayerBundle::default(),
    InputManagerBundle::with_map(input_map),
));
```

Leafwing input manager features:
- Input abstraction
- Multiple input sources
- Action state tracking
- Local multiplayer support
- Chord support
- Input disambiguation

## Animation and Effects

### Tweening System
```rust
// Basic tween setup
let tween = Tween::new(
    EaseFunction::QuadraticInOut,
    Duration::from_secs(1),
    TransformPositionLens {
        start: Vec3::ZERO,
        end: Vec3::new(0.0, 5.0, 0.0),
    },
);

// Spawn entity with tween
commands.spawn((
    SpriteBundle::default(),
    Animator::new(tween),
));
```

Tweening features:
- Multiple easing functions
- Chainable animations
- Sequence and parallel animations
- Custom lens support
- Event callbacks
- Relative and absolute animations

### Particle System (Hanabi)
```rust
// Basic particle effect
let mut color_gradient = Gradient::new();
color_gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
color_gradient.add_key(1.0, Vec4::new(0.0, 0.0, 1.0, 0.0));

let effect = EffectAsset {
    name: "Simple".into(),
    capacity: 32768,
    spawner: Spawner::rate(30.0.into()),
    ..default()
}
.init(vec![
    Lifetime::new(1.0),
    Position::new_2d(),
    Velocity::new_2d(10.0),
])
.render(vec![
    ColorOverLifetime::new(color_gradient),
    SizeOverLifetime::new(ExprWriter::linear(5.0, 0.0)),
]);

// Spawn the effect
commands.spawn((
    ParticleEffect::new(effects.add(effect)),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

Particle system features:
- Full GPU-based simulation
- Visual effect graph system
- Property modifiers over lifetime
- Spawn rate control
- Custom modules support
- 2D and 3D support

## Best Practices

### Component Patterns
- Keep components small and focused
- Use marker components for tags
- Implement Default when appropriate
- Use derive macros for common traits
- Group related components in bundles
- Provide sensible defaults
- Include all necessary components for functionality
- Use modern bundle patterns

### Resource Management
- Use resources for global state
- Keep resource data organized
- Use events for updates
- Implement Default when possible
- Consider using Events for updates
- Handle resource dependencies properly
- Use resource scope for temporary access
- Consider using Local resources for system-specific state

### System Organization
- Keep systems focused and modular
- Use system sets for organization
- Implement proper system ordering
- Use run conditions appropriately
- Consider parallel execution
- Handle state transitions properly
- Use exclusive systems when needed
- Implement proper error handling

## Advanced ECS Architecture

### Entity Component System Deep Dive
```rust
use bevy::prelude::*;

// Entity: Just an ID, created like this
#[derive(Component)]
struct Player {
    health: f32,
    speed: f32,
}

// Components: Pure data
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

// Systems: Logic that operates on components
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Position)>,
) {
    for (velocity, mut position) in &mut query {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

// Complex queries with filters
fn heal_damaged_players(
    mut query: Query<&mut Player, (With<Player>, Without<Invulnerable>, Changed<Health>)>,
) {
    for mut player in &mut query {
        if player.health < 100.0 {
            player.health += 1.0;
        }
    }
}
```

### Advanced Query Patterns
```rust
// Multiple disjoint queries
fn interaction_system(
    players: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    for (player_entity, player_transform) in &players {
        for (enemy_entity, enemy_transform) in &enemies {
            if player_transform.translation.distance(enemy_transform.translation) < 5.0 {
                // Handle collision
            }
        }
    }
}

// Parent-child relationships
fn follow_parent(
    mut children: Query<&mut Transform, With<FollowParent>>,
    parents: Query<&Transform, (Without<FollowParent>, Changed<Transform>)>,
) {
    for mut child_transform in &mut children {
        if let Ok(parent_transform) = parents.get(child_transform.parent) {
            child_transform.translation = parent_transform.translation;
        }
    }
}
```

### Component Storage Patterns
```rust
// Sparse components for rare data
#[derive(Component)]
struct RareComponent(f32);

// Dense components for common data
#[derive(Component, Default)]
struct CommonComponent(f32);

// Table storage for grouped data
#[derive(Bundle)]
struct PhysicsBundle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}
```

## State Management

### Game States
```rust
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}

// State-dependent systems
fn update_game(
    mut query: Query<&mut Transform>,
) -> impl IntoConditionalSystem {
    IntoConditionalSystem::new(move |state: Res<State<GameState>>| {
        state.get() == &GameState::InGame
    })
}

// State transitions
fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => {}
        }
    }
}
```

### Resource States
```rust
#[derive(Resource)]
struct GameProgress {
    level: u32,
    score: u32,
    high_scores: HashMap<u32, u32>,
}

impl GameProgress {
    fn save(&self) -> Result<(), std::io::Error> {
        // Implement save logic
        Ok(())
    }

    fn load() -> Result<Self, std::io::Error> {
        // Implement load logic
        Ok(Self::default())
    }
}

// State initialization system
fn setup_game_state(
    mut commands: Commands,
) {
    if let Ok(progress) = GameProgress::load() {
        commands.insert_resource(progress);
    } else {
        commands.insert_resource(GameProgress::default());
    }
}
```

### Local State
```rust
// System-local state
fn spawn_timer(
    mut local: Local<SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    local.0.tick(time.delta());
    
    if local.0.finished() {
        commands.spawn(EnemyBundle::default());
        local.0.reset();
    }
}

// Component-local state
#[derive(Component)]
struct AnimationState {
    current_frame: usize,
    timer: Timer,
    frames: Vec<Handle<Image>>,
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut Handle<Image>)>,
) {
    for (mut state, mut image) in &mut query {
        state.timer.tick(time.delta());
        if state.timer.finished() {
            state.current_frame = (state.current_frame + 1) % state.frames.len();
            *image = state.frames[state.current_frame].clone();
        }
    }
}
```

### Complex State Patterns
```rust
// State machine component
#[derive(Component)]
enum EnemyState {
    Patrol { waypoint_index: usize },
    Chase { target: Entity },
    Attack { cooldown: Timer },
}

// State transition system
fn enemy_behavior(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut EnemyState, &Transform)>,
    players: Query<(Entity, &Transform), With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut state, transform) in &mut enemies {
        match &mut *state {
            EnemyState::Patrol { waypoint_index } => {
                // Check for players in range
                for (player_entity, player_transform) in &players {
                    if transform.translation.distance(player_transform.translation) < 10.0 {
                        *state = EnemyState::Chase { target: player_entity };
                        break;
                    }
                }
            }
            EnemyState::Chase { target } => {
                if let Ok((_, player_transform)) = players.get(*target) {
                    if transform.translation.distance(player_transform.translation) < 2.0 {
                        *state = EnemyState::Attack { 
                            cooldown: Timer::from_seconds(1.0, TimerMode::Once) 
                        };
                    }
                }
            }
            EnemyState::Attack { cooldown } => {
                cooldown.tick(time.delta());
                if cooldown.finished() {
                    *state = EnemyState::Patrol { waypoint_index: 0 };
                }
            }
        }
    }
}
```

## Event System Patterns

### Basic Events
```rust
// Define custom event
#[derive(Event)]
struct CollisionEvent {
    entity_a: Entity,
    entity_b: Entity,
    point: Vec3,
}

// Add event to app
app.add_event::<CollisionEvent>();

// Send events
fn collision_detection(
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Transform)>,
) {
    for [(entity_a, transform_a), (entity_b, transform_b)] in query.iter_combinations() {
        if check_collision(transform_a, transform_b) {
            collision_events.send(CollisionEvent {
                entity_a,
                entity_b,
                point: calculate_collision_point(transform_a, transform_b),
            });
        }
    }
}

// Handle events
fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for collision in collision_events.read() {
        // Handle collision
        println!("Collision at: {:?}", collision.point);
    }
}
```

### Event Patterns
```rust
// One-shot events
#[derive(Event)]
struct GameOverEvent;

// Events with data
#[derive(Event)]
struct DamageEvent {
    target: Entity,
    amount: f32,
    damage_type: DamageType,
}

// State change events
#[derive(Event)]
struct StateChangeEvent {
    from: GameState,
    to: GameState,
}

// Event with custom ordering
#[derive(Event)]
#[event_order(after = DamageEvent, before = DeathEvent)]
struct HealEvent {
    target: Entity,
    amount: f32,
}
```

## Testing and Debugging

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_system() {
        // Setup app with minimal plugins
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, health_system);
            
        // Spawn test entity
        let entity = app.world.spawn((
            Health { value: 100.0 },
            MaxHealth { value: 100.0 },
        )).id();
        
        // Run systems
        app.update();
        
        // Assert results
        let health = app.world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.value, 100.0);
    }
}
```

### Integration Testing
```rust
#[test]
fn test_game_flow() {
    let mut app = App::new();
    
    // Setup test environment
    app.add_plugins((
        MinimalPlugins,
        TestPlugin,
    ));
    
    // Test state transitions
    app.update();
    assert_eq!(app.world.resource::<State<GameState>>().get(), &GameState::Loading);
    
    // Simulate loading completion
    app.world.resource_mut::<AssetsLoading>().mark_complete();
    app.update();
    assert_eq!(app.world.resource::<State<GameState>>().get(), &GameState::MainMenu);
}
```

### Debug Tools
```rust
// Debug logger system
#[cfg(debug_assertions)]
fn debug_logger(
    query: Query<(Entity, &Transform), Changed<Transform>>,
) {
    for (entity, transform) in &query {
        info!("Entity {:?} moved to {:?}", entity, transform.translation);
    }
}

// Visual debugging
fn debug_draw(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Collider>>,
) {
    for transform in &query {
        gizmos.circle(
            transform.translation,
            Vec3::Y,
            5.0,
            Color::RED,
        );
    }
}
```

## Performance Optimization

### System Optimization
```rust
// Efficient queries
fn optimized_system(
    // Use exact component types needed
    query: Query<&Transform, With<Moving>, Without<Static>>,
) {
    // More efficient than querying all entities
}

// Parallel iteration
fn parallel_system(
    query: Query<&mut Transform>,
) {
    query.par_iter_mut().for_each(|mut transform| {
        // Safe parallel processing
    });
}
```

### Memory Optimization
```rust
// Component storage optimization
#[derive(Component)]
#[component(storage = "SparseSet")]
struct RarelyUsedComponent;

// Resource optimization
#[derive(Resource)]
struct GameCache {
    // Use appropriate data structures
    spatial_hash: HashMap<IVec2, Vec<Entity>>,
    object_pool: Vec<Entity>,
}

// Asset handling
fn asset_management(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loaded_assets: ResMut<LoadedAssets>,
) {
    // Load assets only when needed
    if let Some(texture) = loaded_assets.get("texture") {
        // Use cached asset
    } else {
        // Load and cache new asset
        let handle = asset_server.load("texture.png");
        loaded_assets.insert("texture", handle);
    }
}
```

### Profiling
```rust
// System timing
#[cfg(feature = "profiling")]
fn profile_system(
    time: Res<Time>,
    mut metrics: ResMut<Metrics>,
) {
    let start = std::time::Instant::now();
    
    // System logic here
    
    metrics.record_timing("system_name", start.elapsed());
}

// Memory tracking
#[cfg(feature = "profiling")]
fn track_memory(
    world: &World,
    mut metrics: ResMut<Metrics>,
) {
    metrics.record_component_count::<Transform>(world.components().len());
}
```

## Plugin Development

### Custom Plugin
```rust
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .init_resource::<GameState>()
            // Add events
            .add_event::<GameEvent>()
            // Add systems in sets
            .configure_sets((
                GameSet::Input,
                GameSet::Update,
                GameSet::Render,
            ).chain())
            // Add systems
            .add_systems(Update, (
                input_system.in_set(GameSet::Input),
                update_system.in_set(GameSet::Update),
                render_system.in_set(GameSet::Render),
            ));
    }
}

// Plugin configuration
#[derive(Resource)]
pub struct GamePluginConfig {
    pub debug_mode: bool,
    pub max_entities: usize,
}

// Configurable plugin
pub struct ConfigurableGamePlugin {
    config: GamePluginConfig,
}

impl Plugin for ConfigurableGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone());
        // Additional setup...
    }
}
```

### Asset Pipeline
```rust
// Custom asset
#[derive(Asset, TypePath, Debug)]
pub struct CustomAsset {
    pub data: Vec<u8>,
    pub metadata: AssetMetadata,
}

// Asset loader
#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            // Asset loading logic
            let custom_asset = CustomAsset::from_bytes(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
}
```

## Networking

### Basic Networking
```rust
// Network message
#[derive(Serialize, Deserialize, Event)]
enum NetworkMessage {
    PlayerJoin { id: u32, name: String },
    PlayerMove { id: u32, position: Vec3 },
    PlayerLeave { id: u32 },
}

// Network plugin
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NetworkMessage>()
            .add_systems(Update, (
                handle_network_messages,
                send_network_updates,
            ));
    }
}

// Network system
fn handle_network_messages(
    mut messages: EventReader<NetworkMessage>,
    mut player_query: Query<(&NetworkId, &mut Transform)>,
) {
    for message in messages.read() {
        match message {
            NetworkMessage::PlayerMove { id, position } => {
                if let Ok((_, mut transform)) = player_query.get_mut(*id) {
                    transform.translation = *position;
                }
            }
            // Handle other messages...
        }
    }
}
```

### Replication
```rust
// Replicated component
#[derive(Component, Serialize, Deserialize)]
struct Replicated {
    last_update: f64,
    authority: NetworkId,
}

// Replication system
fn replicate_transforms(
    mut network: ResMut<NetworkConnection>,
    query: Query<(Entity, &Transform, &Replicated), Changed<Transform>>,
) {
    for (entity, transform, replicated) in &query {
        if replicated.authority == network.local_id() {
            network.broadcast(NetworkMessage::Transform {
                entity,
                transform: *transform,
            });
        }
    }
}
```

## Asset Management Patterns

### Asset Loading States
```rust
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AssetState {
    #[default]
    Loading,
    Processing,
    Ready,
    Error,
}

#[derive(Resource)]
struct AssetLoadingState {
    total: usize,
    loaded: usize,
    errors: Vec<String>,
}

fn asset_loading_system(
    mut state: ResMut<State<AssetState>>,
    mut loading_state: ResMut<AssetLoadingState>,
    asset_server: Res<AssetServer>,
) {
    match *state.get() {
        AssetState::Loading => {
            if loading_state.loaded == loading_state.total {
                state.set(AssetState::Processing).unwrap();
            }
        }
        AssetState::Processing => {
            // Process loaded assets
            state.set(AssetState::Ready).unwrap();
        }
        _ => {}
    }
}
```

### Hot Reloading
```rust
fn hot_reload_system(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Image>>,
) {
    for changed in asset_server.get_changed_assets::<Image>() {
        if let Some(texture) = textures.get_mut(&changed) {
            // Update material using changed texture
            for (_, material) in materials.iter_mut() {
                if material.base_color_texture == Some(changed) {
                    // Refresh material
                }
            }
        }
    }
}
```

## Troubleshooting

### Common Issues and Solutions

1. **Performance Issues**
```rust
// Problem: Too many entities
// Solution: Use archetype queries
fn optimize_queries(
    query: Query<&Transform, With<Specific>>,
) {
    // More efficient than querying all entities
}

// Problem: System ordering conflicts
// Solution: Use system sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    Input,
    Physics,
    Render,
}

// Configure ordering
app.configure_sets((
    GameSet::Input,
    GameSet::Physics,
    GameSet::Render,
).chain());
```

2. **Memory Leaks**
```rust
// Problem: Uncleaned resources
// Solution: Proper cleanup in systems
fn cleanup_system(
    mut commands: Commands,
    query: Query<Entity, With<Cleanup>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Problem: Asset handling
// Solution: Strong handle management
#[derive(Resource)]
struct ManagedAssets {
    handles: Vec<Handle<Image>>,
}

fn manage_assets(
    mut managed: ResMut<ManagedAssets>,
    mut assets: ResMut<Assets<Image>>,
) {
    managed.handles.retain(|handle| {
        assets.contains(handle)
    });
}
```

3. **State Management Issues**
```rust
// Problem: Race conditions
// Solution: Use proper state management
fn safe_state_transition(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Transform>,
) {
    if let Ok(()) = validate_state_change(current_state.get(), &query) {
        next_state.set(GameState::Next);
    }
}
```

### Debug Helpers
```rust
// Debug component viewer
#[cfg(debug_assertions)]
fn debug_view_system(
    query: Query<(Entity, &Transform, &Velocity)>,
) {
    for (entity, transform, velocity) in &query {
        debug!(?entity, ?transform, ?velocity);
    }
}

// State validation
#[cfg(debug_assertions)]
fn validate_game_state(world: &World) -> Result<(), String> {
    // Add validation logic
    Ok(())
}
```

## Optimized Cargo.toml Configuration

```toml
[package]
name = "bevy_game"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A Bevy game with optimized dependencies"

# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

[profile.release]
lto = "thin"
codegen-units = 1
opt-level = 3
strip = true

[dependencies]
# Core Dependencies
bevy = { version = "0.15.0", default-features = false }
bevy-inspector-egui = "0.28.0"
bevy_mod_outline = "0.9.0"
bevy_panorbit_camera = "0.21.1"
bevy_tweening = "0.12.0"
leafwing-input-manager = "0.13.2"

# Utility Dependencies
anyhow = { version = "1.0.94", features = ["backtrace"] }
thiserror = "2.0.7"
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"
rand = { version = "0.8", features = ["small_rng"] }

# Performance Dependencies
rayon = "1.8"
parking_lot = "0.12"
dashmap = "5.5"

# Development Dependencies
[dev-dependencies]
criterion = "0.5"
mockall = "0.12"
test-case = "3.3"

# Bevy Features Configuration
[dependencies.bevy]
version = "0.15.0"
default-features = false
features = [
    # Bevy functionality:
    "bevy_asset",         # Assets management
    "bevy_scene",         # Scene management
    "bevy_winit",         # Window management
    "bevy_core_pipeline", # Core rendering pipeline
    "bevy_pbr",          # 3D rendering
    "bevy_sprite",       # 2D rendering
    "bevy_text",         # Text rendering
    "bevy_ui",           # User interface
    
    # Platform-specific features:
    "x11",               # Linux: Support X11 window system
    "wayland",           # Linux: Support Wayland window system
    
    # File formats:
    "png",              # PNG image support
    "hdr",              # HDR image support
    "ktx2",             # KTX2 texture support
    "zstd",             # Compression for KTX2
    
    # Development features:
    "dynamic_linking",   # Enable dynamic linking for faster compilation (dev only)
    "trace",            # Enable tracing for performance profiling
    "trace_tracy",      # Tracy profiling support
]

# Feature flags for conditional compilation
[features]
default = ["dev-tools"]
dev-tools = ["bevy-inspector-egui", "bevy_mod_outline"]
release = []
profiling = ["bevy/trace", "bevy/trace_tracy"]
networking = ["bevy_networking_turbulence"]

# Platform-specific optimizations
[target.'cfg(target_arch = "wasm32")'.dependencies.bevy]
version = "0.15.0"
default-features = false
features = [
    "bevy_winit",
    "bevy_core_pipeline",
    "bevy_sprite",
    "bevy_pbr",
    "webgl2",
]

# Windows-specific optimizations
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "winbase"] }

# Build configuration
[build-dependencies]
build-deps = "0.1"
</code_block_to_apply_changes_from>