use eframe::{
    egui::{self, Context, Ui},
    Frame,
};

const HASH_FIELD_WIDTH: f32 = 380.0;
const LABEL_TO_INPUT_SPACE: f32 = 10.0;
const LABEL_TEXT_SIZE: f32 = 16.0;

pub struct HashApp {
    picked_file: Option<String>,
    md5_hash: String,
    sha1_hash: String,
    sha256_hash: String,
}

impl Default for HashApp {
    fn default() -> Self {
        Self {
            picked_file: None,
            md5_hash: String::new(),
            sha1_hash: String::new(),
            sha256_hash: String::new(),
        }
    }
}

impl eframe::App for HashApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(egui::RichText::new("Calculate Hashes").size(30f32));
            ui.add_space(30f32);
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    // HARDCODED SPACE INSIDE RICHTEXT, MAYBE FIX THIS (FOR EVERY HASH)
                    egui::RichText::new("MD5:").size(LABEL_TEXT_SIZE)
                ));
                ui.add_space(LABEL_TO_INPUT_SPACE + 18.0);
                ui.add_sized([HASH_FIELD_WIDTH, 20.0],
                             egui::TextEdit::singleline(&mut self.md5_hash)
                                 .interactive(false)
                );
            });
            ui.add_space(20f32);
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("SHA1:").size(LABEL_TEXT_SIZE)
                ));
                ui.add_space(LABEL_TO_INPUT_SPACE + 15.0);
                ui.add_sized([HASH_FIELD_WIDTH, 20.0],
                             egui::TextEdit::singleline(&mut self.sha1_hash)
                                 .interactive(false)
                );
            });
            ui.add_space(20f32);
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("SHA256:").size(LABEL_TEXT_SIZE)
                ));
                ui.add_space(LABEL_TO_INPUT_SPACE);
                ui.add_sized([HASH_FIELD_WIDTH, 20.0],
                             egui::TextEdit::singleline(&mut self.sha256_hash)
                                 .interactive(false)
                );
            });
        });
    }
}