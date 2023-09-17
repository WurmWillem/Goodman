
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
var<uniform> win_size: WindowSizeUniform;

struct VertexInput {
    @location(0) pos: vec2<f32>,
};

struct TexCoordsInput {
    @location(1) vec2_0: vec2<f32>,
    @location(2) vec2_1: vec2<f32>,
    @location(3) vec2_2: vec2<f32>,
    @location(4) vec2_3: vec2<f32>,
}

struct InstanceInput {
    @location(5) vec2_0: vec2<f32>,
    @location(6) vec2_1: vec2<f32>,
    @location(7) vec2_2: vec2<f32>,
    @location(8) index: u32,
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
    out.index = instance.index;

    if vertex.pos.x == 0. && vertex.pos.y == -2. {
        out.tex_coords = tex_coords.vec2_0;
    } else if vertex.pos.x == 2. && vertex.pos.y == -2. {
        out.tex_coords = tex_coords.vec2_1;
    } else if vertex.pos.x == 2. && vertex.pos.y == 0. {
        out.tex_coords = tex_coords.vec2_2;
    } else if vertex.pos.x == 0. && vertex.pos.y == 0. {
        out.tex_coords = tex_coords.vec2_3;
    } 

    var instance_mat = mat4x4<f32>(
        vec4<f32>(instance.vec2_0.x , instance.vec2_0.y, 0., 0.),
        vec4<f32>(instance.vec2_1.x, instance.vec2_1.y, 0., 0.),
        vec4<f32>(0., 0., 1., 0.),
        vec4<f32>(instance.vec2_2.x, instance.vec2_2.y, 0., 1.),
    );  
    
    instance_mat.w.x = instance_mat.w.x * win_size.x * 2. - 1.;
    instance_mat.w.y = instance_mat.w.y * win_size.y * -2. + 1.;

    let updated_model = instance_mat * vec4<f32>(vertex.pos.x * win_size.x, vertex.pos.y * win_size.y, 0., 1.);
    // let updated_model = vec4<f32>(updated_pos.x + camera.pos.x, updated_pos.y + camera.pos.y, updated_pos.z, updated_pos.w);
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