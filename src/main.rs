use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("net.koonts.WordCount")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Word Count")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Open File");
        button.connect_clicked(|_| {
            // load file
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}
