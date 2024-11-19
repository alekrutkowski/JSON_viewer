// Made iteratively with 'ChatGPT 4o'

#![windows_subsystem = "windows"]

use eframe::egui;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use rfd::FileDialog;
use std::fs;

struct JsonPreviewApp {
    json_data: Option<JsonValue>,
    error: Option<String>,
}

impl Default for JsonPreviewApp {
    fn default() -> Self {
        Self {
            json_data: None,
            error: None,
        }
    }
}

impl eframe::App for JsonPreviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open JSON File").clicked() {
                if let Some(file_path) = FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => {
                            match serde_json::from_str::<JsonValue>(&content) {
                                Ok(json) => {
                                    self.json_data = Some(json);
                                    self.error = None;
                                }
                                Err(e) => self.error = Some(format!("Failed to parse JSON: {}", e)),
                            }
                        }
                        Err(e) => self.error = Some(format!("Failed to read file: {}", e)),
                    }
                }
            }

            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(json) = &self.json_data {
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        render_json(ui, json, 0);
                    });
            }
        });
    }
}

/// Custom JSON type that preserves key order.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum JsonValue {
    Object(IndexMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(serde_json::Number),
    Bool(bool),
    Null,
}

/// Render JSON in a tree-like structure.
fn render_json(ui: &mut egui::Ui, value: &JsonValue, depth: usize) {
    match value {
        JsonValue::Object(map) => {
            for (key, val) in map {
                ui.collapsing(format!("{}{}", "  ".repeat(depth), key), |ui| {
                    render_json(ui, val, depth + 1);
                });
            }
        }
        JsonValue::Array(array) => {
            for (i, val) in array.iter().enumerate() {
                ui.collapsing(format!("{}[{}]", "  ".repeat(depth), i), |ui| {
                    render_json(ui, val, depth + 1);
                });
            }
        }
        JsonValue::String(s) => {
            ui.label(format!("{}\"{}\"", "  ".repeat(depth), s));
        }
        JsonValue::Number(n) => {
            ui.label(format!("{}{}", "  ".repeat(depth), n));
        }
        JsonValue::Bool(b) => {
            ui.label(format!("{}{}", "  ".repeat(depth), b));
        }
        JsonValue::Null => {
            ui.label(format!("{}null", "  ".repeat(depth)));
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "JSON Preview App",
        options,
        Box::new(|_cc| Box::new(JsonPreviewApp::default())),
    );
}
