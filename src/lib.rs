use std::io::Read;
use eframe::{
    egui::{self, Context, Ui},
    Frame,
};

const HASH_FIELD_WIDTH: f32 = 480.0;
const LABEL_TO_INPUT_SPACE: f32 = 10.0;
const LABEL_TEXT_SIZE: f32 = 16.0;

pub struct HashApp {
    picked_file: String,
    md5_hash: String,
    sha1_hash: String,
    sha256_hash: String,
}

impl Default for HashApp {
    fn default() -> Self {
        Self {
            picked_file: String::new(),
            md5_hash: String::new(),
            sha1_hash: String::new(),
            sha256_hash: String::new(),
        }
    }
}
impl HashApp {
    fn calculate_hashes(&mut self) {
        let mut file = std::fs::File::open(&self.picked_file)
            .expect("Couldn't open the file");
        let mut buffer: Vec<u8> = Vec::new();
        file.read(&mut buffer).expect("Couldn't read the data");

        let md5_digest = md5::compute(&buffer);
        self.md5_hash = format!("{:x}", md5_digest);

        let sha256_digest = sha256::digest_bytes(&buffer);
        self.sha256_hash = sha256_digest;

        let mut sha1_hasher = sha1_smol::Sha1::new();
        sha1_hasher.update(&buffer);
        self.sha1_hash = sha1_hasher.digest().to_string();
    }
}

fn preview_drop_file_on_hover(ctx: &Context) {
    use egui::*;
    use std::fmt::Write;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut file_path = String::new();

       for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(file_path, "{}", path.display()).ok();
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
    }
}

// TODO: Eliminate all hardcoded values and see if sizes are consistent across different machines
impl eframe::App for HashApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(egui::RichText::new("Calculate Hashes").size(30f32));
            ui.add_space(20f32);
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(egui::RichText::new("File: ")
                    .size(LABEL_TEXT_SIZE)));
                ui.add_space(LABEL_TO_INPUT_SPACE + 22.0);
                ui.add_sized([HASH_FIELD_WIDTH-68.0, 20.0], egui::TextEdit::singleline(&mut self.picked_file)
                                .interactive(false)
                );
                if ui.button("Open file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_file = path.display().to_string();
                        self.calculate_hashes();
                    }
                }
            });
            ui.add_space(20f32);
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    // HARDCODED SPACE INSIDE RICHTEXT, MAYBE FIX THIS (FOR EVERY HASH)
                    egui::RichText::new("MD5:").size(LABEL_TEXT_SIZE)
                ));
                ui.add_space(LABEL_TO_INPUT_SPACE + 18.0);
                ui.add_sized([HASH_FIELD_WIDTH, 20.0],
                             egui::TextEdit::singleline(&mut self.md5_hash)
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
                );
            });

            preview_drop_file_on_hover(ctx);
        });

    }
}