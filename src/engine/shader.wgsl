
struct CameraUniform {
    pos: vec2<f32>,
};
struct WindowSizeUniform {
    x: f32,
    y: f32,
};

@group(1) @binding(0) 
var<uniform> camera: CameraUniform;
@group(2) @binding(0) 
var<uniform> window_size: WindowSizeUniform;

struct InstanceInput {
    @location(5) vec2_0: vec2<f32>,
    @location(6) vec2_1: vec2<f32>,
    @location(7) vec2_2: vec2<f32>,
};

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.tex_coords = vertex.tex_coords;

    ///*
    var instance_mat = mat4x4<f32>(
        vec4<f32>(instance.vec2_0.x, instance.vec2_0.y, 0., 0.),
        vec4<f32>(instance.vec2_1.x, instance.vec2_1.y, 0., 0.),
        vec4<f32>(0., 0., 1., 0.),
        vec4<f32>(instance.vec2_2.x, instance.vec2_2.y, 0., 1.),
    );  
    instance_mat.x.x *= window_size.x;
    instance_mat.y.y *= window_size.y;
    instance_mat.w.x = instance_mat.w.x * window_size.x * 2. - 1.;
    instance_mat.w.y = instance_mat.w.y * window_size.y * -2. + 1.;

    let updated_pos = instance_mat * vec4<f32>(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0);
    let updated_model = vec4<f32>(updated_pos.x + camera.pos.x, updated_pos.y + camera.pos.y, updated_pos.z, updated_pos.w);
    out.clip_position = updated_model;
    //*/    

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}