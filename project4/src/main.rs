fn f1(mut n: u32) {
    n = 1;
    println!("f1: n = {}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2: n_ptr = {:p}", n_ptr);
    *n_ptr = 2;
    println!("f2: *n_ptr = {}", *n_ptr);
}

fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}

fn main() {
    let mut n = 0;
    println!("main: n = {}", n);

    f1(n);
    println!("main: n = {}", n);

    f2(&mut n);
    println!("main: n = {}", n);

    // pointer primitive type
    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 1_000;
        assert_eq!(*n1_ptr, 1_000);
    }

    // fn primitive type
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    // tuple
    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
    *x1_ptr += 3;
    *y1_ptr *= -1;
    assert_eq!(t1, ((3, -5), (10, -1)));

    // array
    let array = [0, 1];
    assert_eq!(array.get(1), Some(&1));
    assert_eq!(array.get(2), None);

    // slice
    let mut a4 = [6, 4, 2, 8, 0, 9];
    &mut a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    // &'static str type
    let apples = "Red Apple, Green Apple";
    let mut apple_iter = apples.split(",");
    assert_eq!(apple_iter.next(), Some("Red Apple"));
    
    let green = apple_iter.next();
    assert_eq!(green, Some(" Green Apple"));
    assert_eq!(green.map(str::trim), Some("Green Apple"));

    assert_eq!(apple_iter.next(), None);

    // String
    // it is impossible to get `&muy str` from `&'static str`
    let mut s1 = "abc".to_string(); // String type
    let s2 = s1.as_mut_str(); // &mut str type
    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABC");
}
