#![allow(dead_code, unused_variables, unused_assignments)]
use ::std::sync::Arc;

use ::log::info;
use ::wgpu::{Instance, Surface};
use ::winit::{application::ApplicationHandler, keyboard::PhysicalKey};
use ::winit::event_loop::EventLoop;
#[allow(unused_imports)]
use ::winit::keyboard::NativeKeyCode;
#[allow(unused_imports)]
use ::winit::event::ElementState;

struct ApplicationState<'window> {
    pub gfx: Option<GfxState<'window>>
}
struct GfxState<'window> {
    pub window: Arc<winit::window::Window>,
    pub surface_configured: bool,
    pub surface: ::wgpu::Surface<'window>,
    pub adapter: ::wgpu::Adapter,
    pub device: ::wgpu::Device,
    pub queue: ::wgpu::Queue,
    pub cursor_position: ::winit::dpi::PhysicalPosition<f64>,
}
impl <'window>ApplicationState<'window>  {
    pub fn new () -> Self {
        Self {
            gfx: None,
        }
    }
}
impl <'window>ApplicationState<'window> {
    fn init_wgpu(&mut self, instance: &Instance, surface: &Surface) -> (wgpu::TextureFormat, wgpu::Adapter, wgpu::Device, wgpu::Queue) {
        let adapter = ::futures::executor::block_on(async {
             ::wgpu::util::initialize_adapter_from_env_or_default(
                instance,
                Some(surface)
            ).await.expect("cannot create adapter from env or default")
        });
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities.formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else (|| capabilities.formats.first().copied())
            .expect("get preferred format");
        let (device, queue) = futures::executor::block_on(async {
            adapter.request_device(
                &wgpu::DeviceDescriptor{
                    label: None,
                    required_features: adapter.features(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None
            ).await.expect("Request device")
        });

        (
            format,
            adapter,
            device,
            queue,
        )
    }

    pub fn ensure_render_state_for_surface(&mut self, window: Arc<::winit::window::Window>) {
        let instance = ::wgpu::Instance::new(&::wgpu::InstanceDescriptor {
            backends: ::wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).expect("create surface from window");

        let (format, adapter, device, queue) = self.init_wgpu(&instance, &surface);

        let physical_size = window.inner_size();
        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: physical_size.width,
                height: physical_size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 10,
            },
        );

        self.gfx = Some(GfxState{
            window: window.clone(),
            surface_configured: true,
            surface,
            adapter,
            device,
            queue,
            cursor_position: ::winit::dpi::PhysicalPosition::<f64> {x: 0.0, y: 0.0},
        });
    }
}
impl <'window>ApplicationHandler for ApplicationState<'window> {
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.gfx.is_none() {
            event_loop.exit();
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(winit::window::WindowAttributes::default())
                .expect("cannot create window")
        );
        self.ensure_render_state_for_surface(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.gfx.is_none() {
            return;
        }
        let window = match self.gfx.as_ref() {
            Some(gfx) => gfx.window.clone(),
            None => return,
        };
        let GfxState {
            window,
            surface_configured,
            surface,
            adapter,
            device,
            queue,
            mut cursor_position,
        } = self.gfx.as_ref().unwrap();


        match event {
            ::winit::event::WindowEvent::CloseRequested => {
                self.gfx = None;
                ()
            },
            ::winit::event::WindowEvent::Touch(touch) => {
                cursor_position = touch.location;
            },
            ::winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                #[cfg(target_os="android")]
                if let PhysicalKey::Unidentified(ncode) = event.physical_key {
                    match ncode {
                        NativeKeyCode::Android(val) if val == ::android_activity::input::Button::Back.into() => {
                            match event.state {
                                ElementState::Pressed => info!("back motion motioned"),
                                ElementState::Released => info!("back motion released"),
                            }
                        },
                        NativeKeyCode::Android(val) if val == ::android_activity::input::Keycode::Back.into() => {
                            match event.state {
                                ElementState::Pressed => info!("back button pressed"),
                                ElementState::Released => event_loop.exit(),
                            }
                        },
                        _ => ()
                    }
                }
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if event.state.is_pressed() {
                        info!("key pressed {}", key_code as u32);
                    } else {
                        info!("key released {}", key_code as u32);
                    }
                }
            },
            ::winit::event::WindowEvent::ActivationTokenDone { serial, token } => (),
            ::winit::event::WindowEvent::Resized(physical_size) => (),
            ::winit::event::WindowEvent::Moved(physical_position) => (),
            ::winit::event::WindowEvent::Destroyed => (),
            ::winit::event::WindowEvent::DroppedFile(path_buf) => (),
            ::winit::event::WindowEvent::HoveredFile(path_buf) => (),
            ::winit::event::WindowEvent::HoveredFileCancelled => (),
            ::winit::event::WindowEvent::Focused(_) => (),
            ::winit::event::WindowEvent::ModifiersChanged(modifiers) => (),
            ::winit::event::WindowEvent::Ime(ime) => (),
            ::winit::event::WindowEvent::CursorMoved { device_id, position } => (),
            ::winit::event::WindowEvent::CursorEntered { device_id } => (),
            ::winit::event::WindowEvent::CursorLeft { device_id } => (),
            ::winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => (),
            ::winit::event::WindowEvent::MouseInput { device_id, state, button } => (),
            ::winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => (),
            ::winit::event::WindowEvent::PanGesture { device_id, delta, phase } => (),
            ::winit::event::WindowEvent::DoubleTapGesture { device_id } => (),
            ::winit::event::WindowEvent::RotationGesture { device_id, delta, phase } => (),
            ::winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => (),
            ::winit::event::WindowEvent::AxisMotion { device_id, axis, value } => (),
            ::winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => (),
            ::winit::event::WindowEvent::ThemeChanged(theme) => (),
            ::winit::event::WindowEvent::Occluded(_) => (),
            ::winit::event::WindowEvent::RedrawRequested => {
                   // if *resized {
                    //     let size = window.inner_size();

                    //     *viewport = Viewport::with_physical_size(
                    //         Size::new(size.width, size.height),
                    //         window.scale_factor(),
                    //     );

                    //     surface.configure(
                    //         device,
                    //         &wgpu::SurfaceConfiguration {
                    //             format: *format,
                    //             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    //             width: size.width,
                    //             height: size.height,
                    //             present_mode: wgpu::PresentMode::AutoVsync,
                    //             alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    //             view_formats: vec![],
                    //             desired_maximum_frame_latency: 2,
                    //         },
                    //     );

                    //     *resized = false;
                    // }

                    match surface.get_current_texture() {
                        Ok(frame) => {
                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor { label: None },
                            );

                            // let program = state.program();

                            let view = frame.texture.create_view(
                                &wgpu::TextureViewDescriptor::default(),
                            );

                            {
                                // We clear the frame
                                // let mut render_pass = Scene::clear(
                                //     &view,
                                //     &mut encoder,
                                //     program.background_color(),
                                // );

                                // Draw the scene
                                // scene.draw(&mut render_pass);
                            }

                            // And then iced on top
                            // renderer.present(
                            //     engine,
                            //     device,
                            //     queue,
                            //     &mut encoder,
                            //     None,
                            //     frame.texture.format(),
                            //     &view,
                            //     viewport,
                            //     &debug.overlay(),
                            // );

                            // Then we submit the work
                            // engine.submit(queue, encoder);
                            frame.present();

                            // Update the mouse cursor
                            // window.set_cursor(
                            //     iced_winit::conversion::mouse_interaction(
                            //         state.mouse_interaction(),
                            //     ),
                            // );
                        }
                        Err(error) => match error {
                            wgpu::SurfaceError::OutOfMemory => {
                                panic!(
                                    "Swapchain error: {error}. \
                                Rendering cannot continue."
                                )
                            }
                            _ => {
                                // Try rendering again next frame.
                                window.request_redraw();
                            }
                        },
                    }
            }
        }
    }
}

pub fn _main( event_loop: EventLoop<()>) -> Result<(), winit::error::EventLoopError>
{
    let mut app = ApplicationState::new();
    event_loop.run_app(&mut app)
}

#[cfg(target_os = "android")]
mod android {
    use super::_main;
    use android_activity::AndroidApp;
    use winit::platform::android::EventLoopBuilderExtAndroid;
    #[no_mangle]
    pub fn android_main(app: AndroidApp) {

        log::info!("android_main started");
        let event_loop = ::winit::event_loop::EventLoop::builder()
            .with_android_app(app).build().unwrap();
        let _ = _main(event_loop);
    }

}
