fn _array() {
    let mut a = [1, 2, 3, 4];
    a[1] = 17;
    println!("{:?}", a);

    let mut a = [0u32; 16];
    #[allow(clippy::needless_range_loop)]
    for i in 0..a.len() {
        a[i] = i as u32;
    }
    println!("{:?}", a);

    let a: [[u32; 0]; 1 << 60] = [[]; 1 << 60];
    println!("{:?}", a);
}

fn _vec() {
    // size = 8 + 8 + 8 + 16 = 40
    // let mut a: Vec<u32> = vec![1u32, 2, 3, 4];
    // size = 8 + 8 + 8 + 20 = 44
    let mut a = Vec::with_capacity(5);
    for i in 0..4 {
        a.push(i as u32);
    }
    a[1] = 17;
    a.push(5);
    println!("{:?}", a);
    println!("{}", a.pop().unwrap());
}

fn slice() {
    // &mut is a "fat pointer": pointer plus length
    let mut a = vec![1, 2, 3, 4, 5];
    let (a1, a2) = a.split_at_mut(2);
    a2[2] = 17;
    a1[0] = 15;
    println!("{:?} {:?}", a1, a2);
    println!("{:?}", a);
}

fn main() {
    slice();
}
