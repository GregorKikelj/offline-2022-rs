use std::fs::File;
use std::io::Write;

pub fn write_output(tc:&i8, col:&Vec<i8>){
  let s="513346\nPoplavljanje\n";
  let mut s = format!("{}\n{}\n{}\n", s, tc, col.len());
  for i in col{
    s = format!("{}{} ",s, i);
  }
  s = format!("{}{}", s, "\n");
  let mut f = File::create("output.txt").expect("Unable to create file");
  f.write_all(s.as_bytes()).expect("Unable to write to file");
}