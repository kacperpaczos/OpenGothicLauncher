use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Orientation, Box, Label};

fn main() {
    let app = Application::builder()
        .application_id("com.github.pryoxar.OpenGothicLauncher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenGothic Launcher")
        .default_width(600)
        .default_height(400)
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);

    let title_label = Label::new(Some("Welcome to OpenGothic Launcher"));
    vbox.append(&title_label);

    let launch_button = Button::with_label("Launch OpenGothic");
    launch_button.connect_clicked(|_| {
        println!("Launch button clicked! (Backend integration pending)");
    });
    vbox.append(&launch_button);

    let engines_button = Button::with_label("Manage Engines");
    vbox.append(&engines_button);

    window.set_child(Some(&vbox));
    window.present();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gui_compiles_and_harness_works() {
        // GTK startup tests require a display server (X11/Wayland). 
        // This test ensures that the module compiles correctly and cargo test works.
        assert_eq!(2 + 2, 4);
    }
}
