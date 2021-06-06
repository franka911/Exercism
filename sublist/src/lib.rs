#[derive(Debug,PartialEq)]
pub enum Results{
    Sublist,
    Equal,
    Superlist,
    NotEqual
}

pub fn compareLists<T:PartialEq>(tableList1: &[T], tableList2: &[T] ) -> Results {
    if tableList1 == tableList2 {
        return Results::Equal
    }
    else if tableList1.len() < tableList2.len() {
        if tableList1.is_empty() {
            return Results::Sublist
        }
        else {
            match tableList2.windows(tableList1.len()).any(|x| x == tableList1) {
                true => return Results::Sublist,
                false => return Results::NotEqual,
            }
        }

    }
    else if tableList1.len() > tableList2.len(){
            if tableList2.is_empty() {
                return Results::Sublist
            }
            else {
                match tableList1.windows(tableList2.len()).any(|x| x == tableList2){
                    true => return Results::Superlist,
                    false => return Results::NotEqual,
                }
            }
    }

    Results::NotEqual
}

