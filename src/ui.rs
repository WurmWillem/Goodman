use crate::{prelude::Vec64, time::TimeManager};
use cgmath::vec2;
use egui::ClippedPrimitive;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::Platform;
use wgpu::SurfaceConfiguration;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Ui {
    pub platform: Platform,
    pub egui_rpass: egui_wgpu_backend::RenderPass,
    tps_graph: Vec<Vec64>,
    user_ui: Option<UserUi>,
    show_engine_ui: bool,
}
impl Ui {
    pub fn update_egui_rpass(
        &mut self,
        window: &Window,
        config: &SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (Vec<ClippedPrimitive>, ScreenDescriptor) {
        let full_output = self.platform.end_frame(Some(window));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: config.width,
            physical_height: config.height,
            scale_factor: window.scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        self.egui_rpass
            .add_textures(device, queue, &tdelta)
            .expect("add texture ok");

        self.egui_rpass
            .update_buffers(device, queue, &paint_jobs, &screen_descriptor);

        self.egui_rpass
            .remove_textures(tdelta)
            .expect("remove texture ok");

        (paint_jobs, screen_descriptor)
    }

    pub fn set_user_ui(&mut self, ui: UserUi) {
        self.user_ui = Some(ui);
    }

    pub fn should_render(&self) -> bool {
        self.show_engine_ui || self.user_ui.is_some()
    }

    pub fn update_tps_graph(&mut self, x: f64, y: f64) {
        self.tps_graph.push(vec2(x, y));
        self.tps_graph.retain(|vec| vec.x >= x - 10.)
    }

    pub fn render_engine(
        &self,
        win_size: PhysicalSize<u32>,
        time: &TimeManager,
        target_fps: Option<u32>,
        tex_rendered: usize,
    ) {
        if !self.show_engine_ui {
            return;
        }

        egui::Window::new("Engine").show(&self.platform.context(), |ui| {
            let tps_points: egui_plot::PlotPoints =
                self.tps_graph.iter().map(|vec| [vec.x, vec.y]).collect();
            let line = egui_plot::Line::new(tps_points);

            egui_plot::Plot::new("sd")
                .view_aspect(2.)
                .include_y(0.)
                .show(ui, |plot_ui| plot_ui.line(line));

            ui.label(format!(
                "window size: {:?}x{:?}",
                win_size.width, win_size.height
            ));
            let fps = match target_fps {
                Some(_) => time.get_avg_fps(),
                None => time.get_avg_tps(),
            };

            ui.label(format!("FPS: {:?}", fps));
            ui.label(format!("TPS: {:?}", time.get_avg_tps()));
            ui.label(format!("textures rendered this frame: {:?}", tex_rendered));
        });
    }

    pub fn render_game(&self) {
        if let Some(game_ui) = &self.user_ui {
            egui::Window::new(game_ui.title.clone()).show(&self.platform.context(), |ui| {
                for label in &game_ui.labels {
                    ui.label(label);
                }
            });
        }
    }

    pub fn new(platform: Platform, egui_rpass: RenderPass, show_engine_ui: bool) -> Self {
        Self {
            tps_graph: vec![],
            platform,
            egui_rpass,
            user_ui: None,
            show_engine_ui,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserUi {
    title: String,
    labels: Vec<String>,
}
impl UserUi {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            labels: vec![],
        }
    }
    pub fn set_title(&mut self, label: &str) {
        self.title = label.to_string();
    }
    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }
}
