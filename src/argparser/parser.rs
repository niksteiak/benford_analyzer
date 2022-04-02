use std::collections::HashMap;

struct InputParam {
    arg_name: String,
    param_value: String,
}

pub fn parse_arguments(all_args: Vec<String>) -> HashMap<String, String> {
    let mut parsed_args: HashMap<String, String> = HashMap::new();

    let input_params = to_input_params(all_args);
    for param_item in input_params {
        match param_item.arg_name.as_ref() {
            "i" | "input" => {
                parsed_args.insert(String::from("input_file"), String::from(param_item.param_value));
            },
            "s" | "separator" => {
                parsed_args.insert(String::from("separator"), String::from(param_item.param_value));
            },
            "h" | "help" => {
                parsed_args.insert(String::from("show_help"), String::from("yes"));
            },
            _ => {
            }
        }
    }

    parsed_args
}

/// Convert a set of string values to a Vector of InputParameter items
/// This will check which of the values start with a - or a -- to handle it
/// as a key and the consequent values to see if it is a parameter value or the next key
/// provided by the user
fn to_input_params(all_args: Vec<String>) -> Vec<InputParam> {
    let mut input_params: Vec<InputParam> = Vec::new();

    let mut working_param: Option<InputParam> = None;
    for argument in all_args {
        if argument.starts_with("--") {
            if let Some(item) = working_param {
                input_params.push(item);
            }

            let arg_name = &argument[2..];
            working_param = Some(InputParam {
                arg_name: String::from(arg_name),
                param_value: String::from("") });
        }
        else if argument.starts_with("-") {
            if let Some(item) = working_param {
                input_params.push(item);
            }

            let arg_name = &argument[1..];
            working_param = Some(InputParam {
                arg_name: String::from(arg_name),
                param_value: String::from("") });
        }
        else {
            if let Some(item) = working_param {
                let arg_name = String::from(item.arg_name);
                working_param = Some(InputParam {
                    arg_name: arg_name,
                    param_value: String::from(argument)
                });
            }

            if let Some(item) = working_param {
                input_params.push(item);
                working_param = None
            }
        }
    }

    input_params
}
