use std::num::NonZeroU32;

use glium::Display;
use glutin::{
    config::GlConfig,
    context::NotCurrentGlContext,
    display::{GetGlDisplay, GlDisplay},
    surface::WindowSurface,
};
use winit::window::{Window, WindowBuilder};

// Slightly modified SimpleWindowBuilder
pub fn build_display<T>(
    window_builder: WindowBuilder,
    event_loop: &winit::event_loop::EventLoop<T>,
) -> (Window, Display<WindowSurface>) {
    use raw_window_handle::HasRawWindowHandle;
    let display_builder =
        glutin_winit::DisplayBuilder::new().with_window_builder(Some(window_builder));
    let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
    let (window, gl_config) = display_builder
        .build(&event_loop, config_template_builder, |configs| {
            configs
                .filter(|x| {
                    return x.supports_transparency().unwrap_or(false);
                })
                .next()
                .expect("no matching OpenGL configs found")
        })
        .unwrap();
    let window = window.unwrap();

    let (width, height): (u32, u32) = window.inner_size().into();
    let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .build(
            window.raw_window_handle(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };
    let context_attributes =
        glutin::context::ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let current_context = Some(unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .expect("failed to create context")
    })
    .unwrap()
    .make_current(&surface)
    .unwrap();
    let display = Display::from_context_surface(current_context, surface).unwrap();

    (window, display)
}

pub fn hsva_to_rgb(hsva: [f32; 4]) -> [f32; 4] {
    let h = hsva[0];
    let s = hsva[1];
    let v = hsva[2];
    let a = hsva[3];
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;
    let (r_prime, g_prime, b_prime) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        (r_prime + m),
        (g_prime + m),
        (b_prime + m),
        a,
    ]
}
