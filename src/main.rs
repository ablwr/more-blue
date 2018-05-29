extern crate y4m;

use std::io;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 { return println!("Usage: cargo run input.y4m output.y4m"); }
    let end = Path::new(&args[1]).extension().unwrap();
    if end != "y4m" { return println!("Extension must be y4m"); }
    
    let mut incoming: Box<io::Read> = Box::new(File::open(&args[1]).unwrap());
    let mut outgoing: Box<io::Write> = Box::new(File::create(&args[2]).unwrap());

    let mut decoder = y4m::decode(&mut incoming).unwrap();
    
    let (w1, h1) = (decoder.get_width(), decoder.get_height());
    let very_blue = vec![255;w1/2*h1/2];
    let very_bland = vec![0;w1/2*h1/2];

    let mut encoder = y4m::encode(w1, h1, decoder.get_framerate())
        .with_colorspace(decoder.get_colorspace())
        .write_header(&mut outgoing)
        .unwrap();

    loop {
        match decoder.read_frame() {
            Ok(frame) => {
                let out_frame = y4m::Frame::new([&frame.get_y_plane(), &very_blue, &very_bland], None);
                if encoder.write_frame(&out_frame).is_err() { break }
            },
            _ => break,
        }
    }
}