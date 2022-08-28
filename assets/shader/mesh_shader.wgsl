
struct PointLight {
    position:               vec3<f32>,
    _padding_position:      f32,
    color:                  vec4<f32>
}

struct AliceScene {
    view_proj:  mat4x4<f32>,
    point_light:PointLight
}

@group(0) @binding(0) // 1.
var<uniform> alice_scene:AliceScene;


@group(1) @binding(0) var<storage, read> instance_models : array<mat4x4<f32>>;



// 顶点着色器

struct VertexInput {
     @location(0) position:     vec4<f32>,
     @location(1) normal:       vec4<f32>,
     @location(2) tex_coords:   vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position:   vec4<f32>,
    @location(0) tex_coords:            vec2<f32>,
    @location(1) world_position:        vec3<f32>,
    @location(2) normal:                vec3<f32>
};

@vertex
fn vs_main(
    @builtin(instance_index) index : u32,
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = instance_models[index];
    let normal = model.normal;
    out.tex_coords = model.tex_coords.xy;
    let resolution = 800.0 / 600.0;
    out.clip_position =alice_scene.view_proj * model_matrix * vec4<f32>(model.position.x * resolution,model.position.y ,model.position.z, 1.0); // 2.
    out.world_position = (model_matrix * vec4<f32>(model.position.xyz,1.0)).xyz;
    out.normal = model.normal.xyz;
    return out;
}


// Fragment shader

struct WebgpuPBRMaterialColor {
        base_color:vec4<f32>,
        // metallic:f32,
        // roughness:f32,
        // normal_scale:f32,
        // occlusion_strength:f32,
        // emissive:vec3<f32>,
        // is_blend:u8,
        // pub is_double_sided:u8,
};


@group(2) @binding(0) // 1.
var<uniform> pbr_material_color: vec4<f32>;
@group(2) @binding(1)
var Sampler: sampler;
@group(2) @binding(2)
var base_color_texture: texture_2d<f32>;


fn get_diffuse_color(tex_coords:vec2<f32>) -> vec4<f32> {
    let color = textureSample(base_color_texture, Sampler, tex_coords) * pbr_material_color;
    return color;    
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let normal = normalize(in.normal);

    let light_direction = normalize(alice_scene.point_light.position - in.world_position);

    let diff = max(dot(normal, light_direction) , 0.0);

  
      let ambient = 0.1 ;
    let light_color = (ambient + diff) * alice_scene.point_light.color;

    let diffuse_color = get_diffuse_color(in.tex_coords);
    
 

    let result = light_color * diffuse_color;
    return diffuse_color;
}



