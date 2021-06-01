use std::collections::HashMap;
use std::fmt::Error;
use std::collections::hash_map::RandomState;


pub fn checkValidDNA(mys: &str) -> Result<(),&str >{
    let nucleo = ['A', 'C','G','T'];
    for index in nucleo.iter(){
            if  !mys.to_uppercase().contains(*index){
                return Err("INVALID")
            };
    }
    Ok(())


}

pub fn countDNA(mys: &str)  -> Result<HashMap<char,usize>,&str> {
    let res = checkValidDNA(mys);

    match res
    {
      Ok(t) => Ok("ACGT".chars().map(|ch| (ch,mys.matches(ch).count())).collect::<HashMap<_,_>>()),
      Err(e) =>  Err(e),
    }

}

fn main() {
    let my_dna= "AGCTTT";
    println!("{:?}",countDNA(&my_dna).unwrap());
}
