mod mem;

pub struct Memory {
    pub marker: u32,
}

fn main() {
    let mut memory = Memory { marker: 15121921 };

    println!("PID: {}", std::process::id());
    println!("Address of marker: {:p}", &memory.marker);

    loop {
        let mut input = String::new();
        let _ = std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        memory.marker = input.trim().parse().unwrap();
    }
}
