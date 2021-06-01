extern crate nucleotide_count as nucleo;

#[cfg(test)]
mod tests
{
    use std::collections::HashMap;

    #[test]
    fn invalidInputString()
    {
            let mys = "WDBCGCCTPERO";
            assert!(nucleo::countDNA(&mys).is_err());
    }
    #[test]
    fn invalidInputOnlyOneString()
    {
        let mys = "AAAAAAAAAAAAA";
        assert!(nucleo::countDNA(&mys).is_err());

    }
    #[test]
    fn validInput()
    {
        let mys = "AGCTTT";
        let mut output = HashMap::new();
        output.insert('A',1);
        output.insert('G',1);
        output.insert('C',1);
        output.insert('T',3);
        assert_eq!(nucleo::countDNA(&mys).unwrap(),output );

    }
    #[test]
    fn validInputSmallLetters()
    {
        let mys = "aaccgctt";
        assert!(nucleo::countDNA(&mys).is_ok() );

    }
}