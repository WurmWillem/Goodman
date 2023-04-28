use egui::Context;

pub fn create_ui(ctx: &Context) {
    egui::Window::new("yeh").show(ctx, |ui| {
        ui.heading("Gaming");
        if ui.button("click").clicked() {
            println!("clicked")
        }
    });
}
