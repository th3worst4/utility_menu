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
    let mut file = File::open("./resources/utilities.json")?;
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

fn populate_list(entries: json::iterators::Entries) -> ListView {
    let mut list_view = ListView::new();

    for (key, value) in entries.into_iter() {
        let value_string = value.to_string();

        list_view.add_child(
            "",
            Button::new(
                key.to_string(),
                move |s| run_command(s, value_string.clone())
            )
        );

    }

    list_view
}

fn main() {
    let contents: String = open_json().unwrap();
    let parsed: json::JsonValue = json::parse(&contents).unwrap();
    let entries = parsed.entries();

    let mut siv = Cursive::new();
    siv.load_toml(include_str!("../resources/theme.toml")).unwrap();

    let list_view: ListView = populate_list(entries);
    siv.add_layer(list_view);

    /*
    * why are we still here?
    * just to suffer?
    */
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
