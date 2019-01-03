extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use std::f64::consts::PI;


pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    rotation: f64,
    rotation_target: f64,
    // Rotation for the square.
    position: f64,
    growing: bool,
    movement: f64,
    up_pressed: bool,
    down_pressed: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

//        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

//        let square = rectangle::square(0.0, 0.0, 50.0);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, 50.0, 25.0);
        let rotation = self.rotation;
        let position = self.position;
        let movement = self.movement;
//        let (x, y) = ((args.width / 2) as f64,
//                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c.transform.trans(movement, movement).scale(position, -position).rot_rad(rotation + 0.5 * PI).trans(-25.0, -12.5);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        let mut direction: f64 = (self.rotation_target - self.rotation).signum();
        if (self.rotation_target - self.rotation).abs() > PI {
            direction *= -1.0;
        }
        self.rotation += 2.0 * direction * args.dt;

        if self.rotation > PI {
            self.rotation -= 2.0 * PI;
        } else if self.rotation < -PI {
            self.rotation += 2.0 * PI;
        }

        if (self.position < 0.8) & (self.growing == true) {
            self.position += 2.0 * args.dt;
        } else if self.position < 0.4 {
            self.growing = true;
        } else {
            self.growing = false;
            self.position -= 2.0 * args.dt;
        }
        if self.up_pressed == true {
            self.movement -= 20.0 * args.dt;
        }
        if self.down_pressed == true {
            self.movement += 20.0 * args.dt;
        }
    }

    fn handle_press(&mut self, args: &ButtonArgs) {
        match args.state {
            ButtonState::Press => {
                match args.button {
                    Button::Keyboard(Key::Up) => {
                        self.up_pressed = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_pressed = true;
                    }
                    _ => {}
                }
            }
            ButtonState::Release => {
                match args.button {
                    Button::Keyboard(Key::Up) => {
                        self.up_pressed = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_pressed = false;
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_cursor_input(&mut self, position: &[f64; 2]) {
        let x: f64 = &position[0] - &self.movement;
        let y: f64 = &position[1] - &self.movement;
        self.rotation_target =x.atan2(y);

        //println!("x: {}, y: {}, rot_tar: {}, rot: {}", x, y, self.rotation_target, self.rotation);
    }
}

fn main() {
// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

// Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

// Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation_target: 0.0,
        rotation: 0.0,
        position: 0.0,
        growing: true,
        movement: 100.0,
        up_pressed: false,
        down_pressed: false,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.button_args() {
            app.handle_press(&p);
        }

        if let Some(m) = e.mouse_cursor_args() {
            app.handle_cursor_input(&m);
        }
    }
}



