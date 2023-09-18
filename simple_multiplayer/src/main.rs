#[macro_use] extern crate conrod;

use conrod::backend::glium::glium::{self, Surface};
use conrod::position::Dimension;
use conrod::widget::Rectangle;
use conrod::{widget, Positionable, Colorable, Widget, Sizeable, Dimensions, Color, Borderable};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;



fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
                    .with_title("Hello Conrod");
    let context = glium::glutin::ContextBuilder::new()
                    .with_vsync(true)
                    .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    widget_ids!(struct Ids { Canvas_, rects[] });
    let mut ids = Ids::new(ui.widget_id_generator());

    ids.rects.resize(1, &mut ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut pos_x = 0.0;
    let mut pos_y = 5.0;    

    'main: loop {

        // "Hello World!" in the middle of the screen.
        // widget::Canvas::new()
        //     .top_left()
        //     .color(conrod::color::WHITE).x_dimension(Dimension::Of(ui.window, None)).y_dimension(Dimension::Of(ui.window, None))
        //     .set(ids.Canvas_, &mut ui.set_widgets());
        widget::BorderedRectangle::new([10.0, 10.0]).border(0.0).x_y(pos_x, pos_y).color(Color::Rgba((255.0), (0.0), (0.0), (1.0))).set(ids.rects[0], &mut ui.set_widgets());
        
        

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 0.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }





        
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));
        
        for event in events{
            // if let Some(event) = conrod::backend::winit::convert_event(
            //     event.clone(),
            //     &display
            // ) {
            //     ui.handle_event(event);
            // }
            match event {
                glium::glutin::Event::WindowEvent { event, ..} => match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input : glium::glutin::KeyboardInput{
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::W),
                            ..
                        },
                        ..
                    } => pos_y += 1.0,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input : glium::glutin::KeyboardInput{
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::S),
                            ..
                        },
                        ..
                    } => pos_y -= 1.0,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input : glium::glutin::KeyboardInput{
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::A),
                            ..
                        },
                        ..
                    } => pos_x -= 1.0,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input : glium::glutin::KeyboardInput{
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::D),
                            ..
                        },
                        ..
                    } => pos_x += 1.0,
                    _ => (),
                },


                _ => (),
            }
        }

    }
}