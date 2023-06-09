use self::Layer::*;
use crate::prelude::Engine;

use cgmath::vec2;
use egui_winit_platform::Platform;
use spin_sleep::LoopHelper;
use std::slice::Iter;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec3 = cgmath::Vector3<f64>;

pub type InstIndex = u32;
pub type TexIndex = u32;

pub trait Manager {
    fn new(engine: &mut Engine) -> Self;
    fn start(&mut self) {}
    fn update(&mut self, frame_time: f64, input: &Input);
    fn render(&self, engine: &mut Engine);
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub layer: Layer,
    pub rotation: f64,
}
impl Default for DrawParams {
    fn default() -> Self {
        Self {
            layer: Layer1,
            rotation: 0.,
        }
    }
}

/*pub struct TextureRenderer;
impl TextureRenderer {
    pub fn new() -> TextureRenderer {
        Self {}
    }
    pub fn render_texture(&mut self, rect: &Rect, texture: &Texture) {
        self.render_tex(rect, texture, 0., Layer::Layer1);
    }
    pub fn render_texture_ex(&mut self, rect: &Rect, texture: &Texture, draw_params: DrawParams) {
        self.render_tex(rect, texture, draw_params.rotation, draw_params.layer);
    }
}*/

pub struct TimeManager {
    pub graph_vec: Vec<Vec2>,

    loop_helper: LoopHelper,
    time_since_last_render: f64,
    last_delta_t: f64,
    average_delta_t: f64,
    time_passed_since_creation: f64,

    use_target_tps: bool,
    use_average_tps: bool,
}
impl TimeManager {
    pub fn new() -> Self {
        let loop_helper = LoopHelper::builder()
            .report_interval_s(0.1)
            .build_with_target_rate(144);

        Self {
            graph_vec: vec![],
            loop_helper,
            time_since_last_render: 0.,
            time_passed_since_creation: 0.,
            last_delta_t: 1.,
            average_delta_t: 1. / 100000.,
            use_target_tps: false,
            use_average_tps: false,
        }
    }

    pub fn update(&mut self, platform: &mut Platform) {
        // Sleep until 1 / target_tps is reached
        if self.use_target_tps {
            self.loop_helper.loop_sleep();
        }

        // Get delta_t of last tick and update necessary systems accordingly
        self.last_delta_t = self.loop_helper.loop_start_s();
        self.time_since_last_render += self.last_delta_t;
        self.time_passed_since_creation += self.last_delta_t;

        platform.update_time(self.last_delta_t);

        // Update average delta_t
        if let Some(avg_tps) = self.loop_helper.report_rate() {
            self.average_delta_t = 1. / avg_tps;
            self.graph_vec
                .push(vec2(self.time_passed_since_creation, avg_tps));
            //println!("{}", avg_tps)
        }
    }

    pub fn replace_loop_helper(&mut self, report_interval: f64, target_tps: u32) {
        self.loop_helper = LoopHelper::builder()
            .report_interval_s(report_interval)
            .build_with_target_rate(target_tps);
    }

    pub fn update_graph(&mut self) {
        self.graph_vec
            .retain(|vec| vec.x >= self.time_passed_since_creation - 10.)
    }

    pub fn set_target_tps(&mut self, tps: Option<u32>) {
        match tps {
            Some(tps) => {
                self.loop_helper.set_target_rate(tps);
                self.use_target_tps = true;
            }
            None => self.use_target_tps = false,
        }
    }

    pub fn set_use_target_tps(&mut self, use_target_tps: bool) {
        self.use_target_tps = use_target_tps;
    }

    pub fn reset_time_since_last_render(&mut self) {
        self.time_since_last_render = 0.;
    }

    pub fn get_relevant_delta_t(&self) -> f64 {
        if self.use_average_tps {
            return self.average_delta_t;
        }
        return self.last_delta_t;
    }

    pub fn get_average_tps(&self) -> u32 {
        (1. / self.average_delta_t) as u32
    }

