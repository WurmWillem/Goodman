
struct CameraUniform {
    pos: vec2<f32>,
};

@group(1) @binding(0) 
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) matrix_0: vec4<f32>,
    @location(6) matrix_1: vec4<f32>,
    @location(7) matrix_2: vec4<f32>,
    @location(8) matrix_3: vec4<f32>,
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
    let x = vec4<f32>(instance.matrix_0.x / 1200., instance.matrix_0.y, instance.matrix_0.z, instance.matrix_0.w);
    let y = vec4<f32>(instance.matrix_1.x, instance.matrix_1.y / 800., instance.matrix_1.z, instance.matrix_1.w);
    let w = vec4<f32>(instance.matrix_3.x / 600. - 1., instance.matrix_3.y / 400. - 1., instance.matrix_3.z, instance.matrix_3.w);

    let model_matrix = mat4x4<f32>(
        x,
        y,
        instance.matrix_2,
        w,
    );  
    /*let model_matrix = mat4x4<f32>(
        instance.matrix_0,
        instance.matrix_1,
        instance.matrix_2,
        instance.matrix_3,
    );*/

    let updated_pos = model_matrix * vec4<f32>(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0);
    let updated_model = vec4<f32>(updated_pos.x + camera.pos.x, updated_pos.y + camera.pos.y, updated_pos.z, updated_pos.w);
    out.clip_position = updated_model;

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