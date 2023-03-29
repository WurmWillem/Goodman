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

fn old_render_code() {
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