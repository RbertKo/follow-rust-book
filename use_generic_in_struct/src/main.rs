struct Point<T> {
    x: T,
    y: T,
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T,U> PointV2<T, U> {
    fn mixin<V, W>(self, other: PointV2<V, W>) -> PointV2<T, W> {
        PointV2 {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // let test = Point { x: 5.0, y: 10 }; -> 같은 타입이 아니면 오류 발생

    let union_point = PointV2 { x: 5, y: 1.0 };

    println!("intefer.x = {}", integer.x);

    let integer_v2 = PointV2 { x: 'a', y: 'c'};

    let mixed_point = union_point.mixin(integer_v2);

    println!("mixed_point (x: {}, y: {})", mixed_point.x, mixed_point.y);
}