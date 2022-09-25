use std::process::Stdio;

use clock::Clock;
use daylight_saving::DaylightSaving;
use std::io;
mod daylight_saving;
mod clock;
mod clock_errors;


fn main() {
    let mut clock1 =  match Clock::new(0,5, DaylightSaving::WinterTime) {
        Ok(clock) => clock,
        Err(e) => panic!("{}",e),
    };
    
    let mut clock2 =  match Clock::new(23,9, DaylightSaving::SummerTime) {
        Ok(clock) => clock,
        Err(e) => panic!("{}",e),
    };
    
    let mut clock3 : Box<Clock>;

    clock1.set_daylight_saving(DaylightSaving::SummerTime);

    for i in 0..199 {
        clock2.increment();
    }

    let mut hours_in_str = String::new();
    let mut minutes_in_str = String::new();
    
    println!("Please enter the hours");
    io::stdin().read_line(&mut hours_in_str);
    let hours = hours_in_str.replace("\n", "").parse::<u64>().unwrap();


    println!("Please enter the minutes");
    io::stdin().read_line(&mut minutes_in_str);
    let minutes = minutes_in_str.replace("\n", "").parse::<u64>().unwrap();

    clock3 = Box::new(match Clock::new(hours, minutes, DaylightSaving::SummerTime){
        Ok(clock) => clock,
        Err(_) => Clock::midnight(),
    });

    if(clock2 == *clock3){
        println!("Same time");
    }

    println!("{}", clock1);
    println!("{}", clock2);
    println!("{}", clock3);

}
