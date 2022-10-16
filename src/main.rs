use egui::Vec2;
use egui_memory_editor::{option_data::MemoryEditorOptions, MemoryEditor};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(932., 600.));

    eframe::run_native(
        "Mmap Viewer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    mem_editor: MemoryEditor,
    options: MemoryEditorOptions,
    buf: Vec<u8>,
}

impl Default for App {
    fn default() -> Self {
        let mut options = MemoryEditorOptions::default();
        options.column_count = 32;
        options.is_options_collapsed = true;

        let mem_editor = MemoryEditor::new()
            .with_options(options.clone())
            .with_address_range("All", 0..0xFFFF)
            .with_address_range("IO", 0xFF00..0xFF80);

        Self {
            mem_editor: mem_editor,
            options: options,
            buf: vec![0 as u8; 0x10000],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mem_editor
                .draw_editor_contents_read_only(ui, &mut self.buf, |mem, address| {
                    mem[address].into()
                });
        });
    }
}
