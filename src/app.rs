use crate::world::{CharacterState, World};
use eframe::egui::Button;
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    world: World,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            world: World::new("hello".to_owned(), "qwertyuiopasdfghjklzxcvbnm".to_owned()),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Wordle"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { label, world } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        let characters = world.characters.clone();
        let grid = world.grid.clone();
        let origin_label = label.clone();

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.label(&origin_label);

            for i in 0..grid.len() {
                ui.horizontal(|ui| {
                    for j in 0..grid[i].len() {
                        let item = grid[i][j];
                        let s = String::from(item.inner).to_uppercase();
                        match item.state {
                            CharacterState::Right => {
                                ui.label(
                                    egui::RichText::new(s)
                                        .color(egui::Color32::WHITE)
                                        .background_color(egui::Color32::DARK_GREEN),
                                );
                            }
                            CharacterState::WrongPos => {
                                ui.label(
                                    egui::RichText::new(s)
                                        .color(egui::Color32::WHITE)
                                        .background_color(egui::Color32::KHAKI),
                                );
                            }
                            CharacterState::Wrong => {
                                ui.label(
                                    egui::RichText::new(s)
                                        .color(egui::Color32::WHITE)
                                        .background_color(egui::Color32::DARK_GRAY),
                                );
                            }
                            CharacterState::Untouch => {
                                ui.label(
                                    egui::RichText::new("   ".to_owned())
                                        .background_color(egui::Color32::WHITE),
                                );
                            }
                            CharacterState::Buffer => {
                                ui.label(
                                    egui::RichText::new(s)
                                        .color(egui::Color32::BLACK)
                                        .background_color(egui::Color32::WHITE),
                                );
                            }
                        }
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0;
                for c in &characters[0..10] {
                    let s = String::from(c.inner).to_uppercase();
                    let button = match c.state {
                        CharacterState::Right => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GREEN)
                        }
                        CharacterState::WrongPos => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::KHAKI)
                        }
                        CharacterState::Wrong => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GRAY)
                        }
                        _ => Button::new(egui::RichText::new(s).color(egui::Color32::BLACK))
                            .fill(egui::Color32::LIGHT_GRAY),
                    };
                    if ui.add(button).clicked() {
                        world.input_char(c.inner);
                        dbg!(c);
                    };
                }
            });

            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.spacing_mut().item_spacing.x = 5.0;
                for c in &characters[10..19] {
                    let s = String::from(c.inner).to_uppercase();
                    let button = match c.state {
                        CharacterState::Right => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GREEN)
                        }
                        CharacterState::WrongPos => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::KHAKI)
                        }
                        CharacterState::Wrong => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GRAY)
                        }
                        _ => Button::new(egui::RichText::new(s).color(egui::Color32::BLACK))
                            .fill(egui::Color32::LIGHT_GRAY),
                    };
                    if ui.add(button).clicked() {
                        world.input_char(c.inner);
                        dbg!(c);
                    };
                }
            });

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0;

                let enter_button =
                    Button::new(egui::RichText::new("Enter").color(egui::Color32::BLACK))
                        .fill(egui::Color32::LIGHT_GRAY);
                if ui.add(enter_button).clicked() {
                    match world.enter() {
                        Ok(_) => self.label = "good".to_string(),
                        Err(e) => self.label = e.to_string(),
                    };
                    dbg!(&world);
                }

                for c in &characters[19..] {
                    let s = String::from(c.inner).to_uppercase();
                    let button = match c.state {
                        CharacterState::Right => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GREEN)
                        }
                        CharacterState::WrongPos => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::KHAKI)
                        }
                        CharacterState::Wrong => {
                            Button::new(egui::RichText::new(s).color(egui::Color32::WHITE))
                                .fill(egui::Color32::DARK_GRAY)
                        }
                        _ => Button::new(egui::RichText::new(s).color(egui::Color32::BLACK))
                            .fill(egui::Color32::LIGHT_GRAY),
                    };
                    if ui.add(button).clicked() {
                        world.input_char(c.inner);
                        dbg!(c);
                    };
                }

                let del_button =
                    Button::new(egui::RichText::new("DEL").color(egui::Color32::BLACK))
                        .fill(egui::Color32::LIGHT_GRAY);
                if ui.add(del_button).clicked() {
                    world.delete_char();
                    dbg!("DEL");
                }
            });

            if ui.button("DEBUG").clicked() {
                dbg!(&world);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
