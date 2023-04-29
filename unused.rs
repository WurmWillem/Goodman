fn rotate() {
    for instance in &mut self.instances {
        instance.rotation += 0.1;
    }
    let instance_data = self
        .instances
        .iter()
        .map(Instance::to_raw)
        .collect::<Vec<_>>();
    self.queue.write_buffer(
        &self.instance_buffer,
        0,
        bytemuck::cast_slice(&instance_data),
    );
}

trait Run {
    fn go(&mut self, event_loop: &EventLoop<()>, state: &mut State);
}
impl Run for Manager {
    fn go(&mut self, event_loop: &EventLoop<()>, state: &mut State) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        // UPDATED!
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
                    state.update();
                    match state.render() {
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
}

pub fn create_instances() -> Vec<SquareInstance> {
    (0..INSTANCES_PER_ROW)
        .flat_map(|y| {
            (0..INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 {
                    x: x as f64 * VERTEX_SCALE as f64 * 2.3 - INSTANCE_DISPLACEMENT,
                    y: y as f64 * VERTEX_SCALE as f64 * 4.6 - INSTANCE_DISPLACEMENT,
                    z: 0.,
                };
                let rotation = 0.;
                let scale = vec2(1., 1.);
                SquareInstance {
                    pos: position,
                    rotation,
                    size: scale,
                }
            })
        })
        .collect::<Vec<_>>()
}

fn really_old_render_code() {
    for (bind_group_label, bind_group) in &self.texture_bind_groups {
        if !self.bind_group_indexes.contains_key(bind_group_label) {
            continue;
        }
        render_pass.set_bind_group(0, bind_group, &[]);
        for (instance_label, inst_vec) in &mut self.bind_group_indexes {
            if *instance_label != *bind_group_label {
                continue;
            }
            inst_vec.iter().for_each(|i| {
                let i = *i as u64;
                render_pass.draw_indexed(
                    0..INDICES.len() as u32,
                    0,
                    (i as u32)..(i + 1) as u32,
                );
            });
            inst_vec.clear();
        }
    }
}

fn old_render_code() {
    let x = Instant::now();
        for (tex_index, tex_bind_group) in &self.tex_index_hash_bind {
            if let Some(inst_vec) = self.tex_hash_inst.get_mut(tex_index) {
                render_pass.set_bind_group(0, tex_bind_group, &[]);
                for i in inst_vec.drain(..) {
                    render_pass.draw_indexed(0..INDICES.len() as u32, 0, i..(i + 1));
                }
            }
        }
        let x = x.elapsed().as_micros(); //50-150 micros
        println!("{x}");
        /*
        foreach tex
            if an instance uses tex
                foreach instance that uses tex
                    draw(inst)
        */
}

fn old_draw_texture_partial() {
    match self.tex_hash_inst.get_mut(&texture.index) {
            Some(instance_vec) => instance_vec.push(self.instances_drawn as u32),
            None => {
                self.tex_hash_inst
                    .insert(texture.index, vec![self.instances_drawn as u32]);
            }
        }
}

fn bad_shader_code() {
    /*
    let inst0_x = instance.vec2_0.x * window_size.x;
    let inst1_y = instance.vec2_1.y * window_size.y;
    let inst2_x = instance.vec2_2.x * window_size.x * 2. - 1.;
    let inst2_y = instance.vec2_2.y * window_size.y * -2. + 1.;

    let x = vertex.pos.x * inst0_x + vertex.pos.y * instance.vec2_0.y;
    let y = vertex.pos.x * instance.vec2_1.x + vertex.pos.y * inst1_y;
    let w = inst2_x * vertex.pos.x + inst2_y * vertex.pos.y + 1.;
    // let x = inst0_x * vertex.pos.x + vertex.pos.y * instance.vec2_1.y + inst2_x;

    let updated_model = vec4<f32>(x + camera.pos.x, y + camera.pos.y, vertex.pos.z, w);
    out.clip_position = updated_model;
    */
}

