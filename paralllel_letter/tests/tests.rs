extern crate paralllel_letter as pletter;

#[cfg(test)]
mod tests
{
    #[test]
    fn emptyInput(){

        assert!(pletter::countLetters(&[]).is_err());
    }


    #[test]
    fn singleInput() {
        static text: [&str; 1] = ["abs defg eef"];
        let output: Vec<(char,usize)> = vec![('a', 1), ('b', 1), ('d', 1), ('e', 3), ('f', 2), ('g', 1), ('s', 1)];
        assert_eq!(pletter::countLetters(&text).unwrap(), output);
    }

    #[test]
    fn oneLetter() {
        static text: [&str; 1] = ["a"];
        let output: Vec<(char,usize)> = vec![('a', 1)];
        assert_eq!(pletter::countLetters(&text).unwrap(), output);
    }

    #[test]
    fn caseSensivityLetter() {
        static text: [&str; 1] = ["aAaA"];
        let output: Vec<(char,usize)> = vec![('a', 4)];
        assert_eq!(pletter::countLetters(&text).unwrap(), output);
    }

    #[test]
    fn withoutNumbers() {
        static text: [&str; 1] = ["aAaA123 355bcd"];
        let output: Vec<(char,usize)> = vec![('a', 4), ('b',1),('c',1),('d',1)];
        assert_eq!(pletter::countLetters(&text).unwrap(), output);
    }

    #[test]
    fn multipleInput() {
        static text: [&str;9]= [
            "Wilhelmus van Nassouwe",
            "ben ik, van Duitsen bloed,",
            "den vaderland getrouwe",
            "blijf ik tot in den dood.",
            "Een Prinse van Oranje",
            "ben ik, vrij, onverveerd,",
            "den Koning van Hispanje",
            "heb ik altijd geëerd.",
            "ke he we wr te"];
        let output: Vec<(char,usize)> = vec![('a', 10), ('b', 5), ('d', 12), ('e', 27),
                                             ('f', 1), ('g', 3), ('h', 4), ('i', 13), ('j', 5),
                                             ('k', 6), ('l', 6), ('m', 1), ('n', 20), ('o', 9),
                                             ('p', 2), ('r', 9), ('s', 6), ('t', 6), ('u', 4),
                                             ('v', 8), ('w', 5), ('ë', 1)];
        assert_eq!(pletter::countLetters(&text).unwrap(), output);
    }
}
