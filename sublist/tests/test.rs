extern crate sublist as lists;


#[cfg(test)]
mod tests{

    #[test]
    fn emptyTest(){
        let listA: &[u32] = &[];

        assert_eq!(lists::Results::Equal, lists::compareLists(&listA, &listA));
    }

    #[test]
    fn SublistTest(){
        let listA: &[u32] = &[2,3];
        let listB: &[u32] = &[1,2,3,4,5];

        assert_eq!(lists::Results::Sublist, lists::compareLists(&listA, &listB));
    }


    #[test]
    fn NotEqualTest(){
        let listA: &[u32] = &[6,7];
        let listB: &[u32] = &[1,2,3,4,5];

        assert_eq!(lists::Results::NotEqual, lists::compareLists(&listA, &listB));
    }


    #[test]
    fn EqualTest(){
        let listA: &[u32] = &[1,2,3,4,5];
        let listB: &[u32] = &[1,2,3,4,5];

        assert_eq!(lists::Results::Equal, lists::compareLists(&listA, &listB));
    }


    #[test]
    fn superListTest(){
        let listA: &[u32] = &[1,2,3,4,5];
        let listB: &[u32] = &[3,4,5];

        assert_eq!(lists::Results::Superlist, lists::compareLists(&listA, &listB));
    }

}