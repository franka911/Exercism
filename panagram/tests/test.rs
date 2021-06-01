extern crate panagram as pan;


#[cfg(test)]
mod  tests
{
    fn checkTrue()
    {
        let mystring = "The quick brown fox jumps over the lazy dog";
        assert_eq!(pan::checkPanagram(&mystring), true);
    }

    fn checkFalse()
    {
        let mystring = "The quick  jumps over the lazy dog";
        assert_eq!(pan::checkPanagram(&mystring), false);
    }

    fn checkDigit()
    {
        let mystring = "The quick 11 the lazy dog";
        assert_eq!(pan::checkPanagram(&mystring), false);
    }
}