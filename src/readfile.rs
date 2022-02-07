use std::io::Read;
use std::fs::File;

// D:\\work\\Playaround\\minigrep \\absolute
// .\\xy.txt relative


pub fn read_file(path: &String){
  let mut file = std::fs::File::open(path).unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents);
   print!("{}", contents);
  
}