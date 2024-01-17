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
    // 필드와 동일한 이름의 메서드도 가능
    fn width(&self) -> u32 {
        self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        // 표현식을 dbg!에 넣으면 표현식 자체가 출력된다.
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}