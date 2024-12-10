#![allow(unused_imports, dead_code)]

use std::{
    fs::File,
    process::{Command, Output},
};
use std::io::prelude::*;

use json;
use cursive::views::{ListView, Button};
use cursive::{Cursive, CursiveExt};

fn open_json() -> std::io::Result<String> {
    let mut file = File::open("/home/caioc/prog/utilitymenu/resources/utilities.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run_command(siv: &mut Cursive, command: String) {
    let _cmd_output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();
    siv.quit();
}


fn main() {
    let contents: String = open_json().unwrap();
    let parsed: json::JsonValue = json::parse(&contents).unwrap();
    let entries = parsed.entries();

    let mut siv = Cursive::new();
    let mut list_view = ListView::new();

    for (key, value) in entries.into_iter() {
        let value_string = value.to_string();
        let value_clone = value_string.clone();

        list_view.add_child(
            "",
            Button::new(
                key.to_string(),
                move |s| run_command(s, value_clone.clone())
            )
        );

    }
    siv.add_layer(list_view);

    /*
    * why are we still here?
    * just to suffer?
    */
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
