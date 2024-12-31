use std::error::Error;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use softbuffer::Context;
use winit::raw_window_handle::HasDisplayHandle;
use winit::raw_window_handle::DisplayHandle;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::window::WindowId;
use winit::dpi::{PhysicalPosition, PhysicalSize};
// use winit::dpi::{PhysicalSize};

#[path = "util/tracing.rs"]
mod tracing;
use winit::application::ApplicationHandler;

fn main() -> Result<(), Box<dyn Error>> {

    tracing::init();

    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let _event_loop_proxy = event_loop.create_proxy();

    // Wire the user event from another thread.
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
 //       println!("Starting to send user event every second");
        loop {
            let _ = _event_loop_proxy.send_event(UserEvent::WakeUp);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut state = Application::new(&event_loop);

    event_loop.run_app(&mut state).map_err(Into::into)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum UserEvent {
    WakeUp,
}

/// Application state and event handling.
struct Application {
    /// Custom cursors assets.
    /// With OpenGL it could be EGLDisplay.
    context: Option<Context<DisplayHandle<'static>>>,
}

impl Application {
    fn new<T>(event_loop: &EventLoop<T>) -> Self {
        // SAFETY: we drop the context right before the event loop is stopped, thus making it safe.
        let context = Some(
            Context::new(unsafe {
                std::mem::transmute::<DisplayHandle<'_>, DisplayHandle<'static>>(
                    event_loop.display_handle().unwrap(),
                )
            })
            .unwrap(),
        );
                Self {
            context,
         }
    }
    fn dump_monitors(&self, event_loop: &ActiveEventLoop) {
//        println!("Monitors information");
        let primary_monitor = event_loop.primary_monitor();
        for monitor in event_loop.available_monitors() {
            let intro = if primary_monitor.as_ref() == Some(&monitor) {
                "Primary monitor"
            } else {
                "Monitor"
            };

            if let Some(name) = monitor.name() {
               println!("{intro}: {name}");
            } else {
               println!("{intro}: [no name]");
            }

            let PhysicalSize { width, height } = monitor.size();
            println!("Current mode: +{width}x{height}+");
//                "  Current mode: {width}x{height}{}",
//                if let Some(m_hz) = monitor.refresh_rate_millihertz() {
//                    format!(" @ {}.{} Hz", m_hz / 1000, m_hz % 1000)
//                } else {
//                    String::new()
//                }
//            );

            let PhysicalPosition { x, y } = monitor.position();
            println!("  Position: {x},{y}");

//            println!("  Scale factor: {}", monitor.scale_factor());

//            println!("  Available modes (width x height x bit-depth):");
//            for mode in monitor.video_modes() {
//                let PhysicalSize { width, height } = mode.size();
//                let bits = mode.bit_depth();
//                let m_hz = mode.refresh_rate_millihertz();
//                println!("    {width}x{height}x{bits} @ {}.{} Hz", m_hz / 1000, m_hz % 1000);
//           }
        }
    }

        // You'll have to choose an icon size at your own discretion. On X11, the desired size
        // varies by WM, and on Windows, you still have to account for screen scaling. Here
        // we use 32px, since it seems to work well enough in most cases. Be careful about
        // going too high, or you'll be bitten by the low-quality downscaling built into the
        // WM.

   
}
impl ApplicationHandler<UserEvent> for Application {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        println!("User event: {event:?}");
        _event_loop.exit();
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
//        println!("Device {device_id:?} event: {event:?}");
    }
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
//        println!("Window {window_id:?} event: {event:?}");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//        println!("Resumed the event loop");
        self.dump_monitors(event_loop);
        event_loop.exit();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // We must drop the context here.
        self.context = None;
    }
}


