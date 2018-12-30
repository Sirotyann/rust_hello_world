/***
* 1) We can’t implement external traits on external types
*       -- Without the rule, two crates could implement the same trait for the same type,
*       -- and Rust wouldn’t know which implementation to use.
*
* 2) Traits as arguments
*
* 3) Multiple traits
***/

pub fn demo() {
    println!("###### <<< Rust trait ######");
    demo_create();
    demo_argument();
    demo_multiple_traits();
    println!("###### Rust trait >>> ######");
    println!("");
}

fn create_art() -> NewsArticle {
    NewsArticle {
        headline: String::from("Balalala"),
        location: String::from("Montreal"),
        author: String::from("Cli"),
        content: String::from("Nothing"),
    }
}

fn create_tw() -> Tweet {
    Tweet {
        username: String::from("Cli"),
        content: String::from("Something"),
    }
}

// -- 1) ----------------------
fn demo_create() {
    let art = create_art();
    let tw = create_tw();

    println!("NewsArticle summary : {}", art.summarize());
    println!("NewsArticle description : {}", art.description());

    println!("Tweet summary : {}", tw.summarize());
    println!("Tweet description : {}", tw.description());
}

trait Summary {
    fn summarize(&self) -> String;

    // demo default behavior
    fn description(&self) -> String {
        format!("It is a text")
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn description(&self) -> String {
        format!("It is a NewsArticle")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// -- 2) ----------------------
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn demo_argument() {
    let art = create_art();
    notify(art);
}
// this is the same as above
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// -- 3) ----------------
trait Location {
    fn show_location(&self) -> String;
}

impl Location for NewsArticle {
    fn show_location(&self) -> String {
        format!("Location : {}", self.location)
    }
}

fn show_summary_and_location(item: impl Summary + Location) {
    println!("{} @ {}", item.summarize(), item.show_location());
}

fn demo_multiple_traits() {
    let art = create_art();
    show_summary_and_location(art);
}