fn old_time_code() {
    /*//let i = Instant::now();
            let tick_len = 1000000000. * 0.93 / tps as f64;
            
            

            let nano_to_sleep = tick_len - self.last_delta_t.elapsed().as_nanos() as f64;
            //x = i.elapsed().as_nanos() as f64;
            //println!("{}", micro_to_sleep);
            //x = self.last_delta_t.elapsed().as_nanos() as f64;
            /*if micro_to_sleep - 100 > 0 {
                //micro_to_sleep -= 100;
            }*/
            //println!("{}", self.last_delta_t.elapsed().as_nanos());
            if nano_to_sleep > 0. {
                //let i = Instant::now();
                spin_sleep::sleep(Duration::from_nanos(nano_to_sleep as u64));
                //let i = i.elapsed().as_nanos();
                //x = i as f64;
                //println!("i = {i}");
            }
            //while self.last_delta_t.elapsed().as_secs_f64() < 0.99 / tps as f64 {}*/
}


use goodman::prelude::*;

fn main() {
    block_on(run());
}

const SCREEN_SIZE: Vec2 = vec2(1200., 900.);

async fn run() {
    let event_loop = EventLoop::new();
    let mut engine: Engine = Engine::new(SCREEN_SIZE, &event_loop).await;

    engine.set_fps(Some(144));

    let paddle_bytes = include_bytes!("assets/paddle.png");
    let paddle_tex = engine.create_texture(paddle_bytes, "paddle.png");
    let ball_bytes = include_bytes!("assets/ball.png");
    let ball_tex = engine.create_texture(ball_bytes, "ball.png");
    let block_bytes = include_bytes!("assets/block.png");
    let block_tex = engine.create_texture(block_bytes, "block.png");

    let paddle_bytes = include_bytes!("assets/Ball.png");
    let a = engine.create_texture(paddle_bytes, "a");

    let ball_bytes = include_bytes!("assets/BallMotion.png");
    let b = engine.create_texture(ball_bytes, "b");

    let block_bytes = include_bytes!("assets/Board.png");
    let c = engine.create_texture(block_bytes, "c");

    let block_bytes = include_bytes!("assets/Computer.png");
    let d = engine.create_texture(block_bytes, "d");

    let block_bytes = include_bytes!("assets/Player.png");
    let e = engine.create_texture(block_bytes, "e");

    let block_bytes = include_bytes!("assets/ScoreBar.png");
    let f = engine.create_texture(block_bytes, "f");
    let mut v = vec![paddle_tex, ball_tex, block_tex, a, b, c, d, e, f];
    for i in 0..91 {
        let x = engine.create_texture(block_bytes, &i.to_string());
        v.push(x);
    }

    let breakout = Breakout::new(&mut engine, v);

    engine.enter_loop(breakout, event_loop);
}

struct Breakout {
    ball: Ball,
    paddle: Paddle,
    blocks: Vec<Vec<Block>>,
    textures: Vec<Texture>,
    i: u8,
}
impl Manager for Breakout {
    fn new(state: &mut Engine, textures: Vec<Texture>) -> Self {
        let paddle = Paddle::new(vec2(SCREEN_SIZE.x * 0.5, SCREEN_SIZE.y * 0.1));
        let ball = Ball::new(vec2(0., 0.));

        let mut rects = vec![paddle.rect, ball.to_rect()];

        let mut blocks = Vec::new();
        for j in 0..1 {
            let mut row = Vec::new();
            for i in 0..100 {
                let block = Block::new(i as f64 * 100. + 150., j as f64 * 50. + 500., j);
                rects.push(block.rect);
                row.push(block);
            }
            blocks.push(row);
        }

        state.initialize_instances(rects);

        Self {
            ball,
            paddle,
            blocks,
            textures,
            i: 0,
        }
    }

