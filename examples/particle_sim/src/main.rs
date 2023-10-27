use goodman::prelude::*;
use particle::Particle;

use crate::particle::PartKind;

mod particle;

const WINDOW_SIZE: Vec32 = vec2(1100., 900.);
const SCREEN_SIZE: Vec32 = vec2(1100., 900.);
const PART_AMT: (usize, usize) = (300, 260);
const PART_SIZE: Vec32 = vec2(
    SCREEN_SIZE.x / PART_AMT.0 as f32,
    SCREEN_SIZE.y / PART_AMT.1 as f32,
);

const DISPERSION: isize = 1;

fn main() {
    block_on(run())
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui()
        .with_target_fps(144)
        .use_near_filter_mode()
        // .with_target_tps(10)
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
        create_textures!(engine, textures, "assets/sand.png" "assets/water.png");

        Self {
            particles: create_empty_part_vec(),
            textures,
            circle_size: 15,
        }
    }
    fn update(&mut self, _frame_time: f64, input: &Input, _sound: &mut Sound) {
        if self.circle_size as i16 + input.get_wheel_movement() as i16 > 0 {
            self.circle_size =
                (self.circle_size as i16 + input.get_wheel_movement() as i16) as usize;
        }

        if input.is_button_held(Button::LeftMouse) {
            self.place_particles(input, self.circle_size, PartKind::Sand);
        }
        if input.is_button_held(Button::RightMouse) {
            self.place_particles(input, self.circle_size, PartKind::Water);
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

                if self.particles[y][x].kind == PartKind::Empty || self.particles[y][x].has_updated
                {
                    continue;
                }

                self.particles[y][x].update();

                match self.particles[y][x].kind {
                    PartKind::Empty => panic!("can't update empty particle"),
                    PartKind::Sand => {
                        let c = self.particles[y][x].vel.y as isize;
                        self.update_particle(x, y, vec![(0, c), (-1, c), (1, c)]);
                    }
                    PartKind::Water => {
                        let c = 1;
                        let moves =
                            vec![(0, c), (-1, c), (1, c), (-DISPERSION, 0), (DISPERSION, 0)];
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
        // pr(j);
        for m in &mut moves {
            // println!("{:?}", m)
            if rand::random() {
                (*m).0 *= -1;
            }
            let mut k: isize = if m.1 == 0 {
                0
            } else if m.1 >= 0 {
                1
            } else {
                -1
            };
            let add = if k >= 0 { 1 } else { -1 };

            while k.abs() <= m.1.abs() {
                let (is_safe, new_i, new_j) = self.get_new_pos(i, j, m.0, k);
                k += add;

                if is_safe && self.particles[new_j][new_i].kind == PartKind::Empty {
                    let prev_j = (new_j as isize - add) as usize;

                    self.particles[new_j][new_i] = self.particles[prev_j][i];
                    self.particles[new_j][new_i].has_updated = true;
                    self.particles[prev_j][i] = Particle::new(PartKind::Empty);
                } else {
                    break;
                }
            }

            
        }
    }

    fn get_new_pos(&self, i: usize, j: usize, i_add: isize, j_add: isize) -> (bool, usize, usize) {
        let (new_j, new_i) = (j as isize + j_add, i as isize + i_add);
        let b = new_j >= 0
            && new_i >= 0
            && self.particles.get(new_j as usize).is_some()
            && self.particles[new_j as usize].get(new_i as usize).is_some();

        (b, new_i as usize, new_j as usize)
    }

    fn place_particles(&mut self, input: &Input, amt: usize, part_kind: PartKind) {
        use std::f32::consts::PI;

        let mut x_vec = vec![];
        let d = 2. * PI / amt as f32;
        for i in 0..amt {
            let x = (i as f32 * d).cos() * amt as f32;
            // pr(x);
            x_vec.push(x as isize);
        }

        let mut vec = vec![];
        for i in 0..amt {
            let y = (i as f32 * d).sin() * amt as f32;
            vec.push((x_vec[i], y as isize))
        }

        let i = (input.get_cursor_pos().x / PART_SIZE.x as f64) as usize;
        let j = (input.get_cursor_pos().y / PART_SIZE.y as f64) as usize;
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
