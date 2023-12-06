// example of a particle simulator
// use left, right and middle mouse to place particles
// scroll to make the area you place pixels in bigger or smaller

use goodman::prelude::*;
use particle::Particle;

use crate::particle::PartKind;

mod particle;

const WINDOW_SIZE: Vec32 = vec2(1200., 900.);
const PART_AMT: (usize, usize) = (300, 225);
const PART_SIZE: Vec32 = vec2(
    WINDOW_SIZE.x / PART_AMT.0 as f32,
    WINDOW_SIZE.y / PART_AMT.1 as f32,
);

const DISPERSION: isize = 5;

fn main() {
    block_on(run())
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui()
        .with_target_fps(144)
        .build(&event_loop)
        .await;

    let simulation = Simulation::new(&mut engine);
    engine.start_loop(simulation, event_loop)
}

struct Simulation {
    particles: Vec<Vec<Particle>>,
    textures: Vec<Texture>,
    circle_size: usize,
}
impl Manager for Simulation {
    fn new(engine: &mut Engine) -> Self {
        let mut textures = vec![];
        create_textures!(engine, textures, "assets/sand.png" "assets/water.png" "assets/wood.png");

        Self {
            particles: create_empty_part_vec(),
            textures,
            circle_size: 10,
        }
    }

    fn update(&mut self, _frame_time: f64, input: &Input, _sound: &mut Sound) {
        // input.get_wheel_movement() is 1 if scrolled up, -1 if scrolled down and 0 if not being scrolled
        if self.circle_size as i16 + input.get_wheel_movement() as i16 > 0 {
            self.circle_size =
                (self.circle_size as i16 + input.get_wheel_movement() as i16) as usize;
        }

        if input.is_button_held(Button::LeftMouse) {
            self.place_part_circle(input, self.circle_size, PartKind::Sand);
        }
        if input.is_button_held(Button::RightMouse) {
            self.place_part_circle(input, self.circle_size, PartKind::Water);
        }
        if input.is_button_held(Button::MiddleMouse) {
            self.place_part_line(input);
        }

        if input.is_button_pressed(Button::R) {
            self.particles = create_empty_part_vec();
        }

        let down_to_up = rand::random();
        for _j in 0..self.particles.len() {
            let mut y = _j;
            if down_to_up {
                y = self.particles.len() - 1 - y;
            }

            let right_to_left = rand::random();
            for _i in 0..self.particles[y].len() {
                let mut x = _i;
                if right_to_left {
                    x = self.particles[y].len() - 1 - x;
                }

                if self.particles[y][x].kind == PartKind::Empty
                    || self.particles[y][x].kind == PartKind::Wood
                    || self.particles[y][x].has_updated
                {
                    continue;
                }

                self.particles[y][x].update();

                match self.particles[y][x].kind {
                    PartKind::Empty | PartKind::Wood => panic!("can't update this particle"),
                    PartKind::Sand => {
                        let c = self.particles[y][x].vel.y as isize;
                        self.update_particle(x, y, vec![(0, c), (-1, 1), (1, 1)]);
                    }
                    PartKind::Water => {
                        let c = self.particles[y][x].vel.y as isize;
                        let moves =
                            vec![(0, c), (-1, 1), (1, 1), (-DISPERSION, 0), (DISPERSION, 0)];
                        self.update_particle(x, y, moves);
                    }
                }
            }
        }

        for j in 0..self.particles.len() {
            for i in 0..self.particles[j].len() {
                self.particles[j][i].has_updated = false;
            }
        }
    }
    fn render(&mut self, engine: &mut Engine) {
        for j in 0..self.particles.len() {
            for i in 0..self.particles[j].len() {
                if self.particles[j][i].kind == PartKind::Empty {
                    continue;
                }

                let index = self.particles[j][i].get_index();
                let (i, j) = (i as f32, j as f32);
                let rect = rect32(i * PART_SIZE.x, j * PART_SIZE.y, PART_SIZE.x, PART_SIZE.y);
                engine.render_texture(rect, &self.textures[index])
            }
        }
    }
}
impl Simulation {
    fn update_particle(&mut self, i: usize, j: usize, mut moves: Vec<(isize, isize)>) {
        for m in &mut moves {
            if rand::random() {
                (*m).0 *= -1;
            }
            let mut k: isize = 0;
            if m.1 > 0 {
                k = 1
            } else if m.1 < 0 {
                k = -1
            };
            let add = k;

            /*if m.1 == 0 {
                // pr(m.0);
                continue;
            }*/

            let mut should_return = false;
            while k.abs() <= m.1.abs() {
                let (is_safe, new_i, new_j) = self.get_new_pos(i, j, m.0, k);
                k += add;

                if is_safe && self.particles[new_j][new_i].kind == PartKind::Empty {
                    let prev_j = (new_j as isize - add) as usize;

                    self.particles[new_j][new_i] = self.particles[prev_j][i];
                    self.particles[new_j][new_i].has_updated = true;
                    self.particles[prev_j][i] = Particle::new(PartKind::Empty);

                    should_return = true;
                } else {
                    break;
                }
            }
            if should_return {
                return;
            }
        }
        for m in &moves {
            if m.0 == 0 || m.1 != 0 {
                continue;
            }

            let mut k: isize = 0;
            if m.0 > 0 {
                k = 1
            } else if m.0 < 0 {
                k = -1
            };
            let add = k; // k =
            while k.abs() <= m.0.abs() {
                let (is_safe, new_i, new_j) = self.get_new_pos(i, j, k, m.1);
                k += add;

                if is_safe && self.particles[new_j][new_i].kind == PartKind::Empty {
                    let prev_i = (new_i as isize - add) as usize;

                    self.particles[new_j][new_i] = self.particles[j][prev_i];
                    self.particles[new_j][new_i].has_updated = true;
                    self.particles[j][prev_i] = Particle::new(PartKind::Empty);
                } else {
                    return;
                }
            }
        }
    }

