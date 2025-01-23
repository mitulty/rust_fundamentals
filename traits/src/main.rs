/// @Author: Mitul Tyagi
/// @Date:   2024-11-28 22:26:10
/// @Description: Traits
/*
- A trait defines the functionality a particular type has and can share with other types. They can be used to define shared behavior in an abstract way. Trait bounds
  can be used to specify that a generic type can be any type that has certain behavior.  Traits are similar to a feature often called interfaces in other languages,
  although with some differences.
- Traits can be used to define functions that accept many different types. Trait bounds can be used in the form of generics traits. More than one trait bound can also
  be implemented. Traits can also be returned but only type can be returned.
- Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait
  bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for
  specifying trait bounds inside a where clause after the function signature.
*/

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//-------------------------------------------------------------------------------------------------
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// }

// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// }

//-------------------------------------------------------------------------------------------------
// pub fn notify(item: &(impl Summary + Display)) {
// }

// pub fn notify<T: Summary + Display>(item: &T) {
// }

//-------------------------------------------------------------------------------------------------
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// }

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

//-------------------------------------------------------------------------------------------------

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let newsarticle = NewsArticle {
        headline: String::from("It's Over!!!!!"),
        location: String::from("Mars"),
        author: String::from("Morack"),
        content: String::from("Sports"),
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("News Article: {}", newsarticle.summarize());

    notify(&newsarticle);
    println!("{}", returns_summarizable().summarize());
}
