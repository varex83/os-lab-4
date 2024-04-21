use std::env;
use std::io::{self, Read, Seek, SeekFrom};

fn get_process_memory(pid: u32, pointer: usize) -> Result<Vec<u8>, io::Error> {
    let path = format!("/proc/{}/mem", pid);
    let mut file = std::fs::File::open(path)?;
    file.seek(SeekFrom::Start(pointer as u64))?;
    let mut buffer = vec![0; 4];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn get_secret(pid: u32, pointer: usize) -> Result<u32, io::Error> {
    let memory = get_process_memory(pid, pointer)?;
    Ok(u32::from_le_bytes(memory.try_into().unwrap()))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let pid: u32 = args[1].parse().unwrap();
    let pointer = usize::from_str_radix(&args[2][2..], 16).unwrap();

    println!("Reading secret from PID {} at address {:x}", pid, pointer);

    let mut secret = get_secret(pid, pointer).unwrap();

    println!("Secret: {}", secret);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let new_secret = get_secret(pid, pointer).unwrap();

        if new_secret != secret {
            println!("Secret changed! New value: {}", new_secret);
            secret = new_secret;
        }
    }
}
