fn main() {
    let int_num:i32 = 95;
    println!("The starting integer is {}.\n", int_num);
    let string_num = int_num.to_string();
    println!("The int converted to string is {}.\n", string_num);
    let string_to_int: i32 = string_num.parse().expect("Failed to parse string to integer");
    println!("The string converted back to int is: {}.\n", string_to_int);
}
