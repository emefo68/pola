fn main() {
    let word_count = pola_core::parser::get_word_count("Hello, pola!");
    if let Some((key, value)) = word_count.get_key_value("hello") {
        println!("{} is number {}!", key, value);
    } else {
        println!("I think you manipulated the code!");
    };
}
