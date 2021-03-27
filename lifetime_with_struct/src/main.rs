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
