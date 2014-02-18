use std::io::buffered::BufferedReader;
use std::io::File;

fn main(){
  // File IO
  let path = Path::new("input.txt");
  let mut file = BufferedReader::new(File::open(&path));
  let lines :  ~[~str] = file.lines().collect();

  // Removing the longitudes
  let longitudesCsv : &~str = lines.head();
  let data : &[~str] = lines.tail();

  // Parsing the longitudes
  let longitudes : ~[&str] = longitudesCsv.split(',').collect();

  // The main work
  for line in data.iter() {
    let chunks : ~[&str] = line.split(',').collect();
    printMap()
    //println!(chunks[0], longitudes[i], value);
  }
}
