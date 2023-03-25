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