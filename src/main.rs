mod challenges;

fn main() {
    let day_1 = challenges::day_1::Day1::new();
    println!("###### Day 1 #######");
    day_1.run();

    let day_2 = challenges::day_2::Day2::new();
    println!("###### Day 2 #######");
    day_2.run();

    let day_3 = challenges::day_3::Day3::new();
    println!("###### Day 2 #######");
    day_3.run();

    let day_4 = challenges::day_4::Day4::new();
    println!("###### Day 4 #######");
    day_4.run();

    let day_5 = challenges::day_5::Day5::new();
    println!("###### Day 5 #######");
    day_5.run();
}
