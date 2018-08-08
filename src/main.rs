extern crate gtk;
use gtk::prelude::*;
use gtk::Builder;

fn main() {
    gtk::init().unwrap();
    // First we get the file content.
    let glade_src = include_str!("../quizLetMut.glade");
    // Then we call the Builder call.
    let builder = Builder::new_from_string(glade_src);

    // Our window id is "window1".
    let window: gtk::Window = builder.get_object("window").unwrap();
    window.show_all();

    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    let button: gtk::Button = builder.get_object("button").unwrap();
    let label: gtk::Label = builder.get_object("validation").unwrap();
    let entry: gtk::Entry = builder.get_object("answer").unwrap();

    button.connect_clicked(move |_| {
        match entry.get_buffer().get_text().as_str() {
            "Auto"      => label.set_markup("correct!"),
            "voiture"   => label.set_markup("wrong language"),
            _           => label.set_markup("wrong!")
        }
    });


    // We start the gtk main loop.
    gtk::main();

}
