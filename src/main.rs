#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{time::{Instant, Duration}, vec, iter::Inspect};

use device_query::{DeviceState, Keycode, DeviceQuery};
use eframe::{
    egui::{self, Frame, TextFormat},
    epi, epaint::{Pos2, Color32},
};
// use egui_extras::image::RetainedImage;
use egui::text::LayoutJob;

struct MyEguiApp {
    device_state: DeviceState,
    cooldowns: Vec<Duration>,
    last_used: Vec<Instant>,
    active_chara: usize,
    show: bool,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            device_state: DeviceState::new(),
            cooldowns: vec![Duration::from_secs(6),Duration::from_secs(4),Duration::from_secs(16),Duration::from_secs(21)],
            last_used: vec![Instant::now(),Instant::now(),Instant::now(),Instant::now(),],
            active_chara: 0,
            show: true,
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


        if first_down { self.active_chara = 0; }
        if second_down { self.active_chara = 1; }
        if third_down { self.active_chara = 2; }
        if fourth_down { self.active_chara = 3; }



        let e_down = keys.contains(&Keycode::E);

        if e_down {
            self.last_used[self.active_chara] = now;
            println!("received event");
        }

       
   


        let mut next_use_in_for_1 = (self.cooldowns[0].as_secs().saturating_sub(now.duration_since(self.last_used[0]).as_secs())).to_string();
        let mut next_use_in_for_2 = (self.cooldowns[1].as_secs().saturating_sub(now.duration_since(self.last_used[1]).as_secs())).to_string();
        let mut next_use_in_for_3 = (self.cooldowns[2].as_secs().saturating_sub(now.duration_since(self.last_used[2]).as_secs())).to_string();
        let mut next_use_in_for_4 = (self.cooldowns[3].as_secs().saturating_sub(now.duration_since(self.last_used[3]).as_secs())).to_string();
      


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
                    ui.add_space(20.0);

   

                    ui.label(get_layout_job(next_use_in_for_1));
                    ui.add_space(20.0);
                    ui.label(get_layout_job(next_use_in_for_2));
                    ui.add_space(20.0);
                    ui.label(get_layout_job(next_use_in_for_3));
                    ui.add_space(20.0);
                    ui.label(get_layout_job(next_use_in_for_4));
                });


            });
    }
}

fn get_layout_job(mut text: String) -> LayoutJob {
    let mut job = LayoutJob::default();

    let mut color = Color32::WHITE;

    if text.eq("0") {
        color = Color32::GREEN;
        text = String::from("💡");
    }

    job.append(
        text.as_str(),
        0.0,
        TextFormat {
            color: color,
            background: Color32::from_rgba_premultiplied(0,0,0,50),
            ..Default::default()
        },
    );

    job
}


fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions {
        transparent: true,
        initial_window_size: Some(egui::Vec2 { x: 80.0, y: 300.0 }),
        always_on_top: true,
        decorated: false,
        initial_window_pos: Some(Pos2 {x: 1550.0, y: 240.0}),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);

    
    
}
