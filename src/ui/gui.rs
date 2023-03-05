use std::cell::RefCell;
use std::rc::Rc;

use crate::data::piece::Color;
use crate::data::BoardConfig;
use egui::{Color32, Context};

pub(super) struct Gui {
    board_config: Rc<RefCell<BoardConfig>>,
    fen: String,
    bit_board: String,
}

impl Gui {
    /// Create a `Gui`.
    pub(super) fn new(board_config: Rc<RefCell<BoardConfig>>) -> Self {
        Self {
            board_config,
            fen: "".to_string(),
            bit_board: "p".to_string(),
        }
    }

    /// Create the UI using egui.
    pub(super) fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("Left Panel")
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(5.))
            .show(ctx, |ui| {
                let mut config = self.board_config.borrow_mut();

                ui.strong("chess-rs");

                ui.heading("In Play");
                ui.label({
                    match config.get_active_color() {
                        Color::White => "White",
                        Color::Black => "Black",
                    }
                });

                ui.separator();

                ui.heading("Board Configuration");
                ui.label(egui::RichText::new(config.get_fen()).size(10.0).monospace());
                egui::CollapsingHeader::new("Edit").show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.fen));
                    if ui.button("Load Fen").clicked() {
                        config.load_fen(&self.fen);
                    }
                });

                ui.separator();

                if ui.button("Reset").clicked() {
                    config.reset();
                }

                ui.separator();

                ui.heading("Bit Boards");
                ui.label("Select Bitboard: ");
                ui.text_edit_singleline(&mut self.bit_board);
                if self.bit_board.len() == 1 {
                    let c = self.bit_board.chars().next().unwrap();
                    if let Some(b) = config.get_bit_board(c) {
                        let b = (0..8)
                            .map(|x| format! {"{0:08b}", ((b & (0xff << x*8)) >> x*8) as u8})
                            .map(|s| s.chars().rev().collect::<String>())
                            .collect::<Vec<String>>()
                            .join("\n");
                        ui.label(
                            egui::RichText::new(b)
                                .background_color(Color32::BLACK)
                                .size(10.0)
                                .monospace(),
                        );
                    } else {
                        ui.label(format!("{} is not valid BoardPiece", self.bit_board));
                    }
                }
            });
    }
}
