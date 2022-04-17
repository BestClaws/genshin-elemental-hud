#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    iter::Inspect,
    time::{Duration, Instant},
    vec,
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use eframe::{
    egui::{self, Frame, TextFormat},
    emath::Align2,
    epaint::{Color32, Pos2},
    epi,
};
use egui::text::LayoutJob;
use egui_extras::image::RetainedImage;

struct MyEguiApp {
    device_state: DeviceState,
    cooldowns: Vec<Duration>,
    last_used: Vec<Instant>,
    active_chara: usize,
    show: bool,

    logo: RetainedImage,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            device_state: DeviceState::new(),
            cooldowns: vec![
                Duration::from_secs(6),
                Duration::from_secs(4),
                Duration::from_secs(16),
                Duration::from_secs(21),
            ],
            last_used: vec![
                Instant::now(),
                Instant::now(),
                Instant::now(),
                Instant::now(),
            ],
            active_chara: 0,
            show: true,
            logo: RetainedImage::from_image_bytes("logo", include_bytes!("../logo.png")).unwrap(),
        }
    }
}

impl epi::App for MyEguiApp {
    fn name(&self) -> &str {
        "E-Status"
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // check keyinputs

        let keys: Vec<Keycode> = self.device_state.get_keys();

        let now = Instant::now();

        let first_down = keys.contains(&Keycode::Key1);
        let second_down = keys.contains(&Keycode::Key2);
        let third_down = keys.contains(&Keycode::Key3);
        let fourth_down = keys.contains(&Keycode::Key4);

        if first_down {
            self.active_chara = 0;
        }
        if second_down {
            self.active_chara = 1;
        }
        if third_down {
            self.active_chara = 2;
        }
        if fourth_down {
            self.active_chara = 3;
        }

        let e_down = keys.contains(&Keycode::E);

        if e_down {
            self.last_used[self.active_chara] = now;
            println!("received event");
        }

        // find time remaining
        let ready_in_1 = (self.cooldowns[0]
            .as_secs()
            .saturating_sub(now.duration_since(self.last_used[0]).as_secs()))
            .to_string();
        let ready_in_2 = (self.cooldowns[1]
            .as_secs()
            .saturating_sub(now.duration_since(self.last_used[1]).as_secs()))
            .to_string();
        let ready_in_3 = (self.cooldowns[2]
            .as_secs()
            .saturating_sub(now.duration_since(self.last_used[2]).as_secs()))
            .to_string();
            let ready_in_4 = (self.cooldowns[3]
            .as_secs()
            .saturating_sub(now.duration_since(self.last_used[3]).as_secs()))
        .to_string();

        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::TRANSPARENT,
                ..Default::default()
            })
            .show(ctx, |ui| {
                if keys.contains(&Keycode::F12) {
                    self.show = false;
                } else if keys.contains(&Keycode::F11) {
                    self.show = true;
                }

                ui.set_visible(self.show);

                // let rimg =
                //     RetainedImage::from_image_bytes("img1", include_bytes!("img1.png")).unwrap();
                ui.vertical(|ui| {
                    ctx.set_pixels_per_point(2.3);

                    let image = egui::Image::new(self.logo.texture_id(ctx), self.logo.size_vec2());
                    indicator(ui, String::from(ready_in_1), image);
                    ui.add_space(14.0);
                    let image = egui::Image::new(self.logo.texture_id(ctx), self.logo.size_vec2());
                    indicator(ui, String::from(ready_in_2), image);
                    ui.add_space(14.0);
                    let image = egui::Image::new(self.logo.texture_id(ctx), self.logo.size_vec2());
                    indicator(ui, String::from(ready_in_3), image);
                    ui.add_space(14.0);
                    let image = egui::Image::new(self.logo.texture_id(ctx), self.logo.size_vec2());
                    indicator(ui, String::from(ready_in_4), image);
                });
            });
    }
}







fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions {
        transparent: true,
        initial_window_size: Some(egui::Vec2 { x: 80.0, y: 300.0 }),
        always_on_top: true,
        decorated: false,
        initial_window_pos: Some(Pos2 {
            x: 1500.0,
            y: 240.0,
        }),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}

pub fn indicator(ui: &mut egui::Ui, mut text: String, image: egui::Image) -> egui::Response {

    // the tint color of e skill logo
    let mut img_tint = Color32::from_rgba_premultiplied(50, 50, 50, 50);

    if text.eq("0") {
        img_tint = Color32::WHITE;
        text = String::from("");
    }


    let desired_size = egui::vec2(25.0, 25.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().noninteractive();
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let center = egui::pos2(rect.center().x, rect.center().y);

        

        let image = image.tint(img_tint);
        image.paint_at(
            ui,
            eframe::epaint::Rect {
                min: Pos2 { x: rect.min.x + 5.0, y: rect.min.y + 5.0},
                max: Pos2 { x: rect.max.x - 5.0, y: rect.max.y - 5.0},
            },
        );
        ui.painter().circle_filled(
            center,
            rect.height() / 2.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 30),
        );

        ui.painter().text(
            center,
            Align2::CENTER_CENTER,
            text.clone(),
            eframe::epaint::FontId {
                size: 10.0,
                family: eframe::epaint::FontFamily::Proportional,
            },
            Color32::WHITE,
        );

 
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}
