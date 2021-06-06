
#![allow(warnings)]
pub struct Points{
    x: i32,
    y: i32,
}

impl Points{
    pub fn new( x: i32, y:i32) -> Option<Self>{
        match (x,y){
            (0..=7, 0..=7) => Some(Self{x:x, y:y}),
            _ => None,
        }

    }
}

pub struct Queen{
    position:Points,
}

impl Queen{
    pub fn new (position: Points)  -> Self{
        Queen{position: position }
    }
}

impl PartialEq for Queen{
    fn eq(&self, other: &Self) -> bool {
        self.position.x == other.position.x || self.position.y == other.position.y || (self.position.x - other.position.x).abs() == (self.position.y - other.position.y).abs()
    }
}

