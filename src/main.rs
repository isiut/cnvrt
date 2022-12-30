pub mod mass;
use crate::mass::check_mass;
use crate::mass::mass_convert;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Error, Incorrect number of arguments");
        println!("Correct usage: cnvrt <from> <to> <value>");

        return;
    }

    // Two variables necessary because memory is dropped before it is used
    let from = args[1].to_string().trim().to_lowercase();
    let from = from.as_str();

    let to = args[2].to_string().trim().to_lowercase();
    let to = to.as_str();

    // TODO: Handle input not convertible to f64
    let value: f64 = args[3].trim().parse().expect("Error");
    let converted_value;

    if check_mass(from, to) {
        converted_value = mass_convert(from, to, value);
    } else {
        println!("Invalid unit(s) or units are not of the same type.");
        return;
    }

    if converted_value.0 == false {
        println!("An error occured during the conversion.");
        return;
    }

    println!(
        "Please note: some values, especially ones converted between unit \
        systems may return approximate results instead of exact results"
    );
    println!("{} {} = {} {}", value, from, converted_value.1, to);
}
