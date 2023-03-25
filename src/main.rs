//use goodman::run;

use cgmath::{vec2, Vector2};
use goodman::{
    instances::{CircleInstance, CircleInstanceT, SquareInstance, SquareInstanceT},
    object_data::VERTEX_SCALE,
    state_manager::{self, Input},
    Manager, State,
};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    pollster::block_on(run());
}

type Vec2 = Vector2<f64>;

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f64,
}
impl Ball {
    fn new() -> Self {
        Self {
            pos: vec2(0., 0.),
            vel: vec2(0.003, 0.0017),
            radius: 1.,
        }
    }
    fn update(&mut self, paddle_0: &Paddle, paddle_1: &Paddle) {
        let radius_scaled = self.radius * (VERTEX_SCALE as f64);

        let new_pos = vec2(self.pos.x + self.vel.x, self.pos.y + self.vel.y);
        if new_pos.x + radius_scaled > 1. || new_pos.x - radius_scaled < -1. {
            *self = Ball::new();
        }

        if new_pos.y + radius_scaled > 1. {
            self.vel.y *= -1.;
            self.pos.y = 1. - radius_scaled;
        }
        if new_pos.y - radius_scaled < -1. {
            self.vel.y *= -1.;
            self.pos.y = -1. + radius_scaled;
        }

        let size_scaled_x = paddle_0.size.x * VERTEX_SCALE as f64 * 0.5 + 0.02;
        let size_scaled_y = paddle_0.size.y * VERTEX_SCALE as f64 * 0.5 + 0.02;

        if (new_pos.x + radius_scaled > paddle_1.pos.x - size_scaled_x
            && new_pos.y + radius_scaled > paddle_1.pos.y - size_scaled_y
            && new_pos.y - radius_scaled < paddle_1.pos.y + size_scaled_y
            && self.vel.x > 0.)
            || (new_pos.x - radius_scaled < paddle_0.pos.x + size_scaled_x
                && new_pos.y + radius_scaled > paddle_0.pos.y - size_scaled_y
                && new_pos.y - radius_scaled < paddle_0.pos.y + size_scaled_y
                && self.vel.x < 0.)
        {
            self.vel.x *= -1.;
        }

        self.pos += self.vel;
    }
}
impl CircleInstanceT for Ball {
    fn to_circle_instance(&self) -> CircleInstance {
        CircleInstance::new(self.pos, self.radius)
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    pos: Vec2,
    size: Vec2,
}
impl Paddle {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            size: PADDLE_SIZE,
        }
    }
    fn update(&mut self, up_pressed: bool, down_pressed: bool) {
        let size_scaled_y = self.size.y * VERTEX_SCALE as f64 * 0.5 + PADDLE_SPEED + 0.07;
        if up_pressed && self.pos.y + size_scaled_y < 1. {
            self.pos.y += PADDLE_SPEED;
        }
        if down_pressed && self.pos.y - size_scaled_y > -1. {
            self.pos.y -= PADDLE_SPEED;
        }
    }
}
impl SquareInstanceT for Paddle {
    fn to_square_instance(&self) -> SquareInstance {
        SquareInstance::new(self.pos, PADDLE_SIZE)
    }
}

const PADDLE_SPEED: f64 = 0.005;
const PADDLE_SIZE: Vec2 = vec2(1., 3.);

pub struct Pong {
    paddle_0: Paddle,
    paddle_1: Paddle,
    ball: Ball,
}
impl Manager for Pong {
    fn new(state: &mut State) -> Self {
        let paddle_0 = Paddle::new(vec2(-0.8, 0.));
        let paddle_1 = Paddle::new(vec2(0.8, 0.));
        let ball = Ball::new();

        state.square_instances = vec![paddle_0.to_square_instance(), paddle_1.to_square_instance()];
        state.circle_instances = vec![ball.to_circle_instance()];

        state.update_square_instances();
        state.update_circle_instances();

        Self {
            paddle_0,
            paddle_1,
            ball,
        }
    }

    fn update(&mut self, state: &mut State) {
        let paddle_0 = &mut self.paddle_0;
        let paddle_1 = &mut self.paddle_1;
        paddle_0.update(state.input.w_pressed, state.input.s_pressed);
        paddle_1.update(state.input.up_pressed, state.input.down_pressed);
        self.ball.update(paddle_0, paddle_1);

        state.square_instances[0] = paddle_0.to_square_instance();
        state.square_instances[1] = paddle_1.to_square_instance();
        state.update_square_instances();

        state.circle_instances[0] = self.ball.to_circle_instance();
        state.update_circle_instances();
    }

    fn render(&self, state: &State) -> Result<(), wgpu::SurfaceError> {
        state.render()
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(700., 700.))
        .build(&event_loop)
        .expect("Failed to build window");

    let mut state = State::new(window).await;
    let mut pong = Pong::new(&mut state);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                pong.update(&mut state);
                match pong.render(&state) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
