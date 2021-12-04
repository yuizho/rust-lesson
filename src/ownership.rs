pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 cann not access here because the owner ship was moved to s2
    println!("{}", s2);

    // the fllowing variables are copied in stack
    // i32 implements copy trait
    let i1 = 1;
    let i2 = i1;
    println!("{}  {}", i1, i2);
    println!("stack address of i1 is : {:p}", &i1);
    println!("stack address of i2 is : {:p}", &i2);

    // str slice is also copied.
    // because str slice doesn't have owner ship.
    // it just reference the actual value in static area.
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{}  {}", sl1, sl2);
    println!("stack address of sl1 is : {:p}", &sl1);
    println!("stack address of sl2 is : {:p}", &sl2);

    let s3 = String::from("hello");
    // this is not owner ship moving
    // this is deep copy
    let s4 = s3.clone();
    println!("stack address of s3 is : {:p}", &s3);
    println!("stack address of s4 is : {:p}", &s4);
    // ofcourse these address are differenti
    println!("heap memory address of hello: {:?}", s3.as_ptr());
    println!("heap memory address of hello: {:?}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("stack address of s5: {:p}", &s5);
    println!("heap address of hello: {:?}", s5.as_ptr());
    println!("len is: {}", s5.len());
    println!("cap is: {}", s5.capacity());
    take_ownership(s5);

    let s6 = String::from("hello");
    println!("stack address of s6: {:p}", &s6);
    println!("heap address of hello: {:?}", s6.as_ptr());
    println!("len is: {}", s6.len());
    let s7 = take_giveback_ownership(s6);
    println!("stack address of s7: {:p}", &s7);
    println!("heap address of hello: {:?}", s7.as_ptr());
    println!("len is: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("the length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    //let mut s10 = String::from("hello");
    //let r1 = &s10;
    //let r2 = &mut s10; // also bollowed as immutable error

    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    // this is last line of r1, r2 's reference
    println!("{} {}", r1, r2);
    // that's why mutable reference can create here
    let r3 = &mut s12;
    // rewrite hello value of heap
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}

fn take_ownership(s: String) {
    println!("stack address of s: {:p}", &s);
    println!("heap address of hello: {:?}", s.as_ptr());
    println!("len is: {}", s.len());
    println!("cap is: {}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
