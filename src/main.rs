fn main() {
    let number = "0.225"; // The number to be converted
    let from_base = 10; // Base of above number
    let to_base = 8; // Base to convert to

    match convert_base(number, from_base, to_base) {
        Some(result) 
        => 
        println!("{} in base {} is {} in base {}", number, from_base, result, to_base),
        None => println!("Invalid input."),
    }
}

fn convert_base(number: &str, from_base: u32, to_base: u32) -> Option<String> {
    let decimal_value = convert_to_decimal(number, from_base)?;
    Some(convert_from_decimal(decimal_value, to_base))
}

fn convert_to_decimal(number: &str, base: u32) -> Option<f64> {
    let parts: Vec<&str> = number.split('.').collect();

    let integer_part = parts.get(0)?;
    let fractional_part = parts.get(1).unwrap_or(&"");

    let integer_value = convert_integer_part(integer_part, base)?;
    let fractional_value = convert_fractional_part(fractional_part, base)?;

    Some(integer_value + fractional_value)
}

fn convert_integer_part(part: &str, base: u32) -> Option<f64> {
    i64::from_str_radix(part, base).ok().map(|v| v as f64)
}

fn convert_fractional_part(part: &str, base: u32) -> Option<f64> {
    let mut value = 0.0;
    let mut base_power = base as f64;

    for digit in part.chars() {
        let digit_value = digit.to_digit(base)? as f64;
        value += digit_value / base_power;
        base_power *= base as f64;
    }

    Some(value)
}

fn convert_from_decimal(number: f64, base: u32) -> String {
    let integer_part = number.trunc() as u64;
    let fractional_part = number.fract();

    let integer_str = convert_integer_to_base(integer_part, base);
    let fractional_str = convert_fractional_to_base(fractional_part, base);

    format!("{}.{}", integer_str, fractional_str)
}

fn convert_integer_to_base(mut number: u64, base: u32) -> String {
    let mut result = String::new();

    while number > 0 {
        let remainder = number % base as u64;
        result.push(std::char::from_digit(remainder as u32, base).unwrap());
        number /= base as u64;
    }

    if result.is_empty() {
        result.push('0');
    } else {
        result = result.chars().rev().collect();
    }

    result
}

fn convert_fractional_to_base(mut number: f64, base: u32) -> String {
    let mut result = String::new();
    let mut count = 0;
    let precision = 10; // Define the desired precision for the fractional portion.

    while number > 0.0 && count < precision {
        number *= base as f64;
        let digit = number.trunc();
        result.push(std::char::from_digit(digit as u32, base).unwrap());
        number = number.fract();
        count += 1;
    }

    result
}

