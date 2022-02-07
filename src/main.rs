use std::ptr::read;

mod arguments;
mod readfile;
fn main() {
    let val = arguments::get_arg();

    readfile::read_file(&val[2]);
    println!("{:#?}",val);
}
