// use std::ptr::null_mut;

// // TODO figure out a better way to do this?
// // required, else glib/gio won't be linked in.
// extern crate gio;
// use glib::object::ObjectExt;
// use glib::value::Value;
// use glib::{BoxedAnyObject, MainLoop};

// use lightdm_gobject_sys::{
//     lightdm_greeter_authenticate, lightdm_greeter_connect_to_daemon_sync, lightdm_greeter_new,
// };

// // /home/andrew/c/lightdm/liblightdm-gobject/liblightdm_gobject.a
// fn main() {
//     unsafe {
//         let main_loop = MainLoop::new(None, false);
//         let greeter_ptr = lightdm_greeter_new();
//         let greeter = BoxedAnyObject::new(greeter_ptr);
//         greeter.connect("show-prompt", false, show_prompt);
//         greeter.connect("authentication-complete", false, authentication_complete);

//         if lightdm_greeter_connect_to_daemon_sync(greeter_ptr, null_mut()) == 0 {
//             panic!();
//         }

//         lightdm_greeter_authenticate(greeter_ptr, null_mut(), null_mut());

//         main_loop.run();
//     }
// }

// fn show_prompt(values: &[Value]) -> Option<Value> {
//     None
// }

// fn authentication_complete(values: &[Value]) -> Option<Value> {
//     None
// }
use std::fs::OpenOptions;
use std::io::Write;
use std::ptr;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

use lightdm_gobject_sys::{lightdm_greeter_connect_to_daemon_sync, lightdm_greeter_new};

fn main() {
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/andrew-greeter.log")
        .unwrap();
    write!(log_file, "[greeter] running").unwrap();
    unsafe {
        let greeter = lightdm_greeter_new();
        let status = lightdm_greeter_connect_to_daemon_sync(greeter, ptr::null_mut());
        write!(log_file, "[greeter] lightdm connection status: {}", status).unwrap();
    }

    let app = Application::builder()
        .application_id("dev.andrewhalle.greeter")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello world!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
