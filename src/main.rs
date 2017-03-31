mod boom;
extern crate regex;
use regex::Regex;

#[macro_use] extern crate text_io;


fn is_small(n: i32) -> bool { n <= 0 }
fn is_big(n:i32) -> bool { n > 10i32.pow(7) }

fn main() {
    // read until a whitespace and try to convert what was read into an i32
     // Infinite loop
    println!("use `quit` to exit from program"); 
    loop {
        println!("Enter Boom Number : ");
        let line: String = read!();
        match &line as &str {
            "quit" => { 
                println!("Bye bye ! "); 
                break 
             },
            y => match boom::converter::parse_int(&y) {
                Ok(n) => {
                    if is_small(n) || is_big(n) {
                            println!("Value is not between {}-{}", 0, 10i32.pow(7));
                            continue;
                    } else {
                        // shift 1 and get integer as bits
                        let formatted_result = format!("{:b}", n + 1);
                        // regex to replace from 0 to something
                        let s1_re = Regex::new(r"0").unwrap();
                        // regex to replace from 1 to something
                        let s2_re = Regex::new(r"1").unwrap();
                        // get length of bits
                        let len = formatted_result.len();
                        // remove first character.
                        let slice: String = formatted_result.chars().skip(1).take(len - 1).collect();
                        // replace all 0 character as 2
                        let replaced_result = s1_re.replace_all(&slice, "2");
                        // replace all 1 character as 3
                        println!("Result: {}", s2_re.replace_all(&replaced_result, "3")); // => "xxxxx xxxxx!"

                    }
                },
                Err(err) => println!("Value is not a number !"),
            }
        }  
    }
}



