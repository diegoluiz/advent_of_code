use helper::helper::Process;
use std::time::Instant;

fn main() {
    let is_sample = false;

    let mut d01 = day_01::Challenge {};
    let mut d02 = day_02::Challenge {};
    let mut d03 = day_03::Challenge {};
    let mut d04 = day_04::Challenge {};
    let mut d05 = day_05::Challenge {};
    let mut d06 = day_06::Challenge {};
    let mut d07 = day_07::Challenge {};
    let mut d08 = day_08::Challenge {};
    let mut d09 = day_09::Challenge {};
    let mut d10 = day_10::Challenge {};
    let mut d11 = day_11::Challenge {};
    let mut d12 = day_12::Challenge {};
    let mut d13 = day_13::Challenge {};
    let mut d14 = day_14::Challenge {};
    let mut d15 = day_15::Challenge {};
    let mut d16 = day_16::Challenge {};
    let mut d17 = day_17::Challenge {};
    let mut d18 = day_18::Challenge {};
    let mut d19 = day_19::Challenge {};
    let mut d20 = day_20::Challenge {};
    let mut d21 = day_21::Challenge::new();

    let now = Instant::now();
    println!("Starting challenges");

    d01.process(is_sample);
    d02.process(is_sample);
    d03.process(is_sample);
    d04.process(is_sample);
    d05.process(is_sample);
    d06.process(is_sample);
    d07.process(is_sample);
    d08.process(is_sample);
    d09.process(is_sample);
    d10.process(is_sample);
    d11.process(is_sample);
    d12.process(is_sample);
    d13.process(is_sample);
    d14.process(is_sample);
    d15.process(is_sample);
    d16.process(is_sample);
    d17.process(is_sample);
    d18.process(is_sample);
    d19.process(is_sample);
    d20.process(is_sample);
    d21.process(is_sample);

    println!("Done in {} milliseconds", now.elapsed().as_millis());
}
