#![allow(warnings)]
pub struct HighScores <'a>{
    scores: &'a Vec<i32>
}

impl<'a> HighScores<'a> {
    pub fn new(myscores: &'a mut Vec<i32>) -> Self {
       HighScores { scores: myscores }
    }

    pub fn returnHighScore(&mut self) -> Option<&i32> {
            self.scores.iter().max()
    }

    pub fn return3HighScores(&mut self) -> Vec<i32> {
           let mut newVector = self.scores.clone();
           newVector.sort_unstable();
           newVector[newVector.len() -3 ..].to_owned()
    }

    pub fn returnLastScore (&mut self) -> Option<&i32> {
        self.scores.last()
    }

}


// iterators, lifetimes, vectors
/*fn main()
{

    let mut myScores = vec![300, 150, 19, 40, 70];
    let mut highScore = HighScores::new(&mut myScores);
    println! ("{:?}", highScore.return3HighScores());

}
*/