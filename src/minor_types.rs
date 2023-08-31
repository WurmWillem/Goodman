use crate::{engine::Engine, input::Input};

use cgmath::vec2;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::Platform;
use rodio::{OutputStreamHandle, Source};
use spin_sleep::LoopHelper;
use winit::dpi::PhysicalSize;

pub type Vec2 = cgmath::Vector2<f64>;
pub trait Manager {
    fn new(engine: &mut Engine) -> Self;
    fn start(&mut self) {}
    fn update(&mut self, frame_time: f64, input: &Input, sound: &Sound);
    fn render(&self, engine: &mut Engine);
}
pub struct TimeManager {
    loop_helper: LoopHelper,

    time_since_prev_render: f64,
    prev_iter_was_render: bool,

    total_fps_this_report_interval: f64,
    frames_passed_this_report_interval: u32,
    avg_fps: u32,

    prev_delta_t: f64,
    avg_delta_t: f64,

    time_passed_since_creation: f64,

    use_target_tps: bool,
    use_avg_tps: bool,
}
impl TimeManager {
    pub fn new(average_tps: Option<f64>, target_tps: u32, use_target_tps: bool) -> Self {
        let report_interval = average_tps.unwrap_or(0.1);
        let loop_helper = LoopHelper::builder()
            .report_interval_s(report_interval)
            .build_with_target_rate(target_tps);

        Self {
            loop_helper,
            time_since_prev_render: 0.,
            prev_iter_was_render: false,
            total_fps_this_report_interval: 144.,
            frames_passed_this_report_interval: 0,
            avg_fps: 144,
            avg_delta_t: 1. / target_tps as f64,
            prev_delta_t: 1.,
            time_passed_since_creation: 0.,
            use_target_tps,
            use_avg_tps: average_tps.is_some(),
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        // Sleep until 1 / target_tps is reached
        if self.use_target_tps {
            self.loop_helper.loop_sleep();
        }

        // Get delta_t of last tick and update necessary systems accordingly
        self.prev_delta_t = self.loop_helper.loop_start_s();
        self.time_passed_since_creation += self.prev_delta_t;
        self.time_since_prev_render += self.prev_delta_t;
        ui.platform.update_time(self.prev_delta_t);

        // Run code if there was rendered the previous iteration
        if self.prev_iter_was_render {
            self.total_fps_this_report_interval += 1. / self.time_since_prev_render;
            self.frames_passed_this_report_interval += 1;

            self.time_since_prev_render = 0.;
            self.prev_iter_was_render = false;
        }

        // This if-let is true every report rate
        if let Some(avg_tps) = self.loop_helper.report_rate() {
            self.avg_fps = (self.total_fps_this_report_interval.round() as u32
                / self.frames_passed_this_report_interval) as u32;

            self.frames_passed_this_report_interval = 0;
            self.total_fps_this_report_interval = 0.;

            self.avg_delta_t = 1. / avg_tps;

            ui.tps_graph
                .push(vec2(self.time_passed_since_creation, avg_tps));
            ui.tps_graph
                .retain(|vec| vec.x >= self.time_passed_since_creation - 10.)
        }
    }

    pub fn enable_prev_iter_was_render(&mut self) {
        self.prev_iter_was_render = true;
    }

    pub fn get_relevant_delta_t(&self) -> f64 {
        if self.use_avg_tps {
            return self.avg_delta_t;
        }
        self.prev_delta_t
    }

    pub fn get_avg_tps(&self) -> u32 {
        (1. / self.avg_delta_t) as u32
    }
    pub fn get_avg_fps(&self) -> u32 {
        self.avg_fps
    }

    pub fn get_time_since_last_render(&self) -> f64 {
        self.time_since_prev_render
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub rotation: f64,
}
impl Default for DrawParams {
    fn default() -> Self {
        Self { rotation: 0. }
    }
}

pub struct Ui {
    pub platform: Platform,
    pub egui_rpass: egui_wgpu_backend::RenderPass,
    tps_graph: Vec<Vec2>,
    game_ui: Option<GoodManUI>,
    show_engine_ui: bool,
}
impl Ui {
    pub fn should_render(&self) -> bool {
        self.show_engine_ui || self.game_ui.is_some()
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
            let tps_points: egui::plot::PlotPoints =
                self.tps_graph.iter().map(|vec| [vec.x, vec.y]).collect();
            let line = egui::plot::Line::new(tps_points);

            egui::plot::Plot::new("sd")
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

    pub fn render_game_ui(&self) {
        if let Some(game_ui) = &self.game_ui {
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
            game_ui: None,
            show_engine_ui,
        }
    }
}

pub struct Sound {
    #[allow(dead_code)] // stream is unused but it has to stay in memory
    stream: rodio::OutputStream,
    stream_handle: OutputStreamHandle,
}
impl Sound {
    pub fn new() -> Self {
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().expect("can't find output device");
        Self {
            stream,
            stream_handle,
        }
    }
    pub fn play_sound<S>(&self, source: S) -> Result<(), rodio::PlayError>
    where
        S: Source<Item = f32> + Send + 'static,
    {
        self.stream_handle.play_raw(source)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! create_textures {
    ($engine: expr, $textures: expr, $($name: expr)*) => {
        let mut i = 0;
        $(
            let tex_bytes = include_bytes!($name);
            $textures.push($engine.create_texture(tex_bytes).unwrap());
            i += 1;
        )*
       $engine.use_textures(&$textures, i);
    };
}

pub struct GoodManUI {
    title: String,
    labels: Vec<String>,
}
impl GoodManUI {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
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

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WindowUniform {
    pub size: [f32; 2],
}
