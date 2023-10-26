use goodman::prelude::*;

const WINDOW_SIZE: Vec32 = vec2(700., 700.);
const SCREEN_SIZE: Vec32 = vec2(700., 700.);
const PART_AMT: (usize, usize) = (100, 100);
const PART_SIZE: Vec32 = vec2(
    SCREEN_SIZE.x / PART_AMT.0 as f32,
    SCREEN_SIZE.y / PART_AMT.1 as f32,
);

fn main() {
    block_on(run())
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine = EngineBuilder::new(WINDOW_SIZE)
        .show_engine_ui()
        .with_target_fps(144)
        // .with_target_tps(10)
        .build(&event_loop)
        .await;

    let simulation = Simulation::new(&mut engine);
    engine.start_loop(simulation, event_loop)
}

struct Simulation {
    particles: Vec<Vec<Particle>>,
    textures: Vec<Texture>,
}
impl Manager for Simulation {
    fn new(engine: &mut Engine) -> Self {
        let mut textures = vec![];
        create_textures!(engine, textures, "assets/sand.png" "assets/water.png");

        let mut particles = vec![];
        for _ in 0..PART_AMT.1 {
            let mut row = vec![];
            for _ in 0..PART_AMT.0 {
                row.push(Particle::new(PartKind::Empty));
            }
            particles.push(row);
        }

        Self {
            particles,
            textures,
        }
    }
    fn update(&mut self, _frame_time: f64, input: &Input, _sound: &mut Sound) {
        if input.is_button_held(Button::LeftMouse) {
            let i = (input.get_cursor_pos().x / PART_SIZE.x as f64) as usize;
            let j = (input.get_cursor_pos().y / PART_SIZE.y as f64) as usize;
            self.particles[j][i] = Particle::new(PartKind::Sand);
        }

        for j in 0..self.particles.len() {
            for i in 0..self.particles[j].len() {
                if self.particles[j][i].kind == PartKind::Empty || self.particles[j][i].has_updated
                {
                    continue;
                }

                macro_rules! m {
                    ($parts: expr, $j: expr, $i: expr, $($j_add: expr, $i_add: expr)*) => {
                        $(
                            if $j as isize + $j_add < 0 || $i as isize + $i_add < 0 {
                                continue;
                            }
                            let (new_j, new_i) = (($j as isize + $j_add) as usize, ($i as isize + $i_add) as usize);

                            if $parts.get(new_j).is_some() && $parts.get(new_i).is_some() && $parts[new_j][new_i].kind == PartKind::Empty {
                                self.particles[new_j][new_i] = self.particles[$j][$i];
                                self.particles[$j][$i] = Particle::new(PartKind::Empty);
                                self.particles[new_j][new_i].has_updated = true;
                                continue;
                            }
                        )*
                    };
                }
                m!(self.particles, j, i,    1, 0   1, -1   1, 1);

                
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

#[derive(Debug, Clone, Copy)]
struct Particle {
    kind: PartKind,
    has_updated: bool,
}
impl Particle {
    fn new(kind: PartKind) -> Self {
        Self {
            kind,
            has_updated: false,
        }
    }

    pub fn get_index(&self) -> usize {
        match self.kind {
            PartKind::Empty => panic!("can't render empty particle"),
            PartKind::Sand => 0,
            // ParticleKind::Water => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PartKind {
    Empty,
    Sand,
    // Water
}