    fn get_new_pos(&self, i: usize, j: usize, i_add: isize, j_add: isize) -> (bool, usize, usize) {
        let (new_j, new_i) = (j as isize + j_add, i as isize + i_add);
        let (u_i, u_j) = (new_i as usize, new_j as usize);
        let b = new_j >= 0
            && new_i >= 0
            && self.particles.get(u_j).is_some()
            && self.particles[u_j].get(u_i).is_some();

        (b, u_i, u_j)
    }

    fn place_part_line(&mut self, input: &Input) {
        let i = (input.get_cursor_pos().x / PART_SIZE.x as f64).floor() as usize;
        let j = (input.get_cursor_pos().y / PART_SIZE.y as f64).floor() as usize;

        for c in 0..10 {
            let new_i = i + c - 5;
            for d in 0..10 {
                let j = j + d;
                if self.particles.get(j).is_some()
                    && self.particles[j].get(new_i).is_some()
                    && self.particles[j][new_i].kind == PartKind::Empty
                {
                    self.particles[j][new_i] = Particle::new(PartKind::Wood);
                }
            }
        }
    }

    fn place_part_circle(&mut self, input: &Input, size: usize, part_kind: PartKind) {
        use std::f32::consts::PI;

        let mut x_vec = vec![];
        let d = 2. * PI / size as f32;
        for i in 0..size {
            let x = (i as f32 * d).cos() * size as f32;
            x_vec.push(x as isize);
        }

        let mut vec = vec![];
        for i in 0..size {
            let y = (i as f32 * d).sin() * size as f32;
            vec.push((x_vec[i], y as isize))
        }

        let i = (input.get_cursor_pos().x / PART_SIZE.x as f64).floor() as usize;
        let j = (input.get_cursor_pos().y / PART_SIZE.y as f64).floor() as usize;
        // self.particles[j][i] = Particle::new(part_kind);
        // return;

        for v in &vec {
            let (new_j, new_i) = ((j as isize + v.0) as usize, (i as isize + v.1) as usize);
            if j as isize + v.0 >= 0
                && i as isize + v.1 >= 0
                && self.particles.get(new_j).is_some()
                && self.particles[new_j].get(new_i).is_some()
                && self.particles[new_j][new_i].kind == PartKind::Empty
            {
                self.particles[new_j][new_i] = Particle::new(part_kind);
            }
        }
    }
}

fn create_empty_part_vec() -> Vec<Vec<Particle>> {
    let mut particles = vec![];
    for _ in 0..PART_AMT.1 {
        let mut row = vec![];
        for _ in 0..PART_AMT.0 {
            row.push(Particle::new(PartKind::Empty));
        }
        particles.push(row);
    }
    particles
}

#[allow(unused)]
fn pr<T: std::fmt::Display + std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}
