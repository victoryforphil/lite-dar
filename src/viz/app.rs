use egui::{load::SizedTexture, Color32, Pos2, Stroke, TextureId, Vec2};
use log::info;

use crate::sim::Simulation;

pub struct LiteDarApp {
    sim: Simulation,
}

impl Default for LiteDarApp {
    fn default() -> Self {
        Self {
            sim: Simulation::new(),
        }
    }
}

impl LiteDarApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for LiteDarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.sim.initd == false {
            self.sim.init();
            info!("Simulation initialized");
        }
        let res = self.sim.tick();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });

            ui.separator();

            let lidar_res = res.tof_results[0].result.clone();
            //draw idar res using ui.painter().rect
            for (x, row) in lidar_res.iter().enumerate() {
                for (y, col) in row.iter().enumerate() {
                    // Red if col == 1.0
                    // black if col == 0.0
                    let color = if *col == 1.0 {
                        Color32::RED
                    } else {
                        Color32::BLACK
                    };
                    let size = 10.0;
                    let rect = egui::Rect::from_min_size(
                        Pos2::new(x as f32 * size, y as f32 * size + 100.0),
                        Vec2::new(size, size),
                    );
                    ui.painter()
                        .rect(rect, 0.0, color, Stroke::new(0.0, Color32::RED));
                }
            }
        });
    }
}
