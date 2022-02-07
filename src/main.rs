use std::ptr::read;

mod arguments;
mod readfile;
fn main() {
    let val = arguments::get_arg();

    let file_contents = readfile::read_file(&val[2]);
    println!("{:#?}",file_contents);
}
