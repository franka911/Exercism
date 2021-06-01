
//format, match, unwrap or


fn checkFirstNumber(myString: &str) -> Result<&str,&str> {

    let result = match myString.chars().nth(0).unwrap(){
        '1' => Ok(&myString[1 ..myString.len()]),
        _ => Err("Not a country number")
    };
    result

}


pub fn convert(myphone: &str) -> Result<String, String>
{
    let resulttmp:String = myphone.chars().filter(|ch| ch.is_digit(10)).map(|ch| ch.to_owned()).collect();

    let mut result = String::new();
    if resulttmp.len() == 11 {
            match checkFirstNumber(&resulttmp){
            Ok(v) => return Ok(v.to_string()),
            Err(e) => return Err(e.to_string()),
        }

    }
    else if resulttmp.len() < 10 || resulttmp.len() > 11 {
        return Err("Invalid phone number".to_string())
    }
    else if resulttmp.len() == 10
    {
        result = resulttmp;

    }
    Ok(result)
}


fn main() {
    let myphone = "5613.415.0158";
    println!("result: {:?}",convert(&myphone).unwrap());
}
