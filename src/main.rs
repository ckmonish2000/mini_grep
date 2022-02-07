mod arguments;
fn main() {
    let val = arguments::get_arg();
    println!("{:#?}",val);
}
