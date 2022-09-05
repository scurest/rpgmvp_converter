use std::fs;
use std::path::Path;
use std::env::args_os;

fn main() {
    // Get the path to the file from arguments
    let args: Vec<_> = args_os().collect();
    let path = Path::new(&args[1]);

    println!("Path to file: {}", path.display());

    // Read the file
    let mut buf = fs::read(&path).expect("Error reading the file!");

    // Remove 16 bytes, then change the next 16 bytes to PNG header
    let png = &mut buf[16..];
    let png_head = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
    (&mut png[..16]).copy_from_slice(png_head);

    // Write the new file
    let mut end_filename = path.to_owned();
    end_filename.set_extension("png");
    fs::write(&end_filename, png).expect("Error writing the file!");

    // println!("{:?}", &buf);
    println!("Exported to {}", end_filename.display());
}
