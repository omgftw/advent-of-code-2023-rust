mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
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
    #[arg(long)]
    day3: bool,
    #[arg(long)]
    day4: bool,
    #[arg(long)]
    day5: bool,
    #[arg(long)]
    day6: bool,
}

#[tokio::main]
async fn main() {
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
        // Day 2
        let day2 = day2::day2(12, 13, 14, None);
        println!("Day 2 Part 1: {:?}", day2.0);
        println!("Day 2 Part 2: {:?}", day2.1);
    }
    if !args.single || args.day3 {
        // Day 3
        let day3_part1 = day3::day3(None);
        println!("Day 3 Part 1: {}", day3_part1.0);
        println!("Day 3 Part 2: {}", day3_part1.1);
    }
    if !args.single || args.day4 {
        // Day 4
        let day4 = day4::day4(None);
        println!("Day 4 Part 1: {}", day4.0);
        println!("Day 4 Part 2: {}", day4.1);
    }
    if !args.single || args.day5 {
        // Day 5
        let day5 = day5::day5(None);
        println!("Day 5 Part 1: {}", day5.0);
        println!("Day 5 Part 2: {}", day5.1);
    }
    if !args.single || args.day6 {
        // Day 6
        let day6 = day6::day6(None).await;
        println!("Day 6 Part 1: {}", day6.0);
        println!("Day 6 Part 2: {}", day6.1);
    }
}
