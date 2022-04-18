// anything related to loading
// loading from file, loading to memory etc.,

use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::Read,
};

use egui_extras::RetainedImage;
use serde_yaml;

pub fn load_party() -> Vec<String> {
    let f = std::fs::File::open("party.yaml").unwrap();
    let d: Vec<String> = serde_yaml::from_reader(f).unwrap();
    println!("Read YAML string: {:#?}", d);
    d
}

pub fn load_data() -> Vec<(String, String, HashMap<u8, u8>)> {
    let f = std::fs::File::open("data.yaml").unwrap();
    let d: Vec<(String, String, HashMap<u8, u8>)>;
    d = serde_yaml::from_reader(f).unwrap();
    println!("Read YAML string: {:#?}", d);
    d
}

// loads image data from file into memory.
pub fn retain_image(name: &str) -> RetainedImage {
    let mut f = File::open(name).expect("no file found");
    let metadata = fs::metadata(name).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    let img = RetainedImage::from_image_bytes(name, &buffer).unwrap();
    img
}
