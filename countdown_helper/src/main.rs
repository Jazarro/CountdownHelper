#![allow(unconditional_recursion, dead_code, unused_variables, unused_imports)]

extern crate time;

use time::PreciseTime;
use std::iter;
use std::rc::Rc;

use Number::*;

fn main() {
    let start = PreciseTime::now();
    // solve(100, vec!(1,2,3,4,20,70));
    solve(808, prep(vec!(75, 100, 4, 1, 3, 3)));
    // solve(527, vec!(25, 75, 8, 2, 1, 4));
    // solve(746, vec!(100, 75, 2, 10, 3, 8));
    // solve(24, prep(vec!(8, 4, 3, 3)));
    // solve(900, prep(vec!(1, 2, 7, 890)));
    // solve(100, vec!(25, 75, 100, 8, 9, 10));
    // solve(100, vec!(8, 9, 10, 1));
    // solve(100, vec!(1,2,3,4,10,80));
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}

fn prep(input: Vec<usize>) -> Vec<Rc<Number>> {
    input.iter()
        .map(|&number| Rc::new(Atomic(number)))
        .collect()
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
        match self {
            &Atomic(value) => value,
            &Sum(ref a, ref b) => a.calc() + b.calc(),
            &Difference(ref a, ref b) => a.calc() - b.calc(),
            &Product(ref a, ref b) => a.calc() * b.calc(),
            &Quotient(ref a, ref b) => a.calc() / b.calc(),
        }
    }

    fn print(&self) -> String {
        match self {
            &Atomic(ref value) => format!("{}", value),
            &Sum(ref a, ref b) => format!("[{} + {} = {}]", a.print(), b.print(), self.calc()),
            &Difference(ref a, ref b) => format!("[{} - {} = {}]", a.print(), b.print(), self.calc()),
            &Product(ref a, ref b) => format!("[{} * {} = {}]", a.print(), b.print(), self.calc()),
            &Quotient(ref a, ref b) => format!("[{} / {} = {}]", a.print(), b.print(), self.calc()),
        }
    }

    fn calc_if_valid(&self) -> Option<usize> {
        match self {
            &Difference(ref a, ref b) if a.calc() < b.calc() => None,
            &Quotient(ref a, ref b) if b.calc() == 0 => None,
            _ => Some(self.calc()),
        }
    }
}

fn solve(target: usize, numbers: Vec<Rc<Number>>) {
    for i in 0..numbers.len() {
        // for j in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            // if i == j { continue; }
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
                    _ => panic!()
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
                        solve(target, remaining_numbers)
                        // solutions.extend(check(target, remaining_numbers));
                    }
                };
            }
        }
    }
}
