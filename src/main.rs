extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io;

#[derive(Debug, Deserialize)]
struct UtilityBills {
    date: String,
    electric: i32,
    gas: i32,
    communication: i32,
    water: i32
}

trait HasSum {
    fn make_sum(&self) -> i32;
}

impl HasSum for UtilityBills {
    fn make_sum(&self) -> i32 {
        let sum = self.electric + self.gas + self.communication + self.water;
        sum
    }
}

fn main() {
    // input data from stdin
    let mut input_bills = csv::Reader::from_reader(io::stdin());

    // process rows one by one
    for row in input_bills.deserialize() {
        let bills: UtilityBills = row.expect("Invalid row");

        println!("{:?}",&bills);
        println!("{} -> total bill is {}", &bills.date ,&bills.make_sum());
    }
}
