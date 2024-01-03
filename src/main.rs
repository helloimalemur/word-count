use slint::{ComponentHandle, LogicalSize, Window, WindowSize};
use crate::ui::ui::WordCount;

mod ui;
mod entities;

fn main() {
    let word_count = WordCount::new().unwrap();
    let wc_weak = word_count.as_weak();
    word_count.on_button_pressed(move || {
        let wc = wc_weak.upgrade().unwrap();
        let mut val = wc.get_counter();
        val += 1;
        wc.set_counter(val);
    });
    let ws = WindowSize::Logical(LogicalSize::new(200f32, 300f32));


    word_count.run().unwrap();

}

fn button_pressed() {

}
