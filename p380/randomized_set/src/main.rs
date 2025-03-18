use randomized_set::RandomizedSet;

fn main() {
    let mut set = RandomizedSet::new();

    println!("Insert 1: {}", set.insert(1));
    println!("Remove 2: {}", set.remove(2));
    println!("Insert 2: {}", set.insert(2));
    println!("Random: {}", set.get_random());

    println!("Remove 1: {}", set.remove(1));
    println!("Insert 2: {}", set.insert(2));
    println!("Random: {}", set.get_random());
}