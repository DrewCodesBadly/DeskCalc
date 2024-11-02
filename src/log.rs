use egui::Label;

#[derive(Default)]
pub struct Log {
    pub commands: Vec<(String, String)>
}

impl Log {
    pub fn push_results(&mut self, input: &str, output: &str) {
        self.commands.push((input.to_owned(), output.to_owned()))
    }
}