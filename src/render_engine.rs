pub mod render_engine {
    use glam::Vec3;
    use wgpu::{include_wgsl, util::DeviceExt};
    use winit::window::Window;
	use pollster::FutureExt; 
	
	macro_rules! SHADERS {() => {"shaders.wgsl"};}
	
	pub struct RenderEngine {
		surface: wgpu::Surface,
		//instance: wgpu::Instance,
		adapter: wgpu::Adapter,
		device: wgpu::Device,
		queue: wgpu::Queue,
		surface_config: wgpu::SurfaceConfiguration,
		render_pipline: wgpu::RenderPipeline,
		vertex_buffer: wgpu::Buffer,
	}

	#[repr(C)]
	#[derive(Clone, Copy, Debug)]
	struct Vertex {
		vertex: Vec3,
		colour: Vec3,
	}

	impl RenderEngine {
		pub fn new(window: &Window) -> Self {
			let instance = wgpu::Instance::new(wgpu::Backends::DX12);
			
			let surface = unsafe{instance.create_surface(window)};

 			let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,	
			}).block_on().expect("failed to get device");

			let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{
				features: wgpu::Features::default(),
				limits: wgpu::Limits{..Default::default()},
				label: Some("device and queues"), 
			},
			None).block_on().expect("failed to get device and queues");
			
			let surface_config = wgpu::SurfaceConfiguration{
				usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
				format: wgpu::TextureFormat::Rgba8Unorm,
				width: window.inner_size().width,
				height: window.inner_size().height,
				present_mode: wgpu::PresentMode::Immediate,
			};
			surface.configure(&device, &surface_config);

			let shaders = device.create_shader_module(include_wgsl!(SHADERS!()));
			let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
				label: Some("pipline layout"),
				bind_group_layouts: &[],
				push_constant_ranges: &[],

			});
			
			let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
				label: Some("pipeline"),
				layout: Some(&pipeline_layout),
				vertex: wgpu::VertexState{
					module: &shaders,
					entry_point: "vert_main",
					buffers: &[],
				},
				fragment: Some(wgpu::FragmentState{
					module: &shaders,
					entry_point: "frag_main",
					targets: &[Some(wgpu::ColorTargetState{
						format: surface_config.format,
						blend: Some(wgpu::BlendState::REPLACE),
						write_mask: wgpu::ColorWrites::ALL,	
					})]
				}),
				primitive: wgpu::PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
					strip_index_format: None,
					front_face: wgpu::FrontFace::Ccw,
					cull_mode: Some(wgpu::Face::Back),
					polygon_mode: wgpu::PolygonMode::Fill,
					unclipped_depth: false,
					conservative: false,
				},
				depth_stencil: None,
				multisample: wgpu::MultisampleState { 
					count: 1, 
					mask: !0, //just not zero, thus all 
					alpha_to_coverage_enabled: false 
				},
				multiview: None,
			});


//			let vertices = &[
//				([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
//				([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]),
//				([1.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
//			];
			let vertices = &[
				Vertex{vertex: Vec3 { x: 1.0, y: 0.0, z: 0.0}, colour: Vec3 { x: 1.0, y: 1.0, z: 1.0 }},
				Vertex{vertex: Vec3 { x: 0.5, y: 0.5, z: 0.0}, colour: Vec3 { x: 1.0, y: 1.0, z: 1.0 }},
				Vertex{vertex: Vec3 { x: -0.5, y: 0.5, z: 0.0}, colour: Vec3 { x: 1.0, y: 1.0, z: 1.0 }},
			];
			let vertices_bytes = unsafe {std::slice::from_raw_parts(vertices[..].as_ptr() as *const u8, vertices.len() * std::mem::size_of::<Vertex>())};

			let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
				label: Some("vertex buffer"),
				contents: vertices_bytes,
				usage: wgpu::BufferUsages::VERTEX,
			});
			
			Self{
				surface: surface,
				adapter: adapter,
				device: device,
				queue: queue,			
				surface_config: surface_config,			
				render_pipline: render_pipeline,
				vertex_buffer: vertex_buffer,
			}
		}

		pub fn render(&mut self) {
			let output = self.surface.get_current_texture().expect("failed to get surface texture");
			let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

			let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
				label: Some("encoder"),
			});

			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
				label: Some("render pass encoding"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment{
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations{
						load: wgpu::LoadOp::Clear(wgpu::Color{
							r: 0.5,
							b: 0.5,
							g: 1.0,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
			render_pass.set_pipeline(&self.render_pipline);
			render_pass.draw(0..3, 0..1);

			drop(render_pass);
			
			self.queue.submit(std::iter::once(encoder.finish()));
			output.present();
		}
	}
}