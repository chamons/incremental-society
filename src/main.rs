use eframe::{egui, epi};
use specs::prelude::*;

mod ecs;

pub struct App {
    ecs: World,
}

impl Default for App {
    fn default() -> Self {
        App { ecs: World::new() }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Incremental Society"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });
    }
}

fn main() {
    let app = App::default();
    eframe::run_native(Box::new(app));
}
