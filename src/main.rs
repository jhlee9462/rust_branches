// 출력을 위한 외부 속성
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // 표현식을 dbg!에 넣으면 표현식 자체가 출력된다.
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // 표준 에러 콘솔 스트림에 출력하기, 레퍼런스가 아닌 구조체를 전달하면 소유권을 가져가버린다.
    dbg!(&rect1);

    // 구조체 출력하기
    println!("rect1 is {:#?}", rect1);
}

fn area(demensions: &Rectangle) -> u32 {
    demensions.width * demensions.height
}