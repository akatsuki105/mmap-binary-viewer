use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    // init imgui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    // init imgui sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut event_pump = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        imgui_sdl2_context.prepare_frame(
            imgui_context.io_mut(),
            &window,
            &event_pump.mouse_state(),
        );

        let ui = imgui_context.frame();

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        imgui_sdl2_context.prepare_render(&ui, &window);
        renderer.render(ui);

        window.gl_swap_window();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
