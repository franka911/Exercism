#![allow(warnings)]
extern crate scrable_score as score;


#[cfg(test)]
mod tests{

    #[test]
    fn cabbageTest(){
        assert_eq!(score::calcScrable("cabbage"), 14);
    }

    #[test]
    fn emptyTest(){
        assert_eq!(score::calcScrable(""), 0);
    }

    #[test]
    fn alphabetTest(){
        assert_eq!(score::calcScrable("abcdefghijklmnopqrstuvwxyz"), 87);
    }

    #[test]
    fn oneTest(){
        assert_eq!(score::calcScrable("one"), 3);
    }

    #[test]
    fn oneoneTest(){
        assert_eq!(score::calcScrable("oneone"), 6);
    }
}
