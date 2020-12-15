//! Nothing to change here.
//!
//! Binary for your Zoo.
//!
//! This is just boilerplate for argument parsing, serilization, deserialization
//! and reading/writing files.

use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;

use my_zoo::{cage_em_all, Animal, Result};

// Note that main can return a `Result`. If you want to write a serious command
// line application, you should look at proper error handling techniques in
// Rust. There are multiple crates out there, which handle error better, than
// just throwing them at the user.
fn main() -> Result<()> {
    // First parse the input from the command line
    let matches = App::new("My Own Zoo")
        .version("1.0")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .help("input JSON file"),
        )
        .arg(
            Arg::with_name("food")
                .long("food")
                .short("f")
                .help("specify an animal that should be used as food")
                .takes_value(true),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let food = matches.value_of("food");

    // Read the contents of the JSON file passed through the arguments
    let input_file = PathBuf::from(input);
    let input_json = fs::read_to_string(&input_file)?;

    // Let serde deserialize the json into the specified type `Vec<Animals>`
    let animals: Vec<Animal> = serde_json::from_str(&input_json)?;

    // Run the library function to put the animals in cages
    let cages = cage_em_all(animals, food)?;

    // Serialize the `cages: Vec<Cage>` to JSON
    let output_json = serde_json::to_string_pretty(&cages)?;

    // Write the serialized JSON string to an output file
    let output_file = PathBuf::from(&format!(
        "{}_solution",
        input_file
            .file_stem()
            .expect("input was a file")
            .to_string_lossy()
    ))
    .with_extension("json");
    fs::write(output_file, output_json)?;

    Ok(())
}
