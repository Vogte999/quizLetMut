extern crate gtk;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


mod question_buffer;

use gtk::prelude::*;
use gtk::{Builder, Window, Button, Label, Entry};
use question_buffer::QuestionBuffer;
use std::sync::{Arc, Mutex};



fn main() {

    gtk::init().unwrap();
    // First we get the file content.
    let glade_src = include_str!("../quizLetMut.glade");
    // Then we call the Builder call.
    let builder = Builder::new_from_string(glade_src);

    // Our window id is "window".
    let window: Window = builder.get_object("window").unwrap();
    window.show_all();

    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    let button: Button = builder.get_object("button").unwrap();
    let question: Label = builder.get_object("question").unwrap();
    let label: Label = builder.get_object("validation").unwrap();
    let entry: Entry = builder.get_object("answer").unwrap();

    let qb = Arc::new(Mutex::new(QuestionBuffer::new_from_json()));
    let qb = match QuestionBuffer::new_from_json() {
        Ok(b)   => Arc::new(Mutex::new(b)),
        Err(msg)=> Arc::new(Mutex::new(QuestionBuffer::new(vec![("err", "err")])))
    };
    question.set_markup(&qb.lock().unwrap().next().unwrap().0);
    label.set_markup("");


    button.connect_clicked(move |_| {
        if label.get_text().unwrap().as_str() == "" {
            label.set_markup(qb.lock().unwrap().evaluate(entry.get_buffer().get_text().as_str()));
        } else {
            match qb.lock().unwrap().next() {
                Some(val)   => {
                    question.set_markup(&val.0);
                    label.set_markup("");
                }
                None        => {
                    label.set_markup("Congratulations, you did it!");
                }
            }
        }
    });


    // We start the gtk main loop.
    gtk::main();

}
