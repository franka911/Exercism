extern crate anagram as ana;


#[cfg(test)]
mod tests
{
    #[test]
    fn checkEmpty(){
        let mystring = "listen";
        let mut mylist = [""];
        assert!(ana::convert(&mystring, &mut mylist).unwrap().is_empty());
    }

    #[test]
    fn checkListen(){
        let mystring = "listen";
        let mut mylist = ["enlists", "google", "inlets", "banana"];
        assert_eq!(ana::convert(&mystring, &mut mylist).unwrap(), vec!["inlets"]);

    }


    #[test]
    fn checkHello(){
        let mystring = "hello";
        let mut mylist = ["hello", "olleh"];
        assert_eq!(ana::convert(&mystring, &mut mylist).unwrap(), vec!["olleh"]);

    }


    #[test]
    fn checkBanana(){
        let mystring = "banana";
        let mut mylist = ["bAnana"];
        assert!(ana::convert(&mystring, &mut mylist).unwrap().is_empty() );

    }

    #[test]
    fn checkAllergy(){
        let mystring = "allergy";
        let mut mylist = ["gallery", "ballerina", "regally", "clergy", "largely", "leading",];
        assert_eq!(ana::convert(&mystring, &mut mylist).unwrap(), vec!["gallery","regally", "largely"] );

    }

    #[test]
    fn checkGalea(){
        let mystring = "galea";
        let mut mylist = ["eagle"];
        assert!(ana::convert(&mystring, &mut mylist).unwrap().is_empty() );

    }

}