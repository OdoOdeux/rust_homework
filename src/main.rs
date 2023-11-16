fn main() {
    let string1 = "Hello, world!";
    let string2 = "How are you?";

    let concatenated_string = concatenate_strings( string1,string2 );

    println!("{}", concatenated_string);

}

fn concatenate_strings(s: &str, c: &str) -> String {
    let mut result = String::new();
    result.push_str(s);
    result.push_str(c);
    result
}