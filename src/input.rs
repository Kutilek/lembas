use std::io;

pub fn input() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong");

    return input;
}
