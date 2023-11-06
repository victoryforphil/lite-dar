use egui::{load::SizedTexture, Color32, Pos2, Stroke, TextureId, Vec2};
use log::info;

use crate::sim::{runner::SimRunner, Simulation};

pub struct LiteDarApp {
    sim_runner: SimRunner,
}

impl Default for LiteDarApp {
    fn default() -> Self {
        Self {
            sim_runner: SimRunner::new(),
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
        let result = self.sim_runner.channel_rx.try_recv();

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });

            if ui.button("Start Sim").clicked() {
                self.sim_runner.start();
            }

            ui.separator();

            match result {
                Ok(state) => {
                    let lidar_res = state.tof_results[0].result.clone();
                    //draw idar res using ui.painter().rect
                    for (x, row) in lidar_res.iter().enumerate() {
                        for (y, col) in row.iter().enumerate() {
                            // Red if col == 1.0
                            // black if col == 0.0
                            let color = if *col != 0.0 {
                                Color32::RED
                            } else {
                                Color32::BLACK
                            };
                            let size = 5.0;
                            let rect = egui::Rect::from_min_size(
                                Pos2::new(x as f32 * size, y as f32 * size + 100.0),
                                Vec2::new(size, size),
                            );
                            ui.painter().rect(
                                rect,
                                0.0,
                                color,
                                Stroke::new(0.0, Color32::TRANSPARENT),
                            );

                            ui.ctx().request_repaint();
                        }
                    }
                }
                Err(_) => {
                    ui.label("No results");
                }
            }
        });
    }
}
