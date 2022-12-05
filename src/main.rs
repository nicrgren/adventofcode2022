use anyhow::Result;

mod day01;
mod day02;
mod day03;

fn main() -> Result<()> {
    let day01 = read_input_file("day01.txt")?;
    day01::solve(&day01);

    let day02 = read_input_file("day02.txt")?;
    day02::solve(&day02);

    let day03 = read_input_file("day03.txt")?;
    day03::solve(&day03);

    Ok(())
}

fn read_input_file(fname: &str) -> Result<String> {
    let s = std::fs::read_to_string(format!("inputs/{}", fname))?;
    Ok(s)
}
