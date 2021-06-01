use std::io::stdin;
#[derive(Debug)]
pub enum Error
{
    InvalidBaseInput,
    InvalidBaseOutput,

}


pub fn convertNumber(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error>
{
    if to_base < 1 {
        return Err(Error::InvalidBaseOutput)
    }
    if from_base < 1 {
        return Err(Error::InvalidBaseInput)
    }


    let result_a = convertFromBase(number,from_base).unwrap();
    let result = convertToBase( result_a, to_base).unwrap();
    Ok(result)

}

fn convertFromBase(number: &[u32], from_base: u32) -> Result<u32, Error>
{

    let digits =number.iter().enumerate().fold(0, |acc, (x, y)| acc + from_base.pow(x as u32) * y);
    Ok(digits)
}

fn convertToBase(number: u32, to_base: u32) -> Result<Vec<u32>, Error>
{

    let mut digits:Vec<u32> = Vec::new();
    let mut n= number;
    while n > 0u32
    {
        digits.push(n% to_base);
        n /= to_base;
    }

    digits.reverse();
    Ok(digits)

}



fn main() {
    println!("Give number and base");
    let mut number_bases = String::new();
    stdin().read_line(&mut number_bases).unwrap();
    let received_numbers: Vec<_> = number_bases.split(" ").map(|x| x.trim()).collect();

    let numberdigits:Vec<_> = received_numbers[0].chars().map(|a| a.to_digit(10).unwrap()).collect();
    let result = convertNumber(&numberdigits,received_numbers[1].parse::<u32>().unwrap(),received_numbers[2].parse::<u32>().unwrap());
    println!("Result: {:?}", result);


}
