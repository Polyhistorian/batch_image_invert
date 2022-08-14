use image::io::Reader as ImageReader;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    if !Path::new("./inverted/").exists() {
        fs::create_dir("./inverted/").unwrap();
    }

    let images = fs::read_dir("./images/").unwrap();

    let test: Vec<_> = images.filter_map(|i| i.ok()).collect();

    test.into_par_iter().for_each(|image_res| {
        let entry = image_res;
        let name = entry.file_name();
        let mut img = ImageReader::open(entry.path()).unwrap().decode().unwrap();

        img.invert();
        img.save("./inverted/".to_owned() + "DEV_" + name.to_str().unwrap())
            .unwrap();
    });
}
