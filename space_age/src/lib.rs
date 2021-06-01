#![allow(warnings)]

const mercuryfloat:f64=0.2408467;
const earthfloat:f64 =31557600.0;
const venusfloat:f64=0.61519726;
const marsfloat:f64=1.8808158;
const jupiterfloat:f64=11.862615;
const saturnfloat:f64=29.447498;
const uranusfloat:f64= 84.016846;
const neptunefloat:f64= 164.79132;


#[derive(Debug)]

pub struct AgeSeconds{
    seconds: f64
}


impl From<u64> for AgeSeconds{
    fn from(s:u64) -> Self{
        Self{seconds:s as f64 }
    }
}


pub trait Planet {
     fn calc_age(age:&AgeSeconds) -> f64;
}


#[macro_export]
macro_rules! calcPlanet {
     ($($planet_name:ident, $orbit:expr)+)=> {
          $(
                pub struct $planet_name;
                impl Planet for $planet_name{
                    fn calc_age(age: &AgeSeconds) -> f64{
                        age.seconds / (earthfloat*$orbit)
          }

        })+
     };
 }



calcPlanet!(Mercury, mercuryfloat);
calcPlanet!(Venus, venusfloat);
calcPlanet!(Earth, 1.0);
calcPlanet!(Mars, marsfloat);
calcPlanet!(Jupiter, jupiterfloat);
calcPlanet!(Saturn, saturnfloat);
calcPlanet!(Uranus, uranusfloat);
calcPlanet!(Neptune, neptunefloat);


/*fn main() {

    let value = AgeSeconds::from(2500000);
    let new_value = Uranus::calc_age(&value);
    println!("{:?}",new_value);
}*/
