use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice.trim().to_lowercase()
}

fn validate_unit(mut unit: String) -> String {
    loop {
        match unit.as_str() {
            "celsius" | "fahrenheit" | "kelvin" => return unit,
            _ => {
                println!("Invalid choice. Please choose Celsius, Fahrenheit, or Kelvin:");
                let mut new_input = String::new();
                io::stdin().read_line(&mut new_input).unwrap();
                unit = new_input.trim().to_lowercase();
            }
        }
    }
}

fn choose_temperature_num() -> f64 {
    println!("Please enter the temperature value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

fn calculating(choice: &str, temperature: f64, result_unit: &str) -> f64 {
    if choice == "celsius" {
        if result_unit == "fahrenheit" {
            temperature * 9.0 / 5.0 + 32.0
        } else if result_unit == "kelvin" {
            temperature + 273.15
        } else {
            temperature
        }
    } else if choice == "fahrenheit" {
        if result_unit == "celsius" {
            (temperature - 32.0) * 5.0 / 9.0
        } else if result_unit == "kelvin" {
            (temperature - 32.0) * 5.0 / 9.0 + 273.15
        } else {
            temperature
        }
    } else if choice == "kelvin" {
        if result_unit == "celsius" {
            temperature - 273.15
        } else if result_unit == "fahrenheit" {
            (temperature - 273.15) * 9.0 / 5.0 + 32.0
        } else {
            temperature
        }
    } else {
        temperature
    }
}

fn main() {
    let choice = read_input("Please choose one of the following common temperature units:\nCelsius\nFahrenheit\nKelvin");
    let choice = validate_unit(choice);

    let result_unit = read_input("Please choose the result unit:\nCelsius\nFahrenheit\nKelvin");
    let result_unit = validate_unit(result_unit);

    println!("Valid choice: {}", choice);
    println!("Valid result unit: {}", result_unit);

    let temperature = choose_temperature_num();
    let final_temperature = calculating(&choice, temperature, &result_unit);

    println!("The converted temperature is: {:.2}", final_temperature);
}
