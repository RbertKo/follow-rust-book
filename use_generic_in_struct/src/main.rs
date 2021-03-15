struct Point<T> {
    x: T,
    y: T,
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // let test = Point { x: 5.0, y: 10 }; -> 같은 타입이 아니면 오류 발생

    let union_point = PointV2 { x: 5, y: 1.0 };
}