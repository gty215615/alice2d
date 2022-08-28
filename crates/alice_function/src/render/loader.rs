
pub struct ShaderLoader {}


impl ShaderLoader {
    
    pub fn load(
        device:&wgpu::Device,
        path: &str
    ) -> wgpu::ShaderModule {

        let shader_source = ShaderLoader::reader_source(path);
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        })
      
    }  

    pub fn reader_source(path: &str) -> String {
        println!("path={}",path);
        std::fs::read_to_string(path).unwrap()
        
    }
    
}