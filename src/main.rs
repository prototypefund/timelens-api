#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::process::Command;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use rocket::response::NamedFile;

#[get("/0.1/youtube/<vid>")]
fn hello(vid: String) -> NamedFile {
    let url = format!("https://youtube.com/watch?v={}", &vid);
    let file = format!("/tmp/{}.3gp", &vid);
    let timeline = format!("{}.timeline.jpg", &file);

    if !Path::new(&file).is_file() {
        println!("Downloading...");
        Command::new("youtube-dl").args(&["-f", "18", "-o", &file, &url]).output().unwrap();
    }
    if !Path::new(&timeline).is_file() {
        println!("Running timelens...");
        Command::new("timelens").args(&[&file, "--timeline", &timeline]).output().unwrap();
    }
    println!("Done.");

    NamedFile::open(Path::new(&timeline)).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
