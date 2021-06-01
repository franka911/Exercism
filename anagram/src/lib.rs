

pub fn  convert<'a>(mystring: &str, mylist: &'a [& str]) -> Option< Vec<&'a str>>{
    let word = sortWords(&(mystring));
    let mut myresult: Vec<&str> = Vec::new();
    for index in mylist.iter(){
        if !index.to_lowercase().contains(&mystring.to_lowercase())
        {
            let second_word = sortWords(index);
            if second_word == word{
                    myresult.push(index)
        }
    }
    }
    Some(myresult)
}

pub fn sortWords (words: &str) -> String {
    let mut vectorlist: Vec<char> = words.to_lowercase().chars().collect();
    vectorlist.sort_unstable();
    let myString:String = vectorlist.into_iter().collect();
    myString
}


//["listen"]
// ["enlists", "google", "inlets", "banana"]
/*fn main() {
    let mystring = "listen";
    let mut mylist = ["enlists", "google", "inlets", "banana"];
    println!("{:?}", convert(&mystring, &mut mylist).unwrap());

}*/
