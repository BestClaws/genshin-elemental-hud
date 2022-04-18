#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod arc;
mod loading;
mod cooldown;

use std::{
    collections::HashMap,
    time::{Duration, Instant}, hash::Hash,
};

fn main() {
    let party = loading::load_party();
    let data = loading::load_data();

    let mut app = EStatusApp::new(data, party);
    app.setup();

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

use cooldown::CoolDown;
use device_query::{DeviceQuery, DeviceState, Keycode};
use eframe::{
    egui::{self, Frame},
    emath::Align2,
    epaint::{Color32, Pos2, Stroke},
    epi,
};

use egui_extras::image::{RetainedImage};

struct EStatusApp {
    data:Vec<(String, String, HashMap<u32, u32>)>,
    party: Vec<String>,
    device_state: DeviceState,
    cooldowns: HashMap<String, CoolDown>,
    active_chara: usize,
    show: bool,
    eskill_images: HashMap<String, RetainedImage>,
    e_down_at: Option<Instant>,
}

impl EStatusApp {
    fn new(data: Vec<(String, String, HashMap<u32, u32>)>, party:Vec<String>) -> Self {
        Self {
            data,
            party,
            device_state: DeviceState::new(),
            active_chara: 0,
            show: true,
            cooldowns: HashMap::new(),
            eskill_images: HashMap::new(),
            e_down_at: None,
            
        }
    }


    fn setup(&mut self) {
        self.load_eskill_images();
        self.load_cooldowns();
    
    }



    fn load_eskill_images(&mut self) {
        for chara in &self.party {
            let ret_img = loading::retain_image(&format!("assets/{}.png", chara));
            self.eskill_images.insert(chara.to_string(), ret_img);
        }


    }

    fn load_cooldowns(&mut self) {
        for chara in &self.party {
            let mut cd = CoolDown::default();
            // find chara info in data
            for dat_triplet in &self.data {
                if dat_triplet.0.eq(chara) {
                    for (k, v) in &dat_triplet.2 {
                        let v = Duration::from_secs(*v as u64);
                        cd.available.insert(*k, v);
                    }   
                }

            }
            self.cooldowns.insert(chara.to_owned(), cd);
        }
    }

}

impl epi::App for EStatusApp {
    fn name(&self) -> &str {
        "E-Status"
    }

    fn clear_color(&self) -> egui::Rgba {
        // Make sure we don't paint outside of egui window
        egui::Rgba::TRANSPARENT
    }




    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {

        let now = Instant::now();

        // check key presses
        let pressed_keys: Vec<Keycode> = self.device_state.get_keys();

  

        let first_down = pressed_keys.contains(&Keycode::Key1);
        let second_down = pressed_keys.contains(&Keycode::Key2);
        let third_down = pressed_keys.contains(&Keycode::Key3);
        let fourth_down = pressed_keys.contains(&Keycode::Key4);

        let party_size = self.party.len();

        if first_down && party_size > 0 {
            self.active_chara = 0;
        }
        if second_down && party_size > 1{

            self.active_chara = 1;
        }
        if third_down && party_size > 2 {
            self.active_chara = 2;
        }
        if fourth_down && party_size > 3{
            self.active_chara = 3;
        }

     
        // last_used = now, results in cd timer set to max
        if pressed_keys.contains(&Keycode::E) && self.e_down_at == None {
            self.e_down_at = Some(now);
        }

        if !pressed_keys.contains(&Keycode::E) && self.e_down_at != None {

            let e_held_for = now - self.e_down_at.unwrap();
            let e_held_for = e_held_for.as_millis();
            self.e_down_at = None;

            println!("e held for: {}", e_held_for);

        
            let chara = &mut self.party[self.active_chara];
            let mut cd = self.cooldowns.get_mut(chara).unwrap();



            let mut hold_variants: Vec<u32> = cd.available.keys().cloned().collect();
            hold_variants.sort();
            hold_variants.reverse();

            for variant in hold_variants {
                println!("e_held_for {}ms  variant {}ms", e_held_for, variant);
                if e_held_for  >= variant as u128  { 
                    cd.current = variant;
                    println!("done choosing..");
                    break;
                }
            }



            cd.last_used = now;
        }

        // find ready_in and completion ratio

        for i in 0..party_size {
            
            let chara = &mut self.party[i];
            let mut cd = self.cooldowns.get_mut(chara).unwrap();
            let current_cd = cd.current;
            let available_cds = &cd.available;

            let current_cd = available_cds.get(&current_cd).unwrap();

            cd.ready_in = current_cd.as_millis()
                    .saturating_sub(now.duration_since(cd.last_used).as_millis());
       

            cd.completion_ratio = cd.ready_in as f32 / current_cd.as_millis() as f32;
        }

        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::TRANSPARENT,
                ..Default::default()
            })
            .show(ctx, |ui| {

                // hide show HUD
                if pressed_keys.contains(&Keycode::F12) {
                    self.show = false;
                } else if pressed_keys.contains(&Keycode::F11) {
                    self.show = true;
                }
                ui.set_visible(self.show);

                // HUD UI
                ui.vertical(|ui| {
                    ctx.set_pixels_per_point(2.3);
                    for i in 0..party_size {


                        let chara = &mut self.party[i];
                        let cd = self.cooldowns.get_mut(chara).unwrap();

                        let image = self.eskill_images.get(&self.party[i]).unwrap();
                        let image = egui::Image::new(image.texture_id(ctx), image.size_vec2());

                        indicator(ui, cd.ready_in, cd.completion_ratio, image);
                        ui.add_space(14.0);
                    }
                });
            });
    }
}

