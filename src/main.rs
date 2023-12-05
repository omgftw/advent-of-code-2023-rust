mod day1;
mod day2;
#[cfg(test)]
mod tests;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    single: bool,
    #[arg(long)]
    day1: bool,
    #[arg(long)]
    day2: bool,
}

fn main() {
    let args = Args::parse();

    if !args.single || args.day1 {
        // Day 1 Part 1
        let day1_part1 = day1::day1(false);
        println!("Day 1 Part 1: {}", day1_part1);
        // Day 2 Part 2
        let day1_part2 = day1::day1(true);
        println!("Day 1 Part 2: {}", day1_part2);
    }
    if !args.single || args.day2 {
        // Day 2 Part 1
        let day2_part1 = day2::day2(12, 13, 14, None);
        println!("Day 2 Part 1: {:?}", day2_part1);
    }
}
