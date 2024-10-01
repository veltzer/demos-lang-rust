use std::collections::HashMap;

fn main() {
    // Create two hash maps
    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("apple".to_string(), 1);
    map1.insert("banana".to_string(), 2);
    map1.insert("cherry".to_string(), 3);

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("banana".to_string(), 4);
    map2.insert("date".to_string(), 5);
    map2.insert("elderberry".to_string(), 6);

    println!("Map 1: {:?}", map1);
    println!("Map 2: {:?}", map2);

    // Method 1: Using extend (overwrites values from map1 with those from map2)
    let mut unified1 = map1.clone();
    unified1.extend(map2.iter().map(|(k, v)| (k.clone(), *v)));
    println!("\nUnified using extend: {:?}", unified1);

    // Method 2: Using insert with entry API (allows custom logic for conflicts)
    let mut unified2 = map1.clone();
    for (key, value) in map2.iter() {
        unified2.entry(key.clone()).and_modify(|e| *e += *value).or_insert(*value);
    }
    println!("Unified using entry API (sum on conflict): {:?}", unified2);

    // Method 3: Using iterators and collect (overwrites values from map1 with those from map2)
    let unified3: HashMap<_, _> = map1.clone().into_iter().chain(map2.clone().into_iter()).collect();
    println!("Unified using iterators and collect: {:?}", unified3);

    // Method 4: Using iterators with fold (allows custom logic for conflicts)
    let unified4 = map1.iter().chain(map2.iter()).fold(HashMap::new(), |mut acc, (k, v)| {
        acc.entry(k.clone()).and_modify(|e| *e = std::cmp::max(*e, *v)).or_insert(*v);
        acc
    });
    println!("Unified using fold (max value on conflict): {:?}", unified4);

    // Demonstrate that original maps are still available
    println!("\nOriginal Map 1 (still available): {:?}", map1);
    println!("Original Map 2 (still available): {:?}", map2);
}
