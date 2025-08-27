# Temperature-Converter-Rust

This Rust program is a command-line temperature converter that allows the user to convert temperatures between Celsius, Fahrenheit, and Kelvin.

The program works in several steps:
Reading Input: The read_input function displays a prompt and reads the user’s input as a string. It trims whitespace and converts the input to lowercase to simplify validation.
Validating Units: The validate_unit function ensures that the user enters a valid temperature unit. If the input is invalid, it repeatedly prompts the user using a loop until a correct unit (celsius, fahrenheit, or kelvin) is entered.
Reading Temperature Value: The choose_temperature_num function reads a numeric temperature value from the user and parses it as a floating-point number (f64).
Performing Conversion: The calculating function converts the temperature from the input unit to the desired output unit using standard formulas:

Celsius ↔ Fahrenheit: F = C * 9/5 + 32, C = (F - 32) * 5/9

Celsius ↔ Kelvin: K = C + 273.15, C = K - 273.15

Fahrenheit ↔ Kelvin: K = (F - 32) * 5/9 + 273.15, F = (K - 273.15) * 9/5 + 32

Program Flow: In main, the program asks the user for the input unit, the output unit, and the temperature value. It validates the units, performs the conversion, and prints the final temperature with two decimal points.
Overall, this program combines input handling, validation, loops, and basic arithmetic in Rust to create a user-friendly CLI temperature converter.
