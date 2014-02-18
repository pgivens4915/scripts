use std::io::buffered::BufferedReader;
use std::io::File;

fn main(){
  println!("Start");
  let path = Path::new("input.txt");
  let mut file = BufferedReader::new(File::open(&path));
  let lines :  ~[~str] = file.lines().collect();

  for line in lines.iter() {
    print(*line);
  }
}
