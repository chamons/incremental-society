use crate::society::prelude::*;
use eframe::egui::Ui;
use specs::prelude::*;

pub fn render_resources(ecs: &World, ui: &mut Ui) {
    ui.add_space(3.0);
    let players = ecs.read_storage::<PopComponent>();
    let pop = (&players).join().count();
    ui.label(format!("Population: {}", pop));
    ui.add_space(10.0);
    let resources = ecs.read_resource::<Resources>();
    for r in resources.kinds() {
        ui.label(format!("{}: {}", r, resources.get(r)));
    }

    ui.add_space(1.0);
}

pub fn render_log(_ecs: &World, ui: &mut Ui) {
    ui.add_space(3.0);
    let logs = vec!["asfd".to_string(), "asfd".to_string(), "3".to_string()];
    for i in 0..logs.len().max(8) {
        ui.label(logs.get(i).cloned().unwrap_or_else(|| "".to_string()));
    }
    ui.add_space(1.0);
}
