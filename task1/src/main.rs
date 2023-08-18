fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}",concatenated_string);
}


fn concatenate_strings(a:&str,b:&str) ->String {
    let mut result = String::from(a);
    result.push_str(b);
    result
}