enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // rust's max stack size is 8MB
    // 7MB array (u8 -> 1bytes.  1 * 7000000)
    let a1: [u8; 7000000] = [1; 7000000];
    // when the following code is executed, stackoverflow error will be occured.
    //let a1: [u8; 8000001] = [1; 8000001];

    // vector's memory construction is almost same as String type
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("stack address of v1 is: {:p}", &v1);
    println!("stack address of v2 is: {:p}", &v2);
    println!("heap memory address of v1: {:?}", v1.as_ptr());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
}
