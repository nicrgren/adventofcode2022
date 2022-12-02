use anyhow::Result;

mod day01;
mod day02;

fn main() -> Result<()> {
    println!("Day01");

    let day01 = read_input_file("day01.txt")?;
    println!("Day01:");
    day01::solve(&day01);

    let day02 = read_input_file("day02.txt")?;
    println!("Day02:");
    day02::solve(&day02);
    Ok(())
}

fn read_input_file(fname: &str) -> Result<String> {
    let s = std::fs::read_to_string(format!("inputs/{}", fname))?;
    Ok(s)
}
