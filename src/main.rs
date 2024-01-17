// 출력을 위한 외부 속성
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 메서드 만들기
impl Rectangle {
    // 모든 메서드는 self를 첫 번째 매개변수로 가져야 한다.
    // 일반 함수처럼 단순히 빌리는 용도, 불변, 가변 모두 가능
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        // 표현식을 dbg!에 넣으면 표현식 자체가 출력된다.
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    ); }
