use slint::ComponentHandle;
use crate::ui::ui::WordCount;

mod ui;

fn main() {
    let word_count = WordCount::new().unwrap();
    let wc_weak = word_count.as_weak();
    word_count.on_button_pressed(move || {
        let wc = wc_weak.upgrade().unwrap();
        let mut val = wc.get_counter();
        val += 1;
        wc.set_counter(val);
    });

    word_count.run().unwrap();

}

fn button_pressed() {

}
