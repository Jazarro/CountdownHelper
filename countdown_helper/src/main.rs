#![allow(unconditional_recursion, dead_code, unused_variables, unused_imports)]

use std::iter;

fn main() {
    // solve(746, vec!(100, 75, 2, 10, 3, 8));
    solve(808, vec!(75, 100, 4, 1, 3, 3));
    // solve(100, vec!(25, 75, 100, 8, 9, 10));
    // solve(100, vec!(8, 9, 10, 1));
    // solve(100, vec!(1,2,3,4,10,80));
}

fn solve(target: usize, raw_numbers: Vec<usize>) {
    let numbers = raw_numbers.iter()
        .map(|number| Box::new(Atomic{value:*number}) as Box<Number>)
        .collect::<Vec<Box<Number>>>();
    check(target, numbers);
}

fn print(numbers: &Vec<Box<Number>>) {
    for number in numbers {
        println!("{}", number.print());
    }
}

fn check(target: usize, numbers: Vec<Box<Number>>) -> Vec<Box<Number>> {
    let mut solutions: Vec<Box<Number>> = vec!();
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j { continue; }
            for k in 0..4 {
                // if solutions.len() > 0 { continue; }//TODO: remove to look for more than 1 solution.
                let a = numbers.get(i).unwrap().another();
                let b = numbers.get(j).unwrap().another();
                let combination = match k {
                    0 => Box::new(Add{a:a,b:b}) as Box<Number>,
                    1 => Box::new(Subtract{a:a,b:b}) as Box<Number>,
                    2 => Box::new(Multiply{a:a,b:b}) as Box<Number>,
                    3 => Box::new(Divide{a:a,b:b}) as Box<Number>,
                    _ => panic!()
                };
                if let Some(valid_number) = combination.value() {
                    if valid_number == target {
                        println!("{}", combination.print());
                        solutions.push(combination);
                    } else if numbers.len() > 2 {
                        let remaining_numbers = (0..numbers.len())
                            .filter(|index| index != &i && index != &j)
                            .map(|index| numbers.get(index).unwrap().another())
                            .chain(iter::once(combination))
                            .collect::<Vec<Box<Number>>>();
                        solutions.extend(check(target, remaining_numbers));
                    }
                };
            }
        }
    }
    solutions
}

trait Number {
    fn value(&self) -> Option<usize>;

    fn expect(&self) -> usize {
        self.value().unwrap()
    }

    fn another(&self) -> Box<Number>;

    fn print(&self) -> String {"".to_string()}
}

struct Atomic {
    value: usize
}

struct Add {
    a: Box<Number>,
    b: Box<Number>
}

struct Subtract {
    a: Box<Number>,
    b: Box<Number>
}

struct Multiply {
    a: Box<Number>,
    b: Box<Number>
}

struct Divide {
    a: Box<Number>,
    b: Box<Number>
}

impl Number for Atomic {
    fn value(&self) -> Option<usize> {
        Some(self.value)
    }

    fn another(&self) -> Box<Number> {
        Box::new(Atomic { value: self.value })
    }

    fn print(&self) -> String {
        format!("{}", self.value)
    }
}

impl Number for Add {
    fn value(&self) -> Option<usize> {
        Some(self.a.expect() + self.b.expect())
    }

    fn another(&self) -> Box<Number> {
        Box::new(Add { a: self.a.another(), b: self.b.another() })
    }

    fn print(&self) -> String {
        format!("[{} + {} = {}]", self.a.print(), self.b.print(), self.expect())
    }
}

impl Number for Subtract {
    fn value(&self) -> Option<usize> {
        if self.a.expect() >= self.b.expect() {
            Some(self.a.expect() - self.b.expect())
        } else {
            None
        }
    }

    fn another(&self) -> Box<Number> {
        Box::new(Subtract { a: self.a.another(), b: self.b.another() })
    }

    fn print(&self) -> String {
        format!("[{} - {} = {}]", self.a.print(), self.b.print(), self.expect())
    }
}

impl Number for Multiply {
    fn value(&self) -> Option<usize> {
        Some(self.a.expect() * self.b.expect())
    }

    fn another(&self) -> Box<Number> {
        Box::new(Multiply { a: self.a.another(), b: self.b.another() })
    }

    fn print(&self) -> String {
        format!("[{} * {} = {}]", self.a.print(), self.b.print(), self.expect())
    }
}

impl Number for Divide {
    fn value(&self) -> Option<usize> {
        if self.b.expect() != 0 && self.a.expect() % self.b.expect() == 0 {
            Some(self.a.expect() / self.b.expect())
        } else {
            None
        }
    }

    fn another(&self) -> Box<Number> {
        Box::new(Divide { a: self.a.another(), b: self.b.another() })
    }

    fn print(&self) -> String {
        format!("[{} / {} = {}]", self.a.print(), self.b.print(), self.expect())
    }
}
