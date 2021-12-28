use helper::helper::Process;
use std::time::Instant;

fn main() {
    let is_sample = false;

    let d01 = day_01::Challenge {};
    let d02 = day_02::Challenge {};
    let d03 = day_03::Challenge {};
    let d04 = day_04::Challenge {};
    let d05 = day_05::Challenge {};
    let d06 = day_06::Challenge {};
    let d07 = day_07::Challenge {};
    let d08 = day_08::Challenge {};
    let d09 = day_09::Challenge {};
    let d10 = day_10::Challenge {};
    let d11 = day_11::Challenge {};
    let d12 = day_12::Challenge {};
    let d13 = day_13::Challenge {};
    let d14 = day_14::Challenge {};
    let d15 = day_15::Challenge {};
    let d16 = day_16::Challenge {};
    let d17 = day_17::Challenge {};
    let d18 = day_18::Challenge {};
    let d19 = day_19::Challenge {};
    let d20 = day_20::Challenge {};

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

    println!("Done in {} milliseconds", now.elapsed().as_millis());
}
