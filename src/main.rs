use std::{borrow::BorrowMut, cell::RefCell};

use eframe::{
    egui,
    egui::{epaint::text::FontDefinitions, Style, TextStyle, Ui, Vec2},
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
    log_open: RefCell<bool>,
    style: Style,
    fonts: Option<FontDefinitions>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ecs: create_world(),
            resources_open: RefCell::new(true),
            log_open: RefCell::new(true),
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

        let window = ctx.available_rect();
        let (window_width, window_height) = (window.max.x, window.max.y);

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

        egui::Window::new("Log")
            .collapsible(false)
            .scroll(true)
            .resizable(true)
            .open(&mut self.log_open.borrow_mut())
            .default_pos((window_width - 280.0, window_height - 205.0))
            .default_size((250.0, 200.0))
            .show(ctx, |ui| {
                ui.add_space(3.0);
                ui.label("asdf");
                ui.label("asdf");
                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("");
                ui.add_space(1.0);
            });

        egui::TopPanel::top("menu").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(10.0, 10.0);
                egui::menu::menu(ui, "Views", |ui| {
                    ui.add_space(2.0);

                    let resources = *self.resources_open.borrow();
                    if show_menu_option(ui, "Resources", resources) {
                        *self.resources_open.borrow_mut() = !resources;
                    }

                    let log = *self.log_open.borrow();
                    if show_menu_option(ui, "Log", log) {
                        *self.log_open.borrow_mut() = !log;
                    }

                    if ui.button("Reset Windows").clicked() {
                        *self.resources_open.borrow_mut() = true;
                        *self.log_open.borrow_mut() = true;
                        ui.ctx().memory().borrow_mut().reset_areas();
                    }
                    ui.add_space(2.0);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}

fn main() {
    let app = App::default();
    eframe::run_native(Box::new(app));
}
