use dotext::{Docx, MsDoc};
use std::io::Read;

pub fn read(path: String) -> String {
    let mut file = Docx::open(path).expect("could not load file");
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    println!("CONTENT:");
    println!("----------BEGIN----------");
    println!("{}", isi);
    println!("----------EOF----------");
    isi
}
