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
    printMap(longitudes, chunks)
    //println!(chunks[0], longitudes[i], value);
  }
}

fn printMap(longitudesWithGarbage : &[&str], chunks : &[&str]) -> (){
  let longitudes : &[&str]  = longitudesWithGarbage.tail();
  let lat : &str = *(chunks.head());
  let data : &[&str] = chunks.tail();
  printData(longitudes, lat, data)
}

fn printData(longitudes : &[&str], lat : &str, data : &[&str]){
  match longitudes.head_opt(){
     None => return(),
     _    => {
       let lng : &str = *longitudes.head();
       let dataPt: &str = *data.head();
       let a : ~[&str] = lng.split('\n').collect();
       let b : ~[&str] = dataPt.split('\n').collect();
       let c : &str = *a.head();
       let d : &str = *b.head();
       print!("{},{},{}\n", lat, c, d); 
     }
  }
  printData(longitudes.tail(), lat, data.tail());
  return ();
}
