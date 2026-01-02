// SDF (Signed Distance Field) Text Rendering Shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@group(0) @binding(0)
var glyph_atlas: texture_2d<f32>;
@group(0) @binding(1)
var glyph_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coords = input.tex_coords;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the distance value from the atlas (0-1 range)
    let dist = textureSample(glyph_atlas, glyph_sampler, input.tex_coords).r;
    
    // For standard alpha rendering (not SDF yet, will upgrade later)
    // Just use the sampled value as alpha
    let alpha = dist;
    
    // Apply text color with alpha
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}

// Future SDF enhancement:
// @fragment
// fn fs_main_sdf(input: VertexOutput) -> @location(0) vec4<f32> {
//     let dist = textureSample(glyph_atlas, glyph_sampler, input.tex_coords).r;
//     
//     // SDF rendering with anti-aliasing
//     let threshold = 0.5;
//     let smoothness = 0.25 / fwidth(dist);
//     let alpha = smoothstep(threshold - smoothness, threshold + smoothness, dist);
//     
//     return vec4<f32>(input.color.rgb, input.color.a * alpha);
// }