pub fn indicator(
    ui: &mut egui::Ui,
    ready_in: u128,
    comp_ratio: f32,
    image: egui::Image,
) -> egui::Response {
    // the tint color of e skill logo
    let mut img_tint = Color32::from_rgba_unmultiplied(255, 255, 255, 30);
    let mut circle_tint = Color32::from_rgba_unmultiplied(0, 0, 0, 50);
    let mut text = format!("{:.1}", (ready_in as f32 / 1000.0));

    if ready_in == 0 {
        // img_tint = Color32::from_rgba_unmultiplied(255, 255, 255, 200);
        img_tint = Color32::WHITE;
        circle_tint = Color32::from_rgba_unmultiplied(85, 255, 0, 10);
        text = String::from("");
    }

    let desired_size = egui::vec2(25.0, 25.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().noninteractive();
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let center = egui::pos2(rect.center().x, rect.center().y);

        let image = image.tint(img_tint);
         ui.painter().circle_filled(
            center,
            rect.height() / 2.0,
            circle_tint,
        );
        image.paint_at(
            ui,
            eframe::epaint::Rect {
                min: Pos2 {
                    x: rect.min.x + 5.0,
                    y: rect.min.y + 5.0,
                },
                max: Pos2 {
                    x: rect.max.x - 5.0,
                    y: rect.max.y - 5.0,
                },
            },
        );
        // ui.painter().circle_filled(
        //     center,
        //     rect.height() / 2.0,
        //     circle_tint,
        // );

        ui.painter().text(
            center,
            Align2::CENTER_CENTER,
            text.clone(),
            eframe::epaint::FontId {
                size: 9.0,
                family: eframe::epaint::FontFamily::Proportional,
            },
            Color32::WHITE,
        );


        // cooldown ring
        if ready_in != 0 {
            let mut shapes = vec![];
            let points = arc::get_points(
                center,
                rect.height() / 2.0 - 1.0,
                90.0,
                comp_ratio * 360.0,
                50,
            );
            shapes.push(egui::epaint::Shape::line(
                points,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30)),
            ));
            ui.painter().extend(shapes);
        } else { // acive ring
            let mut shapes = vec![];
            let points = arc::get_points(
                center,
                rect.height() / 2.0 - 1.0,
                90.0,
                365.0,
                50,
            );
            shapes.push(egui::epaint::Shape::line(
                points,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(85, 255, 0, 30)),
            ));
            ui.painter().extend(shapes);
        }
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}


