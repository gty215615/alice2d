// Vertex shader bindings

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

// struct Locals {
//     screen_size: vec2<f32>,
//     // Uniform buffers need to be at least 16 bytes in WebGL.
//     // See https://github.com/gfx-rs/wgpu/issues/2072
//     _padding: vec2<u32>,
// };
// @group(0) @binding(0) var<uniform> r_locals: Locals;

// 0-1 from 0-255
fn linear_from_srgb(srgb: vec3<f32>) -> vec3<f32> {
    let cutoff = srgb < vec3<f32>(10.31475);
    let lower = srgb / vec3<f32>(3294.6);
    let higher = pow((srgb + vec3<f32>(14.025)) / vec3<f32>(269.025), vec3<f32>(2.4));
    return select(higher, lower, cutoff);
}

// [u8; 4] SRGB as u32 -> [r, g, b, a]
fn unpack_color(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32(color & 255u),
        f32((color >> 8u) & 255u),
        f32((color >> 16u) & 255u),
        f32((color >> 24u) & 255u),
    );
}

fn position_from_screen(screen_pos: vec2<f32>) -> vec4<f32> {
    return vec4<f32>(
        2.0 * screen_pos.x / 1280.0 - 1.0,
        1.0 - 2.0 * screen_pos.y /760.0,
        0.0,
        1.0,
    );
}

@vertex
fn vs_main(
    @location(0) a_pos: vec2<f32>,
    @location(1) a_tex_coord: vec2<f32>,
    @location(2) a_color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = a_tex_coord;
    let color = a_color;
    out.color = vec4<f32>(linear_from_srgb(color.rgb), color.a / 255.0);
    out.position = position_from_screen(a_pos);
    return out;
}



// Fragment shader bindings

// @group(1) @binding(0) var r_tex_color: texture_2d<f32>;
// @group(1) @binding(1) var r_tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return in.color * textureSample(r_tex_color, r_tex_sampler, in.tex_coord);
    return vec4<f32>(1.0,0.0,0.0,1.0);
}