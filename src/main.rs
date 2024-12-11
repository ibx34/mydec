use std::{fs::OpenOptions, io::Read};

fn main() {
    let file = OpenOptions::new().read(true).open("ZillaKami - BLEACH ft. Denzel Curry (Official Music Video).mp3").unwrap();

    println!("{:?}", file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>().len());
}
