#![allow(dead_code, unused_variables, unused_assignments)]
use ::std::os::unix::prelude::{AsRawFd, OwnedFd};
use ::std::sync::Arc;

use ::log::info;
use ::winit::event::TouchPhase;
#[cfg(target_os="android")]
use ::winit::platform::android::ActiveEventLoopExtAndroid;
use ::winit::{application::ApplicationHandler, keyboard::PhysicalKey};
use ::winit::event_loop::EventLoop;
#[allow(unused_imports)]
use ::winit::keyboard::NativeKeyCode;
#[allow(unused_imports)]
use ::winit::event::ElementState;
#[cfg(target_os="android")]
mod notification;
#[cfg(target_os="android")]
mod jni_looper;

pub struct ApplicationState {
    pub gfx: Option<GfxState>,
    #[cfg(target_os="android")]
    pub native_window: Option<ndk::native_window::NativeWindow>,
    pub pipe: Option<[OwnedFd;2]>,
}
pub struct GfxState {
    pub window: Arc<winit::window::Window>,
    pub cursor_position: ::winit::dpi::PhysicalPosition<f64>,
    pub did_resize: bool,
    pub size: (u32, u32),
}
impl ApplicationState  {
    pub fn new () -> Self {
        Self {
            gfx: None,
            #[cfg(target_os="android")]
            native_window: None,
            pipe: None,
        }
    }

    pub fn with_looper(&mut self, looper: [OwnedFd;2]) -> &mut Self {
        self.pipe = Some(looper);
        self
    }

    #[cfg(target_os="android")]
    pub fn with_native_window(&mut self, native_window: Option<ndk::native_window::NativeWindow>) -> &mut Self {
        info!("redraw: native window is some: {}", native_window.is_some());
        self.native_window = native_window;
        self
    }

}

impl ApplicationHandler for ApplicationState {

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        info!("Window resumed");
        // let ndk_context = ndk_context::android_context();
        let window = Arc::new(
            event_loop.create_window(winit::window::WindowAttributes::default())
                .expect("cannot create window")
        );
        let physical_size = window.inner_size();
        self.gfx = Some(GfxState{
            window: window.clone(),
            cursor_position: ::winit::dpi::PhysicalPosition::<f64> {x: 0.0, y: 0.0},
            did_resize: false,
            size: (physical_size.width, physical_size.height),
        });
        #[cfg(target_os="android")]
        self.with_native_window(event_loop.android_app().native_window());
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
            cursor_position,
            did_resize,
            size,
        } = self.gfx.as_mut().expect("cannot destruct gfxstate, even though it is not none");

        match event {
            ::winit::event::WindowEvent::CloseRequested => {
                self.gfx = None;
                event_loop.exit();
                ()
            },
            ::winit::event::WindowEvent::Touch(touch) => {
                *cursor_position = touch.location;
                info!("window event touch");
                #[cfg(target_os="android")]
                if touch.phase == TouchPhase::Ended {

                    if let Some(pipe) = self.pipe.as_mut() {
                        info!("pipe was some under touch");
                        let _ = unsafe { libc::write(
                            pipe[1].as_raw_fd(),
                            &1_u8 as *const u8 as *const ::std::os::raw::c_void,
                            1
                        )};
                    }
                }
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
            ::winit::event::WindowEvent::Resized(physical_size) => {
                info!("resized {} x {}", physical_size.width, physical_size.height);
                *did_resize = true;
                window.request_redraw();
            },
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
                    if *did_resize {
                        let wsize = window.inner_size();
                        *size = (wsize.width, wsize.height);
                        *did_resize = false;
                    }
                    info!("redraw requested");

                    #[cfg(target_os="android")]
                    if self.native_window.is_some() {
                        info!("redraw requested: native_window is_some");
                        android::dummy_render(self.native_window.as_ref().unwrap());
                    }
            }
        }
    }
}

pub fn _main(event_loop: EventLoop<()>, app: &mut ApplicationState) -> Result<(), winit::error::EventLoopError>
{
    event_loop.set_control_flow(::winit::event_loop::ControlFlow::Wait);
    event_loop.run_app(app)
}

#[cfg(target_os = "android")]
mod android {
    use crate::ApplicationState;
    use super::_main;
    use super::jni_looper::setup_looper;
    use android_activity::AndroidApp;
    use ::log::info;
    use winit::platform::android::EventLoopBuilderExtAndroid;
    #[no_mangle]
    pub fn android_main(aapp: AndroidApp) {

        ::android_logger::init_once(
            ::android_logger::Config::default()
                .with_tag("NAWINITWGPU")
                .with_max_level(::log::LevelFilter::Info)
        );
        log::info!("android_main setup up looper");
        let looper = setup_looper(&aapp).expect("setup looper");

        let mut app = ApplicationState::new();
        app.with_looper(looper);
        app.with_native_window(aapp.native_window());

        log::info!("android_main started");
        let event_loop = ::winit::event_loop::EventLoop::builder()
            .with_android_app(aapp).build().unwrap();

        let _ = _main(event_loop, &mut app);
    }

    /// Post a NOP frame to the window
    ///
    /// Since this is a bare minimum test app we don't depend
    /// on any GPU graphics APIs but we do need to at least
    /// convince Android that we're drawing something and are
    /// responsive, otherwise it will stop delivering input
    /// events to us.
    /// @see android-activity examples/na-mainloop/src/lib/rs
    pub fn dummy_render(native_window: &ndk::native_window::NativeWindow) {
        info!("redraw requested: dummy_render");
        unsafe {
            let mut buf: ndk_sys::ANativeWindow_Buffer = std::mem::zeroed();
            let mut rect: ndk_sys::ARect = std::mem::zeroed();
            ndk_sys::ANativeWindow_lock(
                native_window.ptr().as_ptr() as _,
                &mut buf as _,
                &mut rect as _,
            );
            // Note: we don't try and touch the buffer since that
            // also requires us to handle various buffer formats
            ndk_sys::ANativeWindow_unlockAndPost(native_window.ptr().as_ptr() as _);
        }
    }
}
