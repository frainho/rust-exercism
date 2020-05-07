pub fn is_armstrong_number(num: u32) -> bool {
    let string_number = num.to_string();
    let power = string_number.len() as u32;
    let sum: u32 = string_number
        .chars()
        .map(|char| char.to_digit(10).unwrap().pow(power))
        .sum();
    sum == num
}
