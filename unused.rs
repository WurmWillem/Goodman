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
