use std::iter;
 
//extern crate futures;
extern crate sdl2;
extern crate wgpu; 

use pollster::block_on;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Testing SDL with WGPU", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let (width, height) = window.size();


	// The instance is a handle to our GPU
	// BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
	let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
	let surface = unsafe { instance.create_surface(&window) };
	let adapter = block_on(instance
		.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			//power_preference: wgpu::PowerPreference::default(),
			compatible_surface: Some(&surface),
		})
		).unwrap();



	let (device, queue) = block_on(adapter
		.request_device(
			&wgpu::DeviceDescriptor {
				label: None,
				features: wgpu::Features::empty(),
				limits: wgpu::Limits::default(),
			},
			None, // Trace path
		)
		).unwrap();
/*
	let mut sc_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
		format: wgpu::TextureFormat::Bgra8UnormSrgb,
		width: width,
		height: height,
		present_mode: wgpu::PresentMode::Fifo,
	};
	*/
	//let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

	let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
		label: Some("Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
	});

	let render_pipeline_layout =
		device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

	let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

	let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
		label: Some("Render Pipeline"),
		layout: Some(&render_pipeline_layout),
		vertex: wgpu::VertexState {
			module: &shader,
			entry_point: "main",
			buffers: &[],
		},
		fragment: Some(wgpu::FragmentState {
			module: &shader,
			entry_point: "main",
			targets: &[swapchain_format.into()],
		}),
		primitive: wgpu::PrimitiveState {
			topology: wgpu::PrimitiveTopology::TriangleList,
			strip_index_format: None,
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: Some(wgpu::Face::Back),
			// Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
			polygon_mode: wgpu::PolygonMode::Fill,
			// Requires Features::DEPTH_CLAMPING
			clamp_depth: false,
			// Requires Features::CONSERVATIVE_RASTERIZATION
			conservative: false,
		},
		depth_stencil: None,
		multisample: wgpu::MultisampleState {
			count: 1,
			mask: !0,
			alpha_to_coverage_enabled: false,
		},
	});


	let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: width,
        height: height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    surface.configure(&device, &config);

	let mut event_pump = sdl_context.event_pump()?;
    'running: loop 
	{
        for event in event_pump.poll_iter() 
		{
            match event 
			{
                Event::Window {
                    win_event:WindowEvent::Resized(width, height),
					..
				} => {
					// Reconfigure the surface with the new size
					config.width = width as u32;
					config.height = height as u32;
					surface.configure(&device, &config);
				}
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

		let frame = surface
		.get_current_frame()
		.expect("Failed to acquire next swap chain texture")
		.output;

		let view = frame
		.texture
		.create_view(&wgpu::TextureViewDescriptor::default());
		//let frame = swap_chain.get_current_frame().unwrap().output;

        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.2,
                            b: 0.4,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(iter::once(encoder.finish()));
    }

    Ok(())
}



