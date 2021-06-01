extern crate testowy as l;

#[cfg(test)]
mod tests{
     #[test]
     fn testOne()
     {
         assert_eq!(l::Romans::from(1990).to_string(), "MCMXC".to_string());
     }
    #[test]
    fn testTwo()
    {
        assert_eq!(l::Romans::from(2008).to_string(), "MMVIII".to_string());
    }

    #[test]
    fn testThree()
    {
        assert_eq!(l::Romans::from(0).to_string(), "".to_string());
    }
    #[test]
    fn testFour()
    {
        assert_eq!(l::Romans::from(4).to_string(), "IV".to_string());
    }
    #[test]
    fn testFive()
    {
        assert_eq!(l::Romans::from(50).to_string(), "L".to_string());
    }
}