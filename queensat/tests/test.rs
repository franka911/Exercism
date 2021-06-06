extern crate queensat as queen;


#[cfg(test)]
mod tests{
    use queen::*;

    #[test]
    fn offBoard(){
        assert!(Points::new(-1, 2).is_none());
        assert!(Points::new(8, 8).is_none());
    }

    #[test]
    fn couldAttackSameRow (){
        let queen1 = Queen::new(Points::new(2, 2).unwrap());
        let queen2 = Queen::new(Points::new(2, 4).unwrap());

        assert!(queen1 ==queen2);
    }

    #[test]
    fn couldAttackSameColumn (){
        let queen1 = Queen::new(Points::new(4, 4).unwrap());
        let queen2 = Queen::new(Points::new(2, 4).unwrap());

        assert!(queen1 ==queen2);
    }

    #[test]
    fn couldAttackDiagonal (){
        let queen1 = Queen::new(Points::new(2, 3).unwrap());
        let queen2 = Queen::new(Points::new(5, 6).unwrap());

        assert!(queen1 ==queen2);
    }



}