fn main() {
    let input = include_str!("../input");

    let mut cals = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| food.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();
    cals.sort_by(|a, b| b.cmp(a));
    println!("{:?}", cals.iter().take(3).sum::<i32>());
}
