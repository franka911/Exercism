extern crate phone_number as phone;

#[cfg(test)]
mod tests
{
    #[test]
    fn firstPhone(){
        let myPhone = "+1 (613)-995-0253";
        assert_eq!(phone::convert(&myPhone).unwrap(), "6139950253");
    }

    #[test]
    fn secondPhone(){
        let myPhone = "613-995-0253";
        assert_eq!(phone::convert(&myPhone).unwrap(), "6139950253");
    }

    #[test]
    fn thirdPhone(){
        let myPhone = "1 613 995 0253";
        assert_eq!(phone::convert(&myPhone).unwrap(), "6139950253");
    }

    #[test]
    fn fourthPhone(){
        let myPhone = "613.995.0253";
        assert_eq!(phone::convert(&myPhone).unwrap(), "6139950253");
    }

    #[test]
    fn notCountryPhone(){
        let myPhone = "5613.995.0253";
        assert_eq!(phone::convert(&myPhone), Err("Not a country number".to_string()));
    }


    #[test]
    fn tooManyNumbers(){
        let myPhone = "57613-98795-0253";
        assert_eq!(phone::convert(&myPhone), Err("Invalid phone number".to_string()));
    }

}