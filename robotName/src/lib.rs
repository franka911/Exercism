#![allow(warnings)]
//lifetimes, randomness, slices
use rand::{prelude, thread_rng, Rng};
use rand::distributions::Alphanumeric;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITSET: &[u8] =b"123456789";

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self{
        Robot(Robot::genName())
    }

    pub fn getName(&self) -> &str{
        &self.0
    }

    pub fn reset(&mut self){
        self.0 = Robot::genName();
    }


    fn genName() -> String {
        let mut rng = rand::thread_rng();
        let robot_name_first: String = (0..2)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        let robot_name_second: String = (0..3)
            .map(|_| {
                let idx = rng.gen_range(0..DIGITSET.len());
                DIGITSET[idx] as char
            })
            .collect();
        format!("{}{}", robot_name_first, robot_name_second)
    }
}

