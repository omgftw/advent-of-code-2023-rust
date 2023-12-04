use clap::Parser;

mod day1;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    day1: bool,
}

fn main() {
    let args = Args::parse();

    if args.day1 {
        // Day 1 Part 1
        let day1_part1 = day1::day1(false);
        println!("Day 1 Part 1: {}", day1_part1);
        // Day 2 Part 2
        let day1_part2 = day1::day1(true);
        println!("Day 1 Part 2: {}", day1_part2);
    }
}
