
use std::collections:: {HashMap, HashSet};


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests_hashmap() {

        let person_1 = "Adam";
        let person_2 = "Laura";


        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert(person_1, 55);
        results_hm.insert( person_2, 56);

        let test_score = results_hm.get(person_1);
        dbg!(test_score.unwrap());

        if results_hm.contains_key("Laura") {
            println!("Laura is in the hashmap");
        } else {
            println!("Laura is not in the hashmap");
        }
    }

    #[test]
    fn tests_hashset() {

        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Adam");
        names_hs.insert("Laura");
        names_hs.insert("Jane");

        if names_hs.contains("Adam") {
            println!("Jane is in the set");
        } else {
            println!("Jane is not in the set");
        }
    }
}