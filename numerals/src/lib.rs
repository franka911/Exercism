#![allow(warnings)]

const LITERALS: [&str; 7] =["I", "V","X", "L", "C", "D", "M"];

pub struct Romans(u32);

impl std::fmt::Display for Romans{
    fn fmt (&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
        let mut roman_number = String::new();
        let mut devide = 0u32;
        let mut tnumber = self.0.clone();
        let mut i= 1000u32;
        while tnumber > 0 {

            devide = tnumber / i;
            tnumber = tnumber % i;
            self.matchNumber(&devide, i, &mut roman_number);

            i = i / 10;
            if i == 0 {
                break;
            }
        }
        write!(f, "{}", roman_number)
    }
}
impl Romans {
    fn match_roman(devide: &u32, integer: usize, roman_number: &mut String) {
        match devide {
            1..=3 => roman_number.push_str(&LITERALS[integer].repeat(*devide as usize)),
            4 => roman_number.push_str(&(format!("{}{}", LITERALS[integer], LITERALS[integer+1]))),
            5 => roman_number.push_str(&LITERALS[integer+1]),
            6..=8 => roman_number.push_str(&(format!("{}{}", LITERALS[integer+1], LITERALS[integer].repeat((*devide - 5) as usize)))),
            9 => roman_number.push_str(&(format!("{}{}", LITERALS[integer], LITERALS[integer+2]))),
            _ => {}
        };
    }


    fn matchNumber(&self, devide: &u32, integer: u32, roman_number: &mut String)
    {
        match integer {
            1000 => roman_number.push_str(&LITERALS[6].repeat(*devide as usize)),
            100 => Romans::match_roman(&devide, 4, roman_number),
            10 => Romans::match_roman(&devide, 2, roman_number),
            1 => Romans::match_roman(&devide, 0, roman_number),
            _ => {}
        }
    }
}

impl From<u32> for Romans{
    fn from(number: u32) -> Self{
        Romans(number)
    }
}



//roman formula
/*fn main(){
    let  number = 2021u32;
    println! ("function: {:?}", Romans::from(number).to_string());
}*/

