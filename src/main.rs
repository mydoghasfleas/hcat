use clap::Parser;

/// Provide a hex dump of the file
#[derive(Parser,Debug)]
struct Cli {
    /// The file to open
    filename: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    // let file_read_result = std::fs::read("./Cargo.toml");
    let file_read_result = std::fs::read(args.filename);
    match file_read_result {
        Err(err) => println!("Failed to open file. ({})", err),
        Ok(bytes) => {
            // println!("Length: {} ", bytes.len());
            dump_bytes(bytes);
        }    
    }
}

fn dump_bytes(bytes : Vec<u8>) {
    let mut len = bytes.len();
    let mut idx = 0;
    
    while len > 16 {
        print!("{:08x}  ", idx);
        let sb = &bytes[idx..idx+16];
        output_byte_group(&sb);
        output_byte_group_ascii(&sb);
        println!();

        len -= 16;
        idx += 16;
    }
    if len > 0 {
        print!("{:08x}  ", idx);
        let sb = &bytes[idx..bytes.len()];
        output_byte_group(&sb);
        output_byte_group_ascii(&sb);
        println!();
    }
}

fn output_byte_group(bytes : &[u8]) {
    let mut c1 = 0;
    for &b in bytes {
        print!("{:02x} ", b);
        // Additional group break after 8 bytes
        c1 += 1;
        if c1 == 8 {
            print!(" ");
            c1 = 0;
        }
    }

    // Pad to 16 places to ensure alignment
    let rem = 16 - bytes.len();
    for _ in 0..rem {
        print!("   ");
        // Additional group break after 8 bytes
        c1 += 1;
        if c1 == 8 {
            print!(" ");
            c1 = 0;
        }        
    }

}

fn output_byte_group_ascii(bytes : &[u8]) {
    print!("|");
    for &b in bytes {
        match b {
            32..=126 => print!("{}", char::from(b)),
            _ => print!(".")
        }
        
    }
    print!("|");
}