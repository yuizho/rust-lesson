struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    //println!("the largetst is {}", largest_i32(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("the largetst is {}", largest(number_list));
    println!("the largetst is {}", largest(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 1.0, y: 2 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
