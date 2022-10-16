use std::{env, fs::File};

use egui::Vec2;
use egui_memory_editor::{option_data::MemoryEditorOptions, MemoryEditor};
use memmap::Mmap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(932., 600.));

    eframe::run_native(filename, options, Box::new(|_cc| Box::new(App::new(mmap))));
}

struct App {
    mem_editor: MemoryEditor,
    buf: Mmap,
}

impl App {
    fn new(mmap: Mmap) -> Self {
        let mut options = MemoryEditorOptions::default();
        options.column_count = 32;
        options.is_options_collapsed = true;

        let mem_editor = MemoryEditor::new()
            .with_options(options.clone())
            .with_address_range("All", 0..mmap.len());

        Self {
            mem_editor: mem_editor,
            buf: mmap,
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
