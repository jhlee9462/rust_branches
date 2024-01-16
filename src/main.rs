fn main() {

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(demensions: (u32, u32)) -> u32 {
    // 인덱스로 접근해야 하기 때문에 명시적이지 않다.
    demensions.0 * demensions.1
}