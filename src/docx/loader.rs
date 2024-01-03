use docx_rs::*;

struct DocX {
    contents: String,
}

impl DocX {
    fn new() -> DocX {
        DocX { contents: "".to_string() }
    }
    fn load(&mut self, path: String) {
        self.contents = "sdf".to_string();
    }
}
