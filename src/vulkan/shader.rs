vulkano_shaders::shader! {
    vulkan_version: "1.2",
    spirv_version: "1.6",
    shaders: {
        vertex: {
            ty: "vertex",
            path: "shaders/shader.vert",
        },
        fragment: {
            ty: "fragment",
            path: "shaders/shader.frag",
        }
    }
}
