use goodman::prelude::*;

fn main() {
    block_on(run())
}

const SIZE_MULT: f32 = 4.;

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(vec2(256. * SIZE_MULT, 224. * SIZE_MULT))
        .use_near_filter_mode()
        // .show_engine_ui()
        .with_target_fps(60)
        .build(&event_loop)
        .await;

    let game = Game::new(&mut engine);
    engine.start_loop(game, event_loop)
}

struct Game {
    textures: Vec<Texture>,
    mario: Mario,
    grid: Vec<Vec<Object>>,
}
impl Manager for Game {
    fn new(engine: &mut Engine) -> Self {
        let mut textures = vec![];
        create_textures!(engine, textures, "mario.png" "1.png");

        let mut grid = vec![vec![Object::new(Kind::Empty); 16]; 14];

        for j in 0..2 {
            for i in 0..16 {
                grid[j + 12][i] = Object::new(Kind::Block)
            }
        }

        Self {
            textures,
            grid,
            mario: Mario::new(),
        }
    }
    fn update(&mut self, delta_t: f64, input: &Input, sound: &mut Sound) {
        self.mario.update(input);
    }
    fn render(&mut self, engine: &mut Engine) {
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                let (x, y) = match self.grid[j][i].kind {
                    Kind::Empty => continue,
                    Kind::Block => (0., 208.),
                };
                let params = DrawParams {
                    source: Some(rect32(x, y, 16., 16.)),
                    ..Default::default()
                };

                let rect = rect32(
                    i as f32 * 16. * SIZE_MULT,
                    j as f32 * 16. * SIZE_MULT,
                    16. * SIZE_MULT,
                    16. * SIZE_MULT,
                );

                engine.render_texture_ex(rect, &self.textures[1], params)
            }
        }

        let rect = rect32_vec(self.mario.pos * SIZE_MULT, vec2(12. * SIZE_MULT, 16. * SIZE_MULT));

        engine.render_texture(rect, &self.textures[0]);
    }
}

#[derive(Debug, Clone, Copy)]
struct Object {
    kind: Kind,
}
impl Object {
    fn new(kind: Kind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Empty,
    Block,
}

#[derive(Debug, Clone, Copy)]
struct Mario {
    pos: Vec32,
    vel: Vec32,
}
impl Mario {
    fn new() -> Self {
        Self {
            pos: vec2(0., 0.),
            vel: vec2(0., 0.),
        }
    }
    fn update(&mut self, input: &Input) {
        // self.vel.y -= 4.;

        if input.is_button_held(Button::D) && input.is_button_held(Button::LShift) {
            self.vel.x += to_vel(0.1);
            
            if self.vel.x > to_vel(2900.) {
                self.vel.x = to_vel(2900.);
            }
        }
        else if input.is_button_held(Button::A) && input.is_button_held(Button::LShift) {
            self.vel.x -= to_vel(0.1);
            
            if self.vel.x < -to_vel(2900.) {
                self.vel.x = -to_vel(2900.);
            }
        }
        else if input.is_button_held(Button::D) {
            let min_x = to_vel(0.130);
            if self.vel.x < min_x {
                self.vel.x = min_x;
            } else {
                self.vel.x += to_vel(0.098);
            }
            if self.vel.x > to_vel(1900.) {
                self.vel.x = to_vel(1900.);
            }
        }
        else if input.is_button_held(Button::A) {
            let min_x = -to_vel(0.130);
            if self.vel.x > min_x {
                self.vel.x = min_x;
            } else {
                self.vel.x -= to_vel(0.098);
                // println!("{}", self.vel.x);
            }
            if self.vel.x < -to_vel(1900.) {
                self.vel.x = -to_vel(1900.);
            }
        } else {
            if self.vel.x > 0. {
                if self.vel.x < 0.0546875 {
                    self.vel.x = 0.;
                } else {
                    self.vel.x -= 0.0546875;
                }
            }
            if self.vel.x < 0. {
                if self.vel.x > -0.0546875 {
                    self.vel.x = 0.;
                } else {
                    self.vel.x += 0.0546875;
                }
            }
        }

        self.pos += self.vel;
    }
}

fn to_vel(x: f32) -> f32 {
    // println!("{x}");

    let mut digits = vec![];
    for d in x.to_string().chars() {
        let digit = match d.to_digit(10) {
            Some(u) => u,
            _ => continue,
        };
        digits.push(digit);
    }

    let mut total = 0.;
    for i in 0..digits.len() {
        if digits[i] == 0 {
            continue;
        }
        total += digits[i] as f32 / 16_i32.pow(i as u32 + 0) as f32;
    }
    // println!("{:?}", total);

    total
}
