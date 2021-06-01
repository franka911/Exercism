
extern crate all_your_base as base;
#[cfg(test)]
mod tests
{
    #[test]
    fn invalidbaseinput()
    {
        let number = 10u32;
        let baseinput = 0u32;
        let baseoutput = 4u32;
        let result = base::convertNumber(&[number], baseinput, baseoutput);
        assert!(result.is_err());
    }

    #[test]
    fn invalidbaseoutput()
    {
        let number = 10u32;
        let baseinput = 4u32;
        let baseoutput = 0u32;
        let result = base::convertNumber(&[number], baseinput, baseoutput);
        assert!(result.is_err());
    }

    #[test]
    fn binaryToDecimal()
    {
        let number:&[u32] = &[1, 1, 1];
        let baseinput = 2u32;
        let baseoutput = 10u32;
        let result = base::convertNumber(number, baseinput, baseoutput).unwrap();
        assert_eq!(result, vec![7]);
    }

    #[test]
    fn decimalToBinary()
    {
        let number:&[u32] = &[10];
        let baseinput = 10u32;
        let baseoutput = 2u32;
        let result = base::convertNumber(number, baseinput, baseoutput).unwrap();
        assert_eq!(result, vec![1,0,1,0]);
    }

    #[test]
    fn numberZero()
    {
        let number:&[u32] = &[0];
        let baseinput = 10u32;
        let baseoutput = 2u32;
        let result = base::convertNumber(number, baseinput, baseoutput).unwrap();
        assert_eq!(result, []);
    }
}
