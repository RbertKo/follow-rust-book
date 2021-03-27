struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me RbertKo. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };
}

/////////////////////

//  라이프타임 생략 규칙 (lifetime elision rules)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}