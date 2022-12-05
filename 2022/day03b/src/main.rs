const ALPHABET: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn common_item(group: &[&str]) -> char {
    println!("{:?}", group);
    for c1 in group[0].chars() {
        for c2 in group[1].chars() {
            if c1 == c2 && group[2].contains(c1) {
                return c1;
            }
        }
    }
    // Should have some kind of wrapping
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

    let score: u32 = input.lines()
    .collect::<Vec<&str>>()
    .chunks(3)
    .map(|chunk| { common_item(chunk) })
    .map(|item| { score_item(item) })
    .map(|i| u32::from(i))
    .sum::<u32>()
    ;
    
    println!("{:?}", score);
}
