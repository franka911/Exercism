#![allow(warnings)]
extern crate highScores as hscores;



#[cfg(test)]
mod tests
{
    #[test]
    fn emptyVec(){
        let mut myScores = vec![];
        let mut highScore = hscores::HighScores::new(&mut myScores);
        assert!(highScore.returnLastScore().is_none());
        assert!(highScore.returnLastScore().is_none());
        //assert_eq!(highScore.return3HighScores(), []);
    }

    #[test]
    fn highScore(){
        let mut myScores = vec![300, 150, 19, 40, 70];
        let mut highScore = hscores::HighScores::new(&mut myScores);
        assert_eq!(highScore.returnHighScore(), Some(&300));
    }


    #[test]
    fn high3Score(){
        let mut myScores = vec![300, 150, 19, 40, 70];
        let mut highScore = hscores::HighScores::new(&mut myScores);
        assert_eq!(highScore.return3HighScores(), vec![70,150,300]);

    }


    #[test]
    fn lastScore(){
        let mut myScores = vec![300, 150, 19, 40, 70];
        let mut highScore = hscores::HighScores::new(&mut myScores);
        assert_eq!(highScore.returnLastScore(), Some(&70));

    }
}