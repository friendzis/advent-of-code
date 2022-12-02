fn main() {
    let input = include_str!("../input");

    let max = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| food.parse::<i32>().unwrap())
                .sum()
        })
        .fold(i32::MIN, |acc, el| if el > acc { el } else { acc });
    println!("{0}", max);
}
