use regex::Regex;

/// Rust string are sequences of unicode characters, 
/// but they are not stored in meoeory as arrays of chars
/// Insted they are stored using UTF-* , a varible-width encoding
// Each ASCII character in a string is one byte 
fn string_in_memory() {
    let noodles = "noodles".to_string();
    let oddles = &noodles[1..];
    let poodles  = "fkdfjsddc'sldf";
    println!("{}\n{}\n{}", noodles, oddles, poodles);
    
}

fn main() {
    let speech: &str = "freind-1: Ai will replace software developers \nfriend-2: Nah bro, Ai can't adjust as human brain can do \nfreind-1: Don't be fool bro i have tried the GPT-4o yesterday \nand it can detect bug in my code \nfreind-2: Ok bro lets talk about science around these models, \nthese models use normalization technique and these models can return the output data \n which is at mean and at mean there is most of craapy code is present you will never found good code and algorithum at mean \nfriend-1: Yah bro now i understand why you were so chill \nthis means i just have to work on my programming skills and thinking";
    println!("{}", speech);

    println!();
    

    // Let's work on regregx patterns
    let pattern = Regex::new(r"Hello (?<name>\w+)!").unwrap();
    let Some(caps) = pattern.captures("Hello World!") else {
        println!("No match");
        return;
    };
    println!("The first word every programmer in every language: {}", &caps["name"]);

    // Date pattern with 2024-01-01
    // Let's break down regex before writing down
    // The date should start from year and for that we use ^ to start from any char
    // \d{n} -> contain numer of letter or char 
    // So the output regrex will look like '^\d{4}-\d{2}-\d{2}$'
    let date_pattern = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert_eq!(date_pattern.is_match("2024-01-01"), true);
    let sample_date: &str = "2024-01-01";
    println!("{}", date_pattern.is_match(sample_date));
 
    // Let's talk about byte string
    // Its just ASCII reprsentation of data
    // [&u8] -> DataType
    let sample_string1 = b"Bytes Sting";
    println!("{:?}", sample_string1);

    string_in_memory();
}
