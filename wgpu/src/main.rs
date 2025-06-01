use android_rust_example_apk_wgpu::_main;

fn main() {
    env_logger::builder()
        .filter_level(::log::LevelFilter::Info) // Default Log Level
        .parse_default_env()
        .init();

    let event_loop = ::winit::event_loop::EventLoop::builder().build()
        .expect("unable to create event loop");
    let _ = _main(event_loop);
}
