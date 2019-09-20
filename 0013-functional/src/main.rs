use std::thread;
use std::time::Duration;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    return intensity;
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        let n = expensive_closure.value(intensity);
        println!(
            "Today, do {} push-ups!",
            n
        );
        println!(
            "Next, do {} sit-ups!",
            n
        );
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
        } else {
            let n = expensive_closure.value(intensity);
            println!(
                "Today, run for {} minutes!",
                n
            );
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

fn workout_stuff() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        return Cacher { calculation, value: None };
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe { size: u32, style: String }

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(in_my_size.len(), 2);
}

fn iter_stuff() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v3, vec![2, 3, 4]);
}

struct Counter { count: u32 }

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

fn counter_stuff() {
    let mut c1 = Counter::new();
    assert_eq!(c1.next(), Some(1));
    assert_eq!(c1.next(), Some(2));
    assert_eq!(c1.next(), Some(3));
    assert_eq!(c1.next(), Some(4));
    assert_eq!(c1.next(), Some(5));
    assert_eq!(c1.next(), None);

    using_other_iterator_trait_methods();
}

fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    workout_stuff();
    iter_stuff();
    filters_by_size();
    counter_stuff();
}
