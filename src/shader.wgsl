
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

struct VertexInput {
    @location(0) pos: vec2<f32>,
};

struct TexCoordsInput {
    @location(1) coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) vec2_0: vec2<f32>,
    @location(3) vec2_1: vec2<f32>,
    @location(4) vec2_2: vec2<f32>,
    @location(5) index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) index: u32
};

@vertex
fn vs_main(
    vertex: VertexInput,
    tex_coords: TexCoordsInput,
    instance: InstanceInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.tex_coords = tex_coords.coords;
    out.index = instance.index;

    var instance_mat = mat4x4<f32>(
        vec4<f32>(instance.vec2_0.x, instance.vec2_0.y, 0., 0.),
        vec4<f32>(instance.vec2_1.x, instance.vec2_1.y, 0., 0.),
        vec4<f32>(0., 0., 1., 0.),
        vec4<f32>(instance.vec2_2.x, instance.vec2_2.y, 0., 1.),
    );  
    //instance_mat.x.x *= window_size.x;
    //instance_mat.y.y *= window_size.y;
    instance_mat.w.x = instance_mat.w.x * window_size.x * 2. - 1.;
    instance_mat.w.y = instance_mat.w.y * window_size.y * -2. + 1.;

    let updated_pos = instance_mat * vec4<f32>(vertex.pos.x, vertex.pos.y, 0., 1.);
    let updated_model = vec4<f32>(updated_pos.x + camera.pos.x, updated_pos.y + camera.pos.y, updated_pos.z, updated_pos.w);
    out.clip_position = updated_model;

    return out;
}

@group(0)@binding(0)
var sam: sampler;
@group(0) @binding(1)
var tex_array: binding_array<texture_2d<f32>>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex_array[in.index], sam, in.tex_coords);
}