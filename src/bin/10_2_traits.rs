use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let vector = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    println!("summarized vector: {}", vector.summarize());
}

trait Summary {
    fn summarize_author(&self) -> String;

    // Default method implementation.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Missing trait implementation will result in the use of the default implementation, if default
// implementation exists.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("[{}]", self.author)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// I can implement custom trait for struct from standard library or another crate.
impl Summary for Vec<String> {
    fn summarize_author(&self) -> String {
        format!("@nobody")
    }

    fn summarize(&self) -> String {
        return self.join(", ");
    }
}

// Traits can be used as function parameters with generics.
#[allow(dead_code)]
fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Alternative syntax of using traits as parameters which is "clearer" according to documentation.
#[allow(dead_code)]
fn notify_where<T>(item: &T) -> impl Clone
where
    T: Summary + Display,
{
    let summary = item.summarize();

    println!("Breaking news! {}", summary);

    return summary;
}
