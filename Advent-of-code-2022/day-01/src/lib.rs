pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n") // split based on double newlines
        .map(|elf_load| {
            elf_load
                .lines()
                .filter_map(|item| item.parse::<u32>().ok()) // ignore lines that fail to parse
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n") // split based on double newlines
        .map(|elf_load| {
            elf_load
                .lines()
                .filter_map(|item| item.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    let summ: u32 = result.iter().take(3).sum();
    dbg!(&result);
    summ.to_string()
}

#[cfg(test)]
mod tests {
    use super::*; // gives us access to all the functions in the parent module
    const INPUT: &str = "15559
3906
7076
11980
11508

6558
2256
7294
6566
2686
2566
1724
4811
5427
4278
3756

3761
5599
13187
12558
7425
9269

59658

34175
9643";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        println!("{}", result);
        assert_eq!(result, "59658");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        println!("{}", result);
        assert_eq!(result, "161486");
    }
}
