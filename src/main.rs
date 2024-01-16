// 출력을 위한 외부 속성
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // 구조체 출력하기
    println!("rect1 is {:#?}", rect1);
}

fn area(demensions: &Rectangle) -> u32 {
    demensions.width * demensions.height
}