use std::ptr::read;

mod arguments;
mod readfile;
fn main() {
    let val = arguments::get_arg();

    let file_contents = readfile::read_file(&val[2]);

    if val[0] == "-c"{
        println!("{:#?}",file_contents.as_str().contains(val[1].as_str()));
    }else if val[0] == "-i"{
        let iter:Vec<&str> = file_contents.split_whitespace().collect();
        
      for i in iter.iter() {
          println!("word = {} search = {}",i, i == &val[1].as_str());
      }
    }

}
