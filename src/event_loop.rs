// From https://github.com/gifnksm/game-of-life-rs
// Thanks!

#[cfg(not(target_os = "emscripten"))]
pub mod event_loop {
    use piston::event_loop::{EventSettings, Events};
    use piston::input::Input;
    use sdl2_window::Sdl2Window;

    pub fn run<T>(mut window: Sdl2Window,
                  handler: fn(w: &mut Sdl2Window, e: Input, a: &mut T),
                  mut arg: T) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            handler(&mut window, e, &mut arg);
        }
    }
}

#[cfg(target_os = "emscripten")]
pub mod event_loop {
    extern crate emscripten_sys;

    use piston::input::{AfterRenderArgs, Input, RenderArgs, UpdateArgs};
    use piston::window::Window;
    use sdl2_window::Sdl2Window;
    use std::mem;
    use std::os::raw::c_void;

    struct EventLoop<T> {
        last_updated: f64,
        window: Sdl2Window,
        handler: fn(window: &mut Sdl2Window, e: Input, arg: &mut T),
        arg: T,
    }

    pub fn run<T>(window: Sdl2Window,
                  handler: fn(w: &mut Sdl2Window, e: Input, a: &mut T),
                  arg: T) {
        unsafe {
            let mut events = Box::new(EventLoop {
                last_updated: emscripten_sys::emscripten_get_now() as f64,
                window: window,
                handler: handler,
                arg: arg,
            });
            let events_ptr = &mut *events as *mut EventLoop<_> as *mut c_void;
            emscripten_sys::emscripten_set_main_loop_arg(Some(main_loop_c::<T>), events_ptr, 0, 1);
            mem::forget(events);
        }
    }

    extern "C" fn main_loop_c<T>(arg: *mut c_void) {
        unsafe {
            let mut events: &mut EventLoop<T> = mem::transmute(arg);
            let window = &mut events.window;
            let handler = events.handler;
            let arg = &mut events.arg;
            window.swap_buffers();

            let e = Input::AfterRender(AfterRenderArgs);
            handler(window, e, arg);

            while let Some(e) = window.poll_event() {
                handler(window, e, arg);
            }

            if window.should_close() {
                emscripten_sys::emscripten_cancel_main_loop();
                return;
            }

            let now = emscripten_sys::emscripten_get_now() as f64;
            let dt = now - events.last_updated;
            events.last_updated = now;

            let e = Input::Update(UpdateArgs { dt: dt });
            handler(window, e, arg);

            let size = window.size();
            let draw_size = window.draw_size();
            let e = Input::Render(RenderArgs {
                ext_dt: dt,
                width: size.width,
                height: size.height,
                draw_width: draw_size.width,
                draw_height: draw_size.height,
            });
            handler(window, e, arg);
        }
    }
}