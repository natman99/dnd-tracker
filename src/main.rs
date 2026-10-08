use std::{fs, path::Path};

use eframe::egui::{self, Color32, Key, RichText};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

use crate::model::{Entities, Entity, Sorting};

mod model;

const PATH: &str = "autosave.json";

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )?)
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MyEguiApp {
    table: Entities,
    copy_q: Vec<usize>,
    del_q: Vec<usize>,
    #[serde(skip)]
    temp: Entity,
    deleted: Option<Entity>,
    sort: Sorting,
    #[serde(skip)]
    path: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut s = Self {
            table: Entities::test(),
            ..Default::default()
        };
        s.table.sort(&s.sort);
        s
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("D&D Tracker");
            let height = ui.available_height();

            for i in &self.copy_q {
                self.table.e.insert(*i, self.table.e[*i].clone());
            }
            self.copy_q.clear();

            for i in self.del_q.iter().rev() {
                self.deleted = Some(self.table.e.remove(*i));
            }
            self.del_q.clear();

            ui.horizontal(|ui| {
                let b = ui.button(format!("{}", self.sort));
                if b.clicked_by(egui::PointerButton::Primary) {
                    self.sort = self.sort.next();
                    self.table.sort(&self.sort);
                }
                if b.clicked_by(egui::PointerButton::Secondary) {
                    self.sort = self.sort.prev();
                    self.table.sort(&self.sort);
                }
                ui.label("Shift + click to delete");
            });

            ui.horizontal(|ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.path);
                if ui.button("...").clicked() {
                    let handle = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_title("Pick file")
                        .set_directory(std::env::current_dir().unwrap_or(".".into()))
                        .pick_file();
                    if let Some(h) = handle {
                        if let Ok(true) = fs::exists(h.as_path()) {
                            load(self, h.as_path());
                        }
                        self.path = format!("{}", h.display());
                    }
                }
            });

            let table = TableBuilder::new(ui)
                .max_scroll_height(height)
                .columns(Column::auto(), 5)
                .column(Column::auto());

            let mut changed = false;
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Initiative");
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Armor Class");
                    });
                    header.col(|ui| {
                        ui.strong("Health");
                    });
                    header.col(|ui| {
                        ui.strong("Piece");
                    });

                    header.col(|ui| {
                        ui.label("-");
                    });
                })
                .body(|mut body| {
                    for (idx, i) in self.table.e.iter_mut().enumerate() {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                changed |= ui.text_edit_singleline(&mut i.initiative).lost_focus();
                            });
                            row.col(|ui| {
                                changed |= ui.text_edit_singleline(&mut i.name).lost_focus();
                            });
                            row.col(|ui| {
                                changed |= ui.text_edit_singleline(&mut i.armor_class).lost_focus();
                            });
                            row.col(|ui| {
                                changed |= ui.text_edit_singleline(&mut i.health).lost_focus();
                            });
                            row.col(|ui| {
                                changed |= ui.text_edit_singleline(&mut i.piece).lost_focus();
                            });
                            row.col(|ui| {
                                egui::Sides::new().show(
                                    ui,
                                    |left| {
                                        if left.button("Copy").clicked() {
                                            self.copy_q.push(idx);
                                        }
                                    },
                                    |right| {
                                        if right
                                            .button(RichText::new("Delete").color(Color32::RED))
                                            .clicked()
                                        {
                                            let valid = right.input(|i| {
                                                i.key_down(Key::ShiftLeft)
                                                    || i.key_down(Key::ShiftRight)
                                            });
                                            if valid {
                                                self.del_q.push(idx);
                                            }
                                        }
                                    },
                                );
                            });
                        });
                    }

                    body.row(18.0, |mut row| {
                        row.col(|_ui| {});
                    });
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.temp.initiative);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.temp.name);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.temp.armor_class);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.temp.health);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.temp.piece);
                        });
                        row.col(|ui| {
                            let b = ui.input(|i| {
                                i.key_down(Key::ShiftLeft) || i.key_down(Key::ShiftRight)
                            });
                            if ui.button("Add").clicked() {
                                self.table.e.push(self.temp.clone());
                                changed |= true;
                                if b {
                                    self.temp.clear();
                                }
                            }
                        });
                    });
                });

            if changed && !ui.text_edit_focused() {
                self.table.sort(&self.sort);
                auto_save(self);
                if !self.path.is_empty() {
                    let p = self.path.clone();
                    p.replace(".json", "").replace("json", "").push_str(".json");

                    save(self, p);
                }
            }
        });
    }
}

fn auto_save(s: &MyEguiApp) {
    save(s, PATH);
}

fn save(s: &MyEguiApp, path: impl AsRef<Path>) {
    if let Ok(e) = serde_json::to_string_pretty(s) {
        match fs::write(path.as_ref(), e) {
            Ok(_) => (),
            Err(e) => println!("Save failed: {e}"),
        }
    }
}

fn load(app: &mut MyEguiApp, path: impl AsRef<Path>) {
    if let Ok(s) = fs::read_to_string(path.as_ref())
        && let Ok(s) = serde_json::from_str(&s)
    {
        *app = s;
    } else {
        println!("Loading failed");
    }
}
