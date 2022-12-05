const ALPHABET: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn common_item(strings: (&str, &str) ) -> char {
    println!("Finding common items in {:?} and {:?}", strings.0, strings.1);
    for c in strings.0.chars() {
        // println!("Searching for {c} in {:?}", strings.0);
        if strings.1.contains(c) {return c;}
    }
    '\0'
}

fn score_item(item: char) -> u8 {
    println!("Scoring {:?}", item);
    let foo: u8 = ALPHABET.chars().position(|predicate| predicate == item).unwrap().try_into().unwrap();
    println!("Score is {foo}");
    foo
}

fn main() {
    let input = include_str!("../input");

    // Is this efficient? ¯\_(ツ)_/¯
    // Is .map(a).map(b) more readable than .map(b(a))? Yes.
    let foo: u32 = input.lines().map(|rucksack| {
        rucksack.split_at(rucksack.len()/2)
    })
    .map(|i| common_item(i))
    .map(|i| score_item(i))
    .map(|i| u32::from(i))
    .sum()
    ;

    println!("{:?}", foo);
}
