fn main() {
    let s1 = "first";
    let s2 = "second";
    let longest: &str = longest(s1, s2);
    println!("{longest}");

    let novel = String::from("Call me Daddy. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let important_excerpt = ImportantExcerpt {
        id: String::from("38cc89b2-143b-409e-8efc-bcc9c9e008ec"),
        part: first_sentence,
    };
    println!("{important_excerpt:#?}");
}

// Lifetime informs that returned variable must live as long as the shortest lived variable from
// that lifetime. For example, if `x` will live shorter than `y` (i.e. will be dropped first), then
// the returned value will be dropped along `x`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
    id: String,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
