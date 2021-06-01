
#![allow(warnings)]
pub fn calcScrable(scrabble: &str) -> usize {
        scrabble.to_lowercase().chars().map(|x| {
            match x {
                'a'| 'e'| 'i'| 'o'| 'u'| 'l'| 'n'| 'r'| 's'| 't' => 1,
                'd' | 'g' => 2,
                'b' | 'c' |'m' |'p' => 3,
                'f' | 'h' |'v' |'w'| 'y' => 4,
                'k' => 5,
                'j' | 'x' => 8,
                'q' | 'z' => 10,
                _ => 0
            }
        }).sum()
}

//hashmap  higher order function
/*fn main() {
    let scrabble = "at cabbage one".to_string();
    println! ("sum: {:?}",calcScrable(&scrabble));

}
*/