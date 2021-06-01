#![allow(warnings)]
extern crate space_age;


#[cfg(test)]
mod tests{

    use space_age::*;

    fn difference(actual: f64, expected: f64)
    {
        let difference = (expected - actual).abs();
        let delta: f64 = 0.01;
        if difference > delta {
            panic!("Value not good")
        }
    }
    #[test]
    fn mercuryAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Mercury::calc_age(&age),131.57);
    }

    #[test]
    fn venusAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Venus::calc_age(&age),51.5);

    }

    #[test]
    fn earthAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Earth::calc_age(&age),31.69);

    }

    #[test]
    fn marsAge()
    {
        let age = AgeSeconds::from(1000000000);
        difference(Mars::calc_age(&age), 16.85);
    }

    #[test]
    fn jupiterAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Jupiter::calc_age(&age), 2.67);

    }

    #[test]
    fn saturnAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Saturn::calc_age(&age), 1.08);

    }

    #[test]
    fn uranusAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Uranus::calc_age(&age), 0.38);

    }

    #[test]
    fn neptuneAge()
    {
        let age = AgeSeconds::from (1000000000);
        difference(Neptune::calc_age(&age), 0.19);


    }
}