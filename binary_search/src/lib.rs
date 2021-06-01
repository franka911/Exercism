use std::cmp::Ordering;


//slices, option type, traits
//binary search


pub fn bs<T:AsRef<[U]>,U:Ord >(myVec:T, key:U) -> Option<usize>
{
    let my_something = myVec.as_ref();
    if my_something.len() == 0
    {
        return None
    }
    let mid = my_something.len() /2;
    match key.cmp(my_something.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => bs(&my_something[0..mid], key),
        Ordering::Greater => bs(&my_something[mid + 1..], key).map(|pos| pos + 1 + mid),

    }
}




fn main() {
    let x = vec![3,8,21,345,52,24,424,3];
    let index = bs(&x, 424).unwrap();

}
