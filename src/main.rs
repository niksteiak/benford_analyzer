use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::string::String;
use std::collections::HashMap;

mod argparser;

struct DataSet {
    name: String,
    values: Vec<String>
}

struct BenfordAnalysis {
    data_set_name: String,
    distribution: HashMap<String, u32>,
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

    let mut analyses: Vec<BenfordAnalysis> = Vec::new();
    if !input_file.is_empty() {
        println!("Input file is: {}", input_file);
        let data_items = read_file(input_file, &separator);

        for cur_data_set in data_items.unwrap() {
            let cur_analysis = analyze_data_set(cur_data_set);
            analyses.push(cur_analysis);
        }
    }

    for cur_analysis in analyses {
        show_read_dataset(cur_analysis);
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

fn analyze_data_set(data_set: DataSet) -> BenfordAnalysis {
    let mut occurences: HashMap<String, u32> = HashMap::new();

    let total_values = data_set.values.len() as f32;

    println!("{} total values for dataset {}", total_values, data_set.name);

    // Go through all the values in the data set get the first digit and count
    // how many items start with the same digit by adding them to the count
    for item in data_set.values {
        if item.len() == 0 {
            continue;
        }

        let first_digit = String::from(&item[0..1]);
        let digit_count = occurences.entry(first_digit).or_insert(0);
        *digit_count += 1;
    }

    let mut distribution: HashMap<String, u32> = HashMap::new();

    for digit in 1..10 {
        let occur_index = format!("{}", digit);
        println!("digit: {}", occur_index);
        let digit_percentage: u32 = match occurences.get(&occur_index) {
            Some(totals) => {

                let digit_totals = *totals as f32;

                let percentage = digit_totals / total_values * 100f32;
                percentage as u32
            },
            None => {
                0u32
            }
        };
        distribution.insert(occur_index, digit_percentage);
    }

    let analysis = BenfordAnalysis { data_set_name: data_set.name.clone(),
        distribution: distribution };

    analysis
}

/// Display the Analyzed data set
///
fn show_read_dataset(analyzed_set: BenfordAnalysis) {
    println!("Dataset Name: {}", analyzed_set.data_set_name);

    for digit in 1..10 {
        let digit_index = format!("{}", digit);

        let digit_percentage = match analyzed_set.distribution.get(&digit_index) {
            Some(percentage) => { *percentage },
            None => 0u32
        };
        println!("{0}: {1} ({2} %)", digit_index, "*".repeat(digit_percentage as usize), digit_percentage);
    }
    println!("");
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