    fn update(&mut self, frame_time: f64, input: &Input) {
        /*self.paddle.update(input, frame_time);
        self.ball.update(frame_time);

        self.ball.resolve_paddle_collision(&self.paddle);*/

        /*self.blocks.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|mut block| {
                if resolve_collision(&mut self.ball.to_rect(), &mut self.ball.vel, block.rect) {
                    block.lives -= 1;
                }
            });
            row.retain(|block| block.lives > 0);
        });*/
    }

    fn render(&self, state: &mut Engine) {
        //state.draw_texture(self.paddle.rect, &self.textures[0]);
        //state.draw_texture(self.ball.to_rect(), &self.textures[1]);
        self.blocks.iter().for_each(|row| {
            row.iter().for_each(|block| {
                state.draw_texture(block.rect, &self.textures[block.i]); //830000
            })
        });
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: Rect) -> bool {
    // early exit
    let intersection = match a.intersect(b) {
        Some(intersection) => intersection,
        None => return false,
    };
    println!("colliding");

    let to = b.center() - a.center();
    let to_signum = vec2(to.x.signum(), to.y.signum());
    if intersection.w > intersection.h {
        // bounce on y
        a.y -= to_signum.y * intersection.h;
        vel.y = -to_signum.y * vel.y.abs();
    } else {
        // bounce on x
        a.x -= to_signum.x * intersection.w;
        vel.x = -to_signum.x * vel.x.abs();
    }
    true
}

struct Block {
    rect: Rect,
    lives: usize,
    i: usize,
}
impl Block {
    const SIZE: Vec2 = vec2(100., 50.);
    pub fn new(x: f64, y: f64, j: usize) -> Self {
        Self {
            rect: rect(vec2(x, y), Self::SIZE),
            lives: 1,
            i: j,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ball {
    pos: Vec2,
    vel: Vec2,
}
impl Ball {
    const RADIUS: f64 = 25.;
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(200., 200.),
        }
    }
    fn update(&mut self, frame_time: f64) {
        self.pos += self.vel * frame_time;

        if self.pos.x + Self::RADIUS > SCREEN_SIZE.x {
            self.pos.x = SCREEN_SIZE.x - Self::RADIUS;
            self.vel.x *= -1.;
        } else if self.pos.x - Self::RADIUS < 0. {
            self.pos.x = Self::RADIUS;
            self.vel.x *= -1.;
        }
        if self.pos.y + Self::RADIUS > SCREEN_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = SCREEN_SIZE.y - Self::RADIUS;
        } else if self.pos.y - Self::RADIUS < 0. {
            self.pos.y = Self::RADIUS;
            self.vel.y *= -1.;
        }
    }

    fn resolve_paddle_collision(&mut self, paddle: &Paddle) {
        if self.pos.x + Self::RADIUS > paddle.rect.x - paddle.rect.w * 0.5
            && self.pos.x - Self::RADIUS < paddle.rect.x + paddle.rect.w * 0.5
            && self.pos.y - Self::RADIUS < paddle.rect.y + paddle.rect.h * 0.5
        {
            self.pos.y = paddle.rect.y + paddle.rect.h * 0.5 + Self::RADIUS;
            self.vel.y *= -1.;
        }
    }

    fn to_rect(self) -> Rect {
        rect(self.pos, vec2(Ball::RADIUS * 2., Ball::RADIUS * 2.))
    }
}

#[derive(Debug, Clone, Copy)]
struct Paddle {
    rect: Rect,
}
impl Paddle {
    const SPEED: f64 = 500.;
    const SIZE: Vec2 = vec2(180., 60.);

    fn new(pos: Vec2) -> Self {
        Self {
            rect: rect(pos, Self::SIZE),
        }
    }

    fn update(&mut self, input: &Input, frame_time: f64) {
        let speed = Self::SPEED * frame_time;
        let width = self.rect.w * 0.5;

        if input.is_d_pressed() && self.rect.x + width < SCREEN_SIZE.x {
            self.rect.x += speed;
        }
        if input.is_a_pressed() && self.rect.x - width > 0. {
            self.rect.x -= speed;
        }
    }
}
