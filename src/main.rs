use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::string::String;

mod argparser;

struct DataSet {
    name: String,
    values: Vec<String>
}

fn main() {
    println!("Hello world, I am Julio, the avenger!");
    println!("Just trying out some stuff here...");
    println!("");

    let mut all_args: Vec<String> = Vec::new();

    for (idx, argument) in env::args().enumerate() {
        if idx == 0 {
            continue;
        }

        all_args.push(argument);
    }

    let parsed_arguments = argparser::parser::parse_arguments(all_args);

    let input_file = match parsed_arguments.get("input_file") {
        Some(input_file_name) => String::from(input_file_name),
        None => String::from(""),
    };

    let separator = match parsed_arguments.get("separator") {
        Some(sep) => String::from(sep),
        None => String::from(","),
    };

    let help_asked = match parsed_arguments.get("show_help") {
        Some(_) => true,
        None => false,
    };

    if help_asked {
        show_help();
    }

    if !input_file.is_empty() {
        println!("Input file is: {}", input_file);
        let data_items = read_file(input_file, &separator);

        show_read_datasets(data_items.unwrap());
    }
}

/// Read a file and return all its lines in a vector of DataSet items
///
fn read_file(input_file: String, separator: &str) -> io::Result<Vec<DataSet>> {
    println!("Reading file and printing it out on screen...");
    println!("");

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut data_sets: Vec<DataSet> = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line_item = line.unwrap();
        let split = line_item.split(separator);
        let parts: Vec<&str> = split.collect();
        if idx == 0 {
            // First line shows the dataset names
            for column in parts {
                data_sets.push(DataSet { name: String::from(column), values: Vec::new()});
            }
            continue;
        }

        for (col_idx, input_value) in parts.iter().enumerate() {
            data_sets[col_idx].values.push(String::from(input_value.clone()));
        }
    }

    Ok(data_sets)
}

/// Display the read data set values to the screen. This will just display the list of
/// all values read with the data set name read by the columns in the input file
fn show_read_datasets(data_sets: Vec<DataSet>) {
    let data_set_count = data_sets.len();
    println!("{} data sets read from input file", data_set_count);

    for cur_data_set in data_sets {
        for (idx, value) in cur_data_set.values.iter().enumerate() {
            let output_line: String;
            if idx == 0 {
                output_line = format!("{}\t{}", &cur_data_set.name, &value);
            }
            else {
                output_line = format!("{}\t{}", " ", &value);
            }
            println!("{}", output_line);
        }
    }
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

