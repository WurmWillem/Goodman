use std::vec;

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

const DISPERSION: isize = 5;

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

        for j in 0..self.particles.len() {
            let right_to_left = rand::random();

            'outer: for i in 0..self.particles[j].len() {
                let mut rand_i = i;
                if right_to_left {
                    rand_i = self.particles[j].len() - 1 - i;
                }

                if self.particles[j][rand_i].kind == PartKind::Empty
                    || self.particles[j][rand_i].has_updated
                {
                    continue;
                }

                self.particles[j][rand_i].update();

                macro_rules! update_particle {
                    ($parts: expr, $($j_add: expr, $i_add: expr)*) => {
                        $(
                            let (i_add, j_add): (isize, isize) = if rand::random() {
                                ($i_add, $j_add)
                            } else {($i_add * -1, $j_add)};

                            let mut k: isize;
                            let add = if j_add == 0 {k = 0; 1} else if j_add > 0 {k = 1; 1} else {k = -1; -1};

                            while k.abs() <= j_add.abs() {
                                if j as isize + k < 0 || rand_i as isize + i_add < 0 {
                                    break;
                                }
                                let (new_j, new_i) = ((j as isize + k) as usize, (rand_i as isize + i_add) as usize);

                                if $parts.get(new_j).is_some() && $parts[new_j].get(new_i).is_some() && $parts[new_j][new_i].kind == PartKind::Empty {
                                    $parts[new_j][new_i] = $parts[j][rand_i];
                                    $parts[j][rand_i] = Particle::new(PartKind::Empty);
                                    $parts[new_j][new_i].has_updated = true;
                                    continue 'outer
                                }
                                k += add;
                            }

                            let mut k: isize;
                            let add = if i_add == 0 {k = 0; 1} else if i_add > 0 {k = 1; 1} else {k = -1; -1};

                            while k.abs() <= i_add.abs() {
                                if j as isize + j_add < 0 || rand_i as isize + k < 0 {
                                    break;
                                }
                                let (new_j, new_i) = ((j as isize + j_add) as usize, (rand_i as isize + k) as usize);

                                if $parts.get(new_j).is_some() && $parts[new_j].get(new_i).is_some() && $parts[new_j][new_i].kind == PartKind::Empty {
                                    $parts[new_j][new_i] = $parts[j][rand_i];
                                    $parts[j][rand_i] = Particle::new(PartKind::Empty);
                                    $parts[new_j][new_i].has_updated = true;
                                    continue 'outer
                                } else if i_add == 0 {
                                    $parts[j][rand_i].vel.y = 1.;
                                }
                                k += add;
                            }
                        )*
                    };
                }

                match self.particles[j][rand_i].kind {
                    PartKind::Empty => panic!("can't update empty particle"),
                    PartKind::Sand => {
                        let j = self.particles[j][rand_i].vel.y as isize;
                        update_particle!(self.particles, j,0  j,-1  j,1);
                    }
                    PartKind::Water => {
                        let j = 1;
                        update_particle!(self.particles,  j,0  j,-1  j,1  0,-DISPERSION  0,DISPERSION);
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
