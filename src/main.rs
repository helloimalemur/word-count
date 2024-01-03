use slint::{ComponentHandle, LogicalSize, Model, SharedString, Window, WindowSize};
use crate::ui::ui::WordCount;

mod ui;
mod entities;
mod docx;

fn main() {
    let word_count = WordCount::new().unwrap();
    let wc_weak = word_count.as_weak();
    word_count.on_button_pressed(move || {
        let wc = wc_weak.upgrade().unwrap();
        let mut val = wc.get_counter();
        val += 1;
        wc.set_counter(val);

        let mut strval = wc.get_astring();
        strval = SharedString::from("wonderful".to_string());
        wc.set_astring(strval.clone());

        let mut list = wc.get_listof();
        list.set_row_data(0, Default::default());

        let mut array = wc.get_list_of_structs();

        array.set_row_data(1, (SharedString::from("text: asdfsd"),));
        wc.set_list_of_structs(array);

        // for i in 0..10 {
        //     let mut array = wc.get_list_of_structs();
        //     array.set_row_data(i as usize, (SharedString::from("asf"), SharedString::from("asf")))
        // }

    });
    let ws = WindowSize::Logical(LogicalSize::new(200f32, 300f32));


    word_count.run().unwrap();

}

fn button_pressed() {

}
