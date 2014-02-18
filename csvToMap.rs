use std::io::buffered::BufferedReader;
use std::io::File;

fn main(){
  // File IO
  let path = Path::new("input.txt");
  let mut file = BufferedReader::new(File::open(&path));
  let lines :  ~[~str] = file.lines().collect();

  // Removing the longitudes
  let longitudes : &~str = lines.head();
  let data : &[~str] = lines.tail();

  // The main work
  for line in data.iter() {
    let chunks : ~[&str] = line.split(',').collect();
    println(chunks[0]);
    for chunk in chunks.iter() {
      print(*chunk);
    }
  }
}
