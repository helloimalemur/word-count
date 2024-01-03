use slint::{ComponentHandle, LogicalSize, Model, SharedString, Window, WindowSize};
use crate::ui::ui::WordCount;

mod ui;
mod entities;
mod docx;

fn main() {
    let word_count = WordCount::new().unwrap();
    let wc_weak = word_count.as_weak();
    word_count.on_button_pressed(move || {
        // word count handle
        let wc = wc_weak.upgrade().unwrap();
        // counter
        let mut val = wc.get_counter();



        let mut array = wc.get_list_of_structs();

        let mut current_row = wc.get_list_of_structs().row_count();
        current_row = val as usize;
        array.set_row_data(current_row, (SharedString::from("text: asdfsd"),));
        wc.set_list_of_structs(array);

        println!("{}", current_row);

        // for i in 0..10 {
        //     let mut array = wc.get_list_of_structs();
        //     array.set_row_data(i as usize, (SharedString::from("asf"), SharedString::from("asf")))
        // }


        // increment counter
        val += 1;
        wc.set_counter(val);
    });
    let ws = WindowSize::Logical(LogicalSize::new(200f32, 300f32));


    word_count.run().unwrap();

}

fn button_pressed() {

}
