use std::ops::Deref;
use std::collections::HashSet;

pub fn checkPanagram(input: &str) -> bool
{
    let mystring = "abcdefghijklmnoprsustqwyzx";
    let new_input= input.to_lowercase();
    mystring.chars().filter(|c| c.is_ascii_alphanumeric()).all(|c| new_input.contains(c))

}

fn main() {
    let input =  "The quick 11 jumps over the lazy dog";
    checkPanagram(&input);
}
