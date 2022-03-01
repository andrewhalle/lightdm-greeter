fn main() {
    println!("cargo:rustc-link-search=native=/home/andrew/c/lightdm/liblightdm-gobject");
    println!("cargo:rustc-link-lib=static=lightdm_gobject");
}
