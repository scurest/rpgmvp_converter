use std::fs;
use std::env::args;

fn main() {
    // Get the path to the file from arguments
    let args: Vec<String> = args().collect();
    let mut path: String = args[1].to_owned();

    println!("Path to file: {}", &path);

    // Read the file
    let buf = fs::read(&path).expect("Error reading the file!");

    // Change first 16 bytes of the RPG Maker V Picture header to 8 bytes of the PNG header
    let png_head: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52];
    let result: Vec<u8> = [&png_head[..], &buf[32..]].concat();

    // Write the new file
    let end_filename = &mut path;
    end_filename.push_str(".png");
    fs::write(&end_filename, &result).expect("Error writing the file!");

    // println!("{:?}", &buf);
    println!("Exported to {}", end_filename);
}
