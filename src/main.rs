use std::cell::RefCell;

use eframe::{
    egui,
    egui::{epaint::text::FontDefinitions, Style, TextStyle, Ui},
    epi,
};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

mod ecs;

#[derive(Component, Serialize, Deserialize, Clone, Default)]
struct PopComponent {}

fn register_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PopComponent>();
    ecs
}

fn create_world() -> World {
    let mut ecs = register_world();
    for _ in 0..5 {
        ecs.create_entity().with(PopComponent::default()).build();
    }
    ecs
}

pub struct App {
    ecs: World,
    resources_open: RefCell<bool>,
    style: Style,
    fonts: Option<FontDefinitions>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ecs: create_world(),
            resources_open: RefCell::new(true),
            style: create_style(),
            fonts: None,
        }
    }
}

fn create_style() -> Style {
    let mut style = Style::default();
    style.spacing.item_spacing.x += 4.0;
    style.spacing.window_padding.x = 12.0;
    style
}

fn configure_fonts(fonts: &FontDefinitions) -> FontDefinitions {
    let mut fonts = fonts.clone();
    fonts.family_and_size.get_mut(&TextStyle::Body).unwrap().1 = 16.0;
    fonts
}

fn show_menu_option(ui: &mut Ui, name: &str, value: bool) -> bool {
    let action = if value { "Hide" } else { "Show" };
    ui.button(format!("{} {}", action, name)).clicked()
}

impl App {
    fn set_style(&mut self, ctx: &egui::CtxRef) {
        ctx.set_style(self.style.clone());

        let fonts = self.fonts.get_or_insert_with(|| configure_fonts(ctx.fonts().definitions()));
        ctx.set_fonts(fonts.clone());
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Incremental Society"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.set_style(ctx);

        egui::Window::new("Resources")
            .collapsible(false)
            .scroll(true)
            .resizable(true)
            .open(&mut self.resources_open.borrow_mut())
            .default_pos((4.0, 28.0))
            .default_size((250.0, 200.0))
            .show(ctx, |ui| {
                ui.add_space(3.0);

                let players = self.ecs.read_storage::<PopComponent>();
                let pop = (&players).join().count();
                ui.label(format!("Population: {}", pop));

                ui.label(format!("Stability: 100"));
                ui.add_space(1.0);
            });

        egui::TopPanel::top("menu").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "Views", |ui| {
                    let borrow = *self.resources_open.borrow();
                    if show_menu_option(ui, "Resources", borrow) {
                        *self.resources_open.borrow_mut() = !borrow;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

fn main() {
    let app = App::default();
    eframe::run_native(Box::new(app));
}
