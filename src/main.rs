extern crate clap;

use clap::{App, Arg};
use std::iter;
use std::rc::Rc;

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
    Solver::solve(target, input_numbers);
}

enum Number {
    Atomic(usize),
    Sum(usize, Rc<Number>, Rc<Number>),
    Difference(usize, Rc<Number>, Rc<Number>),
    Product(usize, Rc<Number>, Rc<Number>),
    Quotient(usize, Rc<Number>, Rc<Number>),
}

impl Number {
    fn value(&self) -> usize {
        match *self {
            Atomic(value)
            | Sum(value, _, _)
            | Difference(value, _, _)
            | Product(value, _, _)
            | Quotient(value, _, _) => value,
        }
    }

    fn print(&self) -> String {
        match *self {
            Atomic(ref value) => format!("{}", value),
            Sum(ref value, ref a, ref b) => format!("[{} + {} = {}]", a.print(), b.print(), value),
            Difference(ref value, ref a, ref b) => {
                format!("[{} - {} = {}]", a.print(), b.print(), value)
            }
            Product(ref value, ref a, ref b) => {
                format!("[{} * {} = {}]", a.print(), b.print(), value)
            }
            Quotient(ref value, ref a, ref b) => {
                format!("[{} / {} = {}]", a.print(), b.print(), value)
            }
        }
    }

    fn is_valid_difference(a: &Number, b: &Number) -> bool {
        a.value() >= b.value()
    }

    fn is_valid_quotient(a: &Number, b: &Number) -> bool {
        b.value() != 0 && a.value() % b.value() == 0
    }

    fn is_sensible(value: usize, a: &Number, b: &Number) -> bool {
        !a.num_in_history(value) && !b.num_in_history(value)
    }

    fn num_in_history(&self, num: usize) -> bool {
        match *self {
            Atomic(value) => value == num,
            Sum(value, ref a, ref b)
            | Difference(value, ref a, ref b)
            | Product(value, ref a, ref b)
            | Quotient(value, ref a, ref b) => {
                value == num || a.num_in_history(num) || b.num_in_history(num)
            }
        }
    }

    fn new_sum(a: Rc<Number>, b: Rc<Number>) -> Option<Rc<Number>> {
        let sum_value = a.value() + b.value();
        if Number::is_sensible(sum_value, &a, &b) {
            Some(Rc::new(Sum(sum_value, a, b)))
        } else {
            None
        }
    }

    fn new_difference(a: Rc<Number>, b: Rc<Number>) -> Option<Rc<Number>> {
        if Number::is_valid_difference(&a, &b) && Number::is_sensible(a.value() - b.value(), &a, &b)
        {
            Some(Rc::new(Difference(a.value() - b.value(), a, b)))
        } else {
            None
        }
    }

    fn new_product(a: Rc<Number>, b: Rc<Number>) -> Option<Rc<Number>> {
        let product_value = a.value() * b.value();
        if Number::is_sensible(product_value, &a, &b) {
            Some(Rc::new(Product(product_value, a, b)))
        } else {
            None
        }
    }

    fn new_quotient(a: Rc<Number>, b: Rc<Number>) -> Option<Rc<Number>> {
        if Number::is_valid_quotient(&a, &b) && Number::is_sensible(a.value() / b.value(), &a, &b) {
            Some(Rc::new(Quotient(a.value() / b.value(), a, b)))
        } else {
            None
        }
    }
}

impl Eq for Number {}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match (self, other) {
            (&Atomic(ref a), &Atomic(ref b)) => a == b,
            (&Sum(_, ref a1, ref a2), &Sum(_, ref b1, ref b2)) => commutative(a1, a2, b1, b2),
            (&Difference(_, ref a1, ref a2), &Difference(_, ref b1, ref b2)) => {
                non_commutative(a1, a2, b1, b2)
            }
            (&Product(_, ref a1, ref a2), &Product(_, ref b1, ref b2)) => {
                commutative(a1, a2, b1, b2)
            }
            (&Quotient(_, ref a1, ref a2), &Quotient(_, ref b1, ref b2)) => {
                non_commutative(a1, a2, b1, b2)
            }
            _ => false,
        }
    }
}

fn commutative(a1: &Number, a2: &Number, b1: &Number, b2: &Number) -> bool {
    (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1)
}

fn non_commutative(a1: &Number, a2: &Number, b1: &Number, b2: &Number) -> bool {
    a1 == b1 && a2 == b2
}

struct Solver {
    solutions: Vec<Rc<Number>>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            solutions: Vec::new(),
        }
    }

    fn solve(target: usize, numbers: Vec<Rc<Number>>) {
        Solver::new().compute(target, numbers);
    }

    fn compute(&mut self, target: usize, numbers: Vec<Rc<Number>>) {
        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                for k in 0..6 {
                    // if solutions.len() > 0 { continue; }//remove to look for more than 1 solution.
                    let a = Rc::clone(&numbers[i]);
                    let b = Rc::clone(&numbers[j]);
                    let result_candidate = match k {
                        0 => Number::new_sum(a, b),
                        1 => Number::new_difference(a, b),
                        2 => Number::new_product(a, b),
                        3 => Number::new_quotient(a, b),
                        4 => Number::new_difference(b, a),
                        5 => Number::new_quotient(b, a),
                        _ => panic!(),
                    };
                    if let Some(result) = result_candidate {
                        if result.value() == target {
                            if self.solutions.iter().any(|value| value == &result) {
                                // println!("Duplicate found!");
                            } else {
                                println!("{}", result.print());
                                self.solutions.push(result);
                            }
                        } else if numbers.len() > 2 {
                            let remaining_numbers = (0..numbers.len())
                                .filter(|index| index != &i && index != &j)
                                .map(|index| Rc::clone(&numbers[index]))
                                .chain(iter::once(result))
                                .collect::<Vec<Rc<Number>>>();
                            self.compute(target, remaining_numbers);
                        }
                    }
                }
            }
        }
    }
}
