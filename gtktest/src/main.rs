use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Entry, Button, Fixed};

fn main() {
    let application = Application::builder()
        .application_id("com.some.Test")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        
        window.set_title("some title");
        window.set_default_size(185, 80);
        window.set_border_width(15);

        let fixed = Fixed::new();
        
        let label = Label::new(None);
        let entry = Entry::new();
        let button = Button::with_label("Show");
        
        label.set_markup("Name:"); 

        fixed.put(&label, 10, 0);
        fixed.put(&entry, 10, 25);
        fixed.put(&button, 115, 75);

        button.connect_clicked(move |_| {
             println!("{}", entry.buffer().text());
             
             entry.buffer().set_text("");
        });
        
        window.add(&fixed);
        
        window.show_all();
    });

    application.run();
}
