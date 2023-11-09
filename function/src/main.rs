fn main() {
    println!("Hello, world!");
    
    another_function(5, 6);

    let x = 6; // statements

    let y = { // expression (여기서는 코드 블록으로 구분)
        let x = 3;
        x + 1 // 표현식은 마지막에 세미콜론 포함하지 않음
    };

    println!("y의 값 : {}", y);

    let z = five();

    println!("z의 값 : {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("또 다른 함수 !");
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}

fn five() -> i32 {
    5
}
