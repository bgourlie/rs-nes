use std::{fs, io::Read, mem::size_of};

use gfx_hal::{format as f, pass::Subpass, pso, Backend, Device, Primitive};

use crate::vertex::Vertex;

pub struct PipelineState<B: Backend> {
    pub pipeline: Option<B::GraphicsPipeline>,
    pub pipeline_layout: Option<B::PipelineLayout>,
}

impl<B: Backend> PipelineState<B> {
    pub unsafe fn new<IS>(desc_layouts: IS, render_pass: &B::RenderPass, device: &B::Device) -> Self
    where
        IS: IntoIterator,
        IS::Item: std::borrow::Borrow<B::DescriptorSetLayout>,
    {
        let pipeline_layout = device
            .create_pipeline_layout(desc_layouts, &[(pso::ShaderStageFlags::VERTEX, 0..8)])
            .expect("Can't create pipeline layout");

        let pipeline = {
            let vs_module = {
                let glsl = fs::read_to_string("data/quad.vert").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Vertex)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.create_shader_module(&spirv).unwrap()
            };
            let fs_module = {
                let glsl = fs::read_to_string("data/quad.frag").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Fragment)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.create_shader_module(&spirv).unwrap()
            };

            const SHADER_ENTRY_NAME: &str = "main";

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint::<B> {
                        entry: SHADER_ENTRY_NAME,
                        module: &vs_module,
                        specialization: pso::Specialization::default(),
                    },
                    pso::EntryPoint::<B> {
                        entry: SHADER_ENTRY_NAME,
                        module: &fs_module,
                        specialization: pso::Specialization::default(),
                    },
                );

                let shader_entries = pso::GraphicsShaderSet {
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry),
                };

                let subpass = Subpass {
                    index: 0,
                    main_pass: render_pass,
                };

                let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                    shader_entries,
                    Primitive::TriangleList,
                    pso::Rasterizer::FILL,
                    &pipeline_layout,
                    subpass,
                );
                pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    pso::BlendState::ALPHA,
                ));
                pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                    binding: 0,
                    stride: size_of::<Vertex>() as u32,
                    rate: 0,
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: f::Format::Rg32Float,
                        offset: 0,
                    },
                });
                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: f::Format::Rg32Float,
                        offset: 8,
                    },
                });

                device.create_graphics_pipeline(&pipeline_desc, None)
            };

            device.destroy_shader_module(vs_module);
            device.destroy_shader_module(fs_module);

            pipeline.unwrap()
        };

        PipelineState {
            pipeline: Some(pipeline),
            pipeline_layout: Some(pipeline_layout),
        }
    }

    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            device.destroy_graphics_pipeline(
                state.pipeline.take().expect("Pipeline shouldn't be None"),
            );
            device.destroy_pipeline_layout(
                state
                    .pipeline_layout
                    .take()
                    .expect("Pipeline layout shouldn't be None"),
            );
        }
    }
}
