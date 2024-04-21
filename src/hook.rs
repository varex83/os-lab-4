use std::io::Read;

fn main() {
    // read /dev/input/mice
    let path = "/dev/input/mice";

    println!("Reading from {}", path);

    let mut file = std::fs::File::open(path).unwrap();

    let mut buffer = [0; 1];

    println!("Printing bytes from file");
    loop {
        file.read_exact(&mut buffer).unwrap();

        println!("{:?}", buffer);
    }
}
