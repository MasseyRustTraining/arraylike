fn no_return() -> ! {
    loop {}
}

fn proc() {
    println!("hi");
}

fn main() -> ! {
    let mut t = ('c', "hello", 17);
    t.1 = "world";
    let (c, s, _) = t;
    println!("{} {}", c, s,);

    let t: (i32,) = (1i32,);
    println!("{:?}", t);

    // "unit" type
    let t: () = ();
    println!("{:?}", t);

    let t = proc();
    println!("{:?}", t);

    let mut t = (0,);
    for i in 0..100 {
        t = (i,);
        println!("{:?}", t);
    }
    println!("{:?}", t);

    proc();

    no_return();
}
