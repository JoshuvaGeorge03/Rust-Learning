use std::io::{self, Write};

const CELSIUS: &str = "celsius";

const KELVIN: &str = "kelvin";

const FAHRENHEIT: &str = "fahrenheit";

const DEFAULT_INPUT_UNIT: &str = CELSIUS;

const DEFAULT_OUTPUT_UNIT: &str = FAHRENHEIT;

fn main() {
    const LIST_OF_TEMPERATURE_UNITS: [&str; 3] = [CELSIUS, KELVIN, FAHRENHEIT];
    let mut temperature_input = String::new();

    print!("Please enter the temperature input value  :  ");

    io::stdout()
    .flush().expect("std::io failed");

    io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to enter the value");

    let temperature_input: f64 = temperature_input
        .trim()
        .parse()
        .expect("Please type a number");

    let input_unit = get_temp_input_unit(
        LIST_OF_TEMPERATURE_UNITS,
        LIST_OF_TEMPERATURE_UNITS.len() + 1,
    );

    if input_unit == CELSIUS {
        let temp_output_unit = get_templ_output_unit(
            LIST_OF_TEMPERATURE_UNITS,
            LIST_OF_TEMPERATURE_UNITS.len() + 1,
        );

        if temp_output_unit == FAHRENHEIT {
            let output_value = convert_temp_from_celsius_to_fahrenheit(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        } else {
            let output_value = convert_temp_from_celsius_to_kelvin(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        }
        // println!("")
    } else if input_unit == FAHRENHEIT {
        let temp_output_unit = get_templ_output_unit(
            LIST_OF_TEMPERATURE_UNITS,
            LIST_OF_TEMPERATURE_UNITS.len() + 1,
        );

        if temp_output_unit == CELSIUS {
            let output_value = convert_temp_from_fahrenheit_to_celsius(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        } else {
            let output_value = convert_temp_from_fahrenheit_to_kelvin(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        }
    } else {
        let temp_output_unit = get_templ_output_unit(LIST_OF_TEMPERATURE_UNITS, LIST_OF_TEMPERATURE_UNITS.len() + 1);
        if temp_output_unit == CELSIUS {
            let output_value = convert_temp_from_kelvin_to_celsius(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        } else {
            let output_value = convert_temp_from_kelvin_to_fahrenheit(temperature_input);
            println!("The converted value for the input unit {input_unit} with the value of {temperature_input} to converted output unit of {} with the value {}", temp_output_unit, output_value );
        }
    }
}

fn get_temp_input_unit(list_of_units: [&str; 3], index: usize) -> &str {
    if index < list_of_units.len() {
        return list_of_units[index];
    }
    return DEFAULT_INPUT_UNIT;
}

fn get_templ_output_unit(list_of_units: [&str; 3], index: usize) -> &str {
    if index < list_of_units.len() {
        return list_of_units[index];
    }
    // implicit return by rust, if no semi colon at the end of expression,
    // hence therer is no need for return keyword
    DEFAULT_OUTPUT_UNIT
}

fn convert_temp_from_celsius_to_fahrenheit(celsius: f64) -> f64 {
    // this is trait impl to convert integer to float
    let fahrenheit = (celsius * 1.8) + f64::from(32);
    return fahrenheit;
}

fn convert_temp_from_celsius_to_kelvin(celsius: f64) -> f64 {
    let kelvin = celsius + 273.15;
    return kelvin;
}

fn convert_temp_from_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // this is casting to convert integer to float 64 bits
    (fahrenheit - 32 as f64) / 1.8
}

fn convert_temp_from_fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    let celsius = convert_temp_from_fahrenheit_to_celsius(fahrenheit);
    let kelvin = convert_temp_from_celsius_to_kelvin(celsius);
    return kelvin;
}

fn convert_temp_from_kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn convert_temp_from_kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    convert_temp_from_celsius_to_fahrenheit(convert_temp_from_kelvin_to_celsius(kelvin))
}
