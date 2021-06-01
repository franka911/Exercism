extern crate binary_search as binary;

#[cfg(test)]
pub mod tests
{
    #[test]
    fn testOne(){
        let myNumbers = vec![3,8,21,345,52,24,424,3];
        let index = binary::bs(&myNumbers, 424).unwrap();
        assert_eq!(index,6)
    }


    #[test]
    fn testTwo(){
        let myNumbers = vec![3,8,21,35,52,24,424,3];
        let index = binary::bs(&myNumbers, 1);
        assert_eq!(index,None)
    }

    #[test]
    fn testEmpty(){
        let myNumbers = vec![];
        let index = binary::bs(&myNumbers, 1);
        assert_eq!(index,None)
    }


    #[test]
    fn testMiddle(){
        let myNumbers = vec![3,8,21,35,52,24,424,3];
        let index = binary::bs(&myNumbers, 35).unwrap();
        assert_eq!(index,3)
    }
}