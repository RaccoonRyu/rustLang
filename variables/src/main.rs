fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    // scalar
    const MAX_POINTS: u32 = 100_000;
    println!("maxPoints의 값: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y + 2;
    println!("y의 값: {}", y);

    //let mut spaces = "      ";
    //spaces = spaces.len();

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let rema = 43 % 5;

    println!("덧셈, 뺄셈, 곱셈, 나눗셈, 나머지 : {} {} {} {} {}", sum, diff, prod, quot, rema);

    let t = true;

    let f: bool = false;

    // char
    let c = 'z';
    let z = 'z';

    // compound
    // tuple ()
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (600, 7.4, 2);
    let (x, y, z) = tup2;
    println!("y의 값 : {}", y);

    let tup3: (i32, f64, u8) = (800, 8.4, 3);
    let eight_hundred = tup3.0;
    let eight_point_four = tup3.1;
    let three = tup3.2;

    // array []
    let array1 = [1, 2, 3, 4, 5];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];
    let array3 = [3; 5];

    let first = array1[0];
    let second = array1[1];

    let ten = array1[10];
    println!("요소의 값 : {}", ten); // err
}
