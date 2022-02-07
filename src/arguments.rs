use std::env;

pub fn get_arg()->Vec<String>{
  let flag = env::args().nth(1).expect("need flag");
  let to_search = env::args().nth(2).expect("need flag");
  let path = env::args().nth(3).expect("need flag");

  // is flag check
  assert_eq!(flag.chars().next().unwrap(),"-".chars().next().unwrap());
  
  let val = vec![flag,to_search,path];
  val
}