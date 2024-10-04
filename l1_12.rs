use std::collections::HashSet;
use std::hash::Hash;

pub fn intersection<T: Hash + Eq>(set_1: HashSet<T>, set_2: HashSet<T>) -> HashSet<T>{
    let mut ans = HashSet::new();
    for elem in set_1{
        if set_2.contains(&elem){
            ans.insert(elem);
        }
    }
    ans
}