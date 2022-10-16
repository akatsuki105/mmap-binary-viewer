use egui_memory_editor::MemoryEditor;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mmap Viewer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    mem_editor: MemoryEditor,
    buf: Vec<u8>,
    is_open: bool,
}

impl Default for App {
    fn default() -> Self {
        let mem_editor = MemoryEditor::new()
            .with_address_range("All", 0..0xFFFF)
            .with_address_range("IO", 0xFF00..0xFF80);
        Self {
            mem_editor: mem_editor,
            buf: vec![0 as u8; 0x10000],
            is_open: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mem_editor.window_ui_read_only(
                ctx,
                &mut self.is_open,
                &mut self.buf,
                |mem, address| mem[address].into(),
            );
        });
    }
}
