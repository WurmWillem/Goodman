use goodman::prelude::*;

fn main() {
    block_on(run());
}

const WINDOW_SIZE: Vec64 = vec2(1200., 800.);
const BACKGROUND_COLOR: Color = Color::new(105., 105., 105., 0.);

async fn run() {
    let event_loop = EventLoop::new();

    let window_size = vec2(WINDOW_SIZE.x as f32, WINDOW_SIZE.y as f32);
    let mut engine = EngineBuilder::new(window_size)
        .with_background_color(BACKGROUND_COLOR)
        .show_engine_ui()
        .with_target_fps(144)
        // .set_target_tps(100 * 1000)
        .build(&event_loop)
        .await;

    let physics = Physics::new(&mut engine);

    engine.start_loop(physics, event_loop);
}

struct Physics {
    circles: Vec<Circle>,
    textures: Vec<Texture>,
}
impl Manager for Physics {
    fn new(engine: &mut Engine) -> Self {
        let mut textures = vec![];
        create_textures!(engine, textures, "assets/circleb.png");

        let circle_0 = Circle::new(32., vec2(100., 100.));
        let circle_1 = Circle::new(64., vec2(500., 500.));

        Physics {
            circles: vec![circle_0, circle_1],
            textures,
        }
    }
    fn update(&mut self, delta_t: f64, _input: &Input, _sound: &Sound) {
        self.circles.iter_mut().for_each(|circle| {
            circle.update(delta_t);
        });

        self.resolve_collisions();
    }
    fn render(&mut self, engine: &mut Engine) {
        /*let mut ui = UserUi::new("Physics Engine");
        ui.add_label(format!(
            "circle position: {} {}",
            self.circle.pos.x as u32, self.circle.pos.y as u32
        ));
        engine.set_user_ui(ui);*/

        self.circles.iter().for_each(|circle| {
            engine.render_texture(circle.to_rect(), &self.textures[0]);
        });
    }
}
impl Physics {
    fn resolve_collisions(&mut self) {
        for j in 0..self.circles.len() {
            for i in j + 1..self.circles.len() {
                let dist_x = self.circles[j].pos.x - self.circles[i].pos.x;
                let dist_y = self.circles[j].pos.y - self.circles[i].pos.y;

                let dist = dist_x.powi(2) + dist_y.powi(2);
                if dist > (self.circles[j].radius + self.circles[i].radius).powi(2) * 1.1 {
                    return;
                }
                println!("d");

                /*let middle_x = (self.circles[j].pos.x + self.circles[i].pos.x) * 0.5;
                let middle_y = (self.circles[i].pos.y + self.circles[i].pos.y) * 0.5;

                self.circles[j].pos.x = middle_x + self.circles[j].radius * dist_x / dist;
                self.circles[j].pos.y = middle_y + self.circles[j].radius * dist_y / dist;

                self.circles[i].pos.x = middle_x + self.circles[i].radius * -dist_x / dist;
                self.circles[i].pos.y = middle_y + self.circles[i].radius * -dist_y / dist;

                self.circles[j].vel *= -1.;
                self.circles[i].vel *= -1.;*/

                let normal = (self.circles[i].pos - self.circles[j].pos).normalize();

                let relative_vel = self.circles[i].vel - self.circles[j].vel;
                // Calculate relative velocity in terms of the normal direction
                let vel_along_normal = normal.dot(relative_vel);
                if vel_along_normal > 0. {
                    continue; // Only resolve collision if objects are moving towards each other
                }

                let inv_mass_0 = 1. / self.circles[j].mass;
                let inv_mass_1 = 1. / self.circles[i].mass;

                let mut impulse_scalar = -2. * vel_along_normal;
                impulse_scalar /= inv_mass_0 + inv_mass_1;

                // Calculate impulse, clamp the impulse so the simulation won't explode because of extreme velocities
                let impulse = impulse_scalar * normal;
                if impulse.magnitude() > 10000. {
                    dbg!("Dsfs");
                }

                // Calculate new velocity based on impulse
                self.circles[j].vel -= inv_mass_0 * impulse;
                self.circles[i].vel += inv_mass_1 * impulse;
                // let new_vel_0 = self.circles[j].vel - inv_mass_0 * impulse;
                // let new_vel_1 = self.circles[i].vel + inv_mass_1 * impulse;
            }
        }
    }
}

struct Circle {
    radius: f64,
    pos: Vec64,
    vel: Vec64,
    mass: f64,
}
impl Circle {
    fn new(radius: f64, pos: Vec64) -> Circle {
        Circle {
            radius,
            pos,
            vel: vec2(500., -500.),
            mass: 10.,
        }
    }

    fn update(&mut self, delta_t: f64) {
        let mut f_res = vec2(0., 0.);
        // f_res.y -= self.mass * 9.81 * 3.;

        /*if f_res == vec2(0., 0.) {
            return;
        }*/
        let acc = f_res / self.mass;

        self.vel += acc * delta_t;

        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y > WINDOW_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = WINDOW_SIZE.y;
        } else if next_pos.y - self.radius * 2. < 0. {
            self.vel.y *= -1.;
            self.pos.y = self.radius * 2.;
        } else if next_pos.x + self.radius * 2. > WINDOW_SIZE.x {
            self.vel.x *= -1.;
            self.pos.x = WINDOW_SIZE.x - self.radius * 2.;
        } else if next_pos.x < 0. {
            self.vel.x *= -1.;
            self.pos.x = 0.;
        }

        self.pos += self.vel * delta_t;
    }

    fn to_rect(&self) -> Rect32 {
        rect64(
            self.pos.x,
            WINDOW_SIZE.y - self.pos.y,
            self.radius * 2.,
            self.radius * 2.,
        )
        .into()
    }
}
