use std::ptr::null_mut;

use glib_sys::g_main_loop_new;

use lightdm_gobject_sys::lightdm_greeter_new;

// /home/andrew/c/lightdm/liblightdm-gobject/liblightdm_gobject.a
fn main() {
    unsafe {
        let main_loop = g_main_loop_new(null_mut(), 0);
        let greeter = lightdm_greeter_new();
    }
}
