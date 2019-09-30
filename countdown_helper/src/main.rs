#![allow(unconditional_recursion, dead_code, unused_variables, unused_imports)]

extern crate clap;
extern crate time;

use clap::{App, Arg, SubCommand};
use std::iter;
use std::rc::Rc;
use time::PreciseTime;

use Number::*;

fn main() {
    let matches = App::new("Countdown solver")
        .version("1.0")
        .author("Arjan Boschman")
        .about("Calculates all possible solutions to a countdown numbers round.")
        .arg(
            Arg::with_name("TARGET")
                .help("The target number.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input numbers to use. Enter at least one number.")
                .required(true)
                .multiple(true)
                .index(2),
        )
        .get_matches();

    let target = matches
        .value_of("TARGET")
        .unwrap()
        .parse::<usize>()
        .expect("TARGET must be an integral number.");
    let input_numbers = matches
        .values_of("INPUT")
        .unwrap()
        .map(|string| {
            string
                .parse::<usize>()
                .expect("INPUTS must be integral numbers.")
        })
        .map(|number| Rc::new(Atomic(number)))
        .collect();
    solve(target, input_numbers);
}

enum Number {
    Atomic(usize),
    Sum(Rc<Number>, Rc<Number>),
    Difference(Rc<Number>, Rc<Number>),
    Product(Rc<Number>, Rc<Number>),
    Quotient(Rc<Number>, Rc<Number>),
}

impl Number {
    fn calc(&self) -> usize {
        match *self {
            Atomic(value) => value,
            Sum(ref a, ref b) => a.calc() + b.calc(),
            Difference(ref a, ref b) => a.calc() - b.calc(),
            Product(ref a, ref b) => a.calc() * b.calc(),
            Quotient(ref a, ref b) => a.calc() / b.calc(),
        }
    }

    fn print(&self) -> String {
        match *self {
            Atomic(ref value) => format!("{}", value),
            Sum(ref a, ref b) => format!("[{} + {} = {}]", a.print(), b.print(), self.calc()),
            Difference(ref a, ref b) => {
                format!("[{} - {} = {}]", a.print(), b.print(), self.calc())
            }
            Product(ref a, ref b) => format!("[{} * {} = {}]", a.print(), b.print(), self.calc()),
            Quotient(ref a, ref b) => format!("[{} / {} = {}]", a.print(), b.print(), self.calc()),
        }
    }

    fn calc_if_valid(&self) -> Option<usize> {
        match *self {
            Difference(ref a, ref b) if a.calc() < b.calc() => None,
            Quotient(ref a, ref b) if b.calc() == 0 || a.calc() % b.calc() != 0 => None,
            _ => Some(self.calc()),
        }
    }
}

fn solve(target: usize, numbers: Vec<Rc<Number>>) {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in 0..6 {
                // if solutions.len() > 0 { continue; }//remove to look for more than 1 solution.
                let a = Rc::clone(&numbers[i]);
                let b = Rc::clone(&numbers[j]);
                let combination = match k {
                    0 => Rc::new(Sum(a, b)),
                    1 => Rc::new(Difference(a, b)),
                    2 => Rc::new(Product(a, b)),
                    3 => Rc::new(Quotient(a, b)),
                    4 => Rc::new(Difference(b, a)),
                    5 => Rc::new(Quotient(b, a)),
                    _ => panic!(),
                };
                if let Some(valid_number) = combination.calc_if_valid() {
                    if valid_number == target {
                        println!("{}", combination.print());
                    // solutions.push(combination);
                    } else if numbers.len() > 2 {
                        let remaining_numbers = (0..numbers.len())
                            .filter(|index| index != &i && index != &j)
                            .map(|index| Rc::clone(&numbers[index]))
                            .chain(iter::once(combination))
                            .collect::<Vec<Rc<Number>>>();
                        solve(target, remaining_numbers);
                        // solutions.extend(check(target, remaining_numbers));
                    }
                }
            }
        }
    }
}
