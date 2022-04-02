use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::path::Path;

struct InputParam {
    arg_name: String,
    param_value: String,
}

struct DataLine {
    date: String,
    value1: String,
    value2: String,
    value3: String,
}

impl DataLine {
    fn to_string(self) -> String {
        format!("Date: {}, Value1: {}, Value2: {}, Value3: {}",
            self.date, self.value1, self.value2, self.value3)
    }
}


fn main() {
    println!("Hello world, I am Julio, the avenger!");
    println!("Just trying out some stuff here...");
    println!("");
    println!("");

    let mut all_args: Vec<InputParam> = Vec::new();

    for argument in env::args() {

        let split = argument.split("=");
        let parts: Vec<&str> = split.collect();

        let mut cur_arg = InputParam {
            arg_name: String::from(parts[0]),
            param_value: String::from("")
        };
        if parts.len() == 2 {
            cur_arg.param_value = String::from(parts[1])
        }
        all_args.push(cur_arg);
    }

    let mut input_file: String = String::from("");
    let mut separator = String::from(",");

    let mut help_asked = false;

    for arg_item in all_args {
        match arg_item.arg_name.as_ref() {
            "-i" | "--input" => {
                input_file = arg_item.param_value
            },
            "-h" | "--help" => {
                help_asked = true;
                println!("DEBUG: help asked for");
            },
            "-s" | "--separator" => {
                separator = String::from(arg_item.param_value);
            },
            _ => {
            }

        }
    }

    if help_asked {
        show_help();
    }

    if !input_file.is_empty() {
        println!("Input file is: {}", input_file);
        let data_items = read_file(input_file, &separator);

        for cur_item in data_items.unwrap() {
            println!("{}", cur_item.to_string());
        }
    }
}

/// Read a file and return all its lines in a vector of DataLine items
///
fn read_file(input_file: String, separator: &str) -> io::Result<Vec<DataLine>> {
    println!("Reading file and printing it out on screen...");
    println!("");

    let mut read_data: Vec<DataLine> = Vec::new();
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_item = line.unwrap();
        let split = line_item.split(separator);
        let parts: Vec<&str> = split.collect();
        if parts.len() != 4 {
            continue;
        }
        if parts[0] == String::from("Date") {
            continue;
        }

        let data_line = DataLine {
            date: String::from(parts[0]),
            value1: String::from(parts[1]),
            value2: String::from(parts[2]),
            value3: String::from(parts[3]),
        };
        read_data.push(data_line);
    }

    Ok(read_data)
}

/// Show help for the valid arguments and parameters to use when calling the application
///
/// # Examples
/// ...
/// benford_analyze -i=sample1.txt
/// benford_analyze -i=sample2.txt -o=testme.txt
/// ...
fn show_help() {
    println!("No worry amigo, help is on the way...");
    println!("...");
    println!("Input parameters:");
    println!("\t-i=<filename> or --input=<filename>");
    println!("\t-o=<filename> or --output=<filename>");
    println!("\t-s=<separator> or --separator=<separator> use the specified character/string to split the lines by");
    println!("");
    println!("... and of course we might add stuff later!");
}

