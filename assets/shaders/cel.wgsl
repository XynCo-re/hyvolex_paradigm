struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct CelMaterial {
    color: vec4<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> material: CelMaterial;

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    out.world_position = vertex.position;
    out.world_normal = vertex.normal;
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    let V = normalize(camera.view_position.xyz - in.world_position);
    let NdotV = max(dot(N, V), 0.0);
    
    // Simple cel shading with 3 levels
    let cel = floor(NdotV * 3.0) / 3.0;
    
    // Add simple rim lighting
    let rim_light = pow(1.0 - NdotV, 3.0);
    
    let final_color = material.color * cel;
    
    return vec4<f32>(
        final_color.rgb + rim_light * 0.3,
        final_color.a
    );
} 