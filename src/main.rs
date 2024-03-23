mod graphics;

use std::time::Instant;

use glium::Surface;
use overlaylib::{
    primitives::{text, Circle, Outline, Rectangle, Text},
    texture::Texture2D,
    Overlay,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    platform::x11::{WindowBuilderExtX11, XWindowType},
    window::WindowBuilder,
};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(2560, 1600))
        .with_transparent(true)
        .with_x11_window_type(vec![XWindowType::Dock])
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        .with_decorations(false);

    let (window, display) = graphics::build_display(window_builder, &event_loop);
    window.set_cursor_hittest(false).unwrap();

    let overlay = Overlay::initialize(&display).expect("overlay initialization");
    let mut position = (400.0, 400.0);
    let mut velocity = (3.0, 3.0);
    let mut hue = 0.0;
    let mut frames = vec![];

    let squirrel =
        Texture2D::load_from_memory(&display, include_bytes!("../squirrel.png")).unwrap();

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut frame = overlay.new_frame();
                    frames.push(Instant::now());
                    frames.retain(|frame| frame.elapsed().as_secs() < 1);

                    let text_size = text::calc_text_size(
                        "Hello, world",
                        overlay.current_font().expect("No font on the stack"),
                        24.0,
                    );
                    let window_size = display.get_framebuffer_dimensions();

                    if position.0 + text_size[0] > window_size.0 as f32 || position.0 < 0.0 {
                        velocity.0 = -velocity.0;
                    }
                    if position.1 + text_size[1] > window_size.1 as f32 || position.1 < 0.0 {
                        velocity.1 = -velocity.1;
                    }

                    position.0 += velocity.0;
                    position.1 += velocity.1;

                    let rgb = graphics::hsva_to_rgb([hue % 360.0, 0.7, 1.0, 1.0]);
                    hue += 3.0;

                    frame.add(
                        Text::new("Hello, world")
                            .centered(false)
                            .position([position.0, position.1])
                            .size(24.0)
                            .color(rgb),
                    );
                    frame.add(
                        Text::new(format!(
                            "FPS: {:.2}",
                            if frames.len() > 1 {
                                let elapsed = frames.first().unwrap().elapsed().as_secs_f32();
                                frames.len() as f32 / elapsed
                            } else {
                                0.0
                            }
                        ))
                        .centered(false)
                        .position([10.0, 50.0])
                        .size(24.0)
                        .color([1.0, 0.0, 0.0, 1.0]),
                    );
                    frame.add(
                        Rectangle::new()
                            .texture(&squirrel)
                            .dimensions([128.0 * 2.0, 96.0 * 2.0])
                            .position([200.0, 200.0])
                            .color([1.0, 1.0, 1.0, 1.0]),
                    );
                    frame.add(
                        Circle::new()
                            .border(Some(
                                Outline::new().thickness(1.0).color([1.0, 0.0, 0.0, 1.0]),
                            ))
                            .color([0.2, 0.2, 0.2, 0.2])
                            .position([
                                position.0 + text_size[0] / 2.0,
                                position.1 + text_size[1] / 2.0,
                            ])
                            .radius(text_size[0] / 2.0 + 5.0),
                    );

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 0.0);

                    overlay.draw(&display, &mut target, &mut frame).unwrap();
                    target.finish().unwrap();
                }
                WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .expect("event_loop.run()");
}
