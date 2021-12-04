mod vars;

// const can define out of function
// let must be defined in function
const MAX_POINTS: u32 = 100_00;

fn main() {
    println!("Hello, world!");
    //vars::sub_a::func_a();
    //vars::sub_b::func_b();

    // mut means this variable is mutable.
    let mut x = 5;
    println!("{}", x);
    // x can rewrite because x is defined as mutable.
    x = 6;
    println!("{}", x);

    // default i32
    // _ means this variable might be unused.
    let _i1 = 3;
    // default f64 (double)
    let _f1 = 0.1;

    // size of integer type in bit
    println!("{}", usize::BITS);
    // {:p} shows an address of a variable (pointer)
    println!("memory address of const is : {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    // variables in stack are stored in stack (diffrerent area from static variable).
    // that's why the address if really different from const variable.
    // and the stack variable address is in a row like below.
    // stack address of i2 is : 0x7ffd73ff5028
    // stack address of i3 is : 0x7ffd73ff5030
    println!("stack address of i2 is : {:p}", &i2);
    println!("stack address of i3 is : {:p}", &i3);

    let y = 5;
    println!("stack address of y is {:p}", &y);
    // first defined y is hided by shadowing feature.
    let y = y + 1;
    println!("stack address of y is {:p}", &y);
    // address is also diffrent from first one.
    let y = y * 2;
    println!("stack address of y is {:p}", &y);
    println!("the value of y is: {}", y);
    {
        let y = 0;
        println!("the value of y in blacket is: {}", y);
    }
    // the y in blacket doesn't see here
    println!("the value of y out of blacket is: {}", y);

    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("the value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    // to receive nested tuple ref keyword requires
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    // define 0 * 10 array
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // string slice
    let s1 = "helloこんにちは挨拶"; // 26bytes. (5bytes + 21bytes).
    let s2 = "hello";
    // the actual string value is stored in static area
    // str slice data of stack has the following data
    // ptr: a pointer of the string value head.
    // len: length of the string value
    println!("stack address of s1 is {:p}", &s1);
    println!("stack address of s2 is {:p}", &s2);
    println!("static memory address of s1: {:?}", s1.as_ptr());
    println!("static memory address of s2: {:?}", s2.as_ptr());
    println!("str length of s1: {:?}", s1.len());
    println!("str length of s2: {:?}", s2.len());

    // String
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    // String's actual string value is stored in heap.
    // String slice data of stack has the folowing data
    // ptr: a pointer of the string value head in heap.
    // len: length of the string value
    // cap: capacity length of the string value. capacity is adjusted dynamically.
    println!("stack address of s1 is {:p}", &s1);
    println!("stack address of s2 is {:p}", &s2);
    println!("heap memory address of s1: {:?}", s1.as_ptr());
    println!("heap memory address of s2: {:?}", s2.as_ptr());
    println!("str length of s1: {:?}", s1.len());
    println!("str length of s2: {:?}", s2.len());
    println!("Capacity of s1: {:?}", s1.capacity());
    println!("Capacity of s2: {:?}", s2.capacity());
    s1.push('_');
    s2.push('_');
    println!("Capacity of s1: {:?}", s1.capacity());
    println!("Capacity of s2: {:?}", s2.capacity());
}