    pub fn get_time_since_last_render(&self) -> f64 {
        self.time_since_last_render
    }
}

pub struct GoodManUI {
    pub title: String,
    pub labels: Vec<String>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Feature {
    EngineUi,
    GameUi,
    AverageTPS(f64),
}

pub struct Features {
    pub engine_ui_enabled: bool,
    pub game_ui_enabled: bool,
    pub average_tps: Option<f64>,
}
impl Features {
    pub fn new() -> Self {
        Self {
            engine_ui_enabled: false,
            game_ui_enabled: false,
            average_tps: None,
        }
    }
    pub fn enable_feature(&mut self, feature: Feature) {
        match feature {
            Feature::EngineUi => self.engine_ui_enabled = true,
            Feature::GameUi => self.game_ui_enabled = true,
            Feature::AverageTPS(report_rate) => self.average_tps = Some(report_rate),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
}
impl Layer {
    pub fn iterator() -> Iter<'static, Layer> {
        static LAYERS: [Layer; 5] = [Layer1, Layer2, Layer3, Layer4, Layer5];
        LAYERS.iter()
    }
}

pub struct Input {
    cursor_pos: Vec2,
    left_mouse_button_pressed: bool,
    right_mouse_button_pressed: bool,
    d_pressed: bool,
    a_pressed: bool,
    w_pressed: bool,
    s_pressed: bool,
    right_arrow_pressed: bool,
    left_arrow_pressed: bool,
    up_arrow_pressed: bool,
    down_arrow_pressed: bool,
}
impl Input {
    pub fn new() -> Self {
        Self {
            cursor_pos: vec2(0., 0.),
            left_mouse_button_pressed: false,
            right_mouse_button_pressed: false,
            d_pressed: false,
            a_pressed: false,
            w_pressed: false,
            s_pressed: false,
            right_arrow_pressed: false,
            left_arrow_pressed: false,
            up_arrow_pressed: false,
            down_arrow_pressed: false,
        }
    }
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => {
                        self.w_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.a_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.s_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.d_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.right_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.left_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Up => {
                        self.up_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.down_arrow_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.left_mouse_button_pressed = is_pressed;
                        true
                    }
                    MouseButton::Right => {
                        self.right_mouse_button_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = vec2(position.x, position.y);
                false
            }
            _ => false,
        }
    }
    pub fn reset_buttons(&mut self) {
        self.left_mouse_button_pressed = false;
        self.right_mouse_button_pressed = false;
        self.d_pressed = false;
        self.a_pressed = false;
        self.w_pressed = false;
        self.s_pressed = false;
        self.right_arrow_pressed = false;
        self.left_arrow_pressed = false;
        self.up_arrow_pressed = false;
        self.down_arrow_pressed = false;
    }

    pub fn get_cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }
    pub fn is_left_mouse_button_pressed(&self) -> bool {
        self.left_mouse_button_pressed
    }
    pub fn is_right_mouse_button_pressed(&self) -> bool {
        self.right_mouse_button_pressed
    }
    pub fn is_d_pressed(&self) -> bool {
        self.d_pressed
    }
    pub fn is_a_pressed(&self) -> bool {
        self.a_pressed
    }
    pub fn is_w_pressed(&self) -> bool {
        self.w_pressed
    }
    pub fn is_s_pressed(&self) -> bool {
        self.s_pressed
    }
    pub fn is_right_arrow_pressed(&self) -> bool {
        self.right_arrow_pressed
    }
    pub fn is_left_arrow_pressed(&self) -> bool {
        self.left_arrow_pressed
    }
    pub fn is_up_arrow_pressed(&self) -> bool {
        self.up_arrow_pressed
    }
    pub fn is_down_arrow_pressed(&self) -> bool {
        self.down_arrow_pressed
    }
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[allow(missing_docs)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WindowUniform {
    pub size: [f32; 2],
}
