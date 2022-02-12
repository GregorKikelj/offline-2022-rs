use std::fs;

#[derive(Debug)]
pub struct TestCase{
  id: i32,
  n: i32,
  col: i32,
  data: Vec<Vec<i32>>,
}

pub fn import_tc(tc:i32) -> Option<TestCase>{
  if tc <= 0 || tc>30{
    return None;
  }
  let filename = "poplavljanje-unix.in";
  let contents = fs::read_to_string(filename).expect("File not found");
  let lines : Vec<&str> = contents.trim_end_matches("\n").split("\n").collect();
  return Some(TestCase{
    id: tc,
    n: 1,
    col: 1,
    data: vec![],
  });
}