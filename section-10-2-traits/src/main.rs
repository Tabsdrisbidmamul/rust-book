use section_10_2_traits::{notify, NewsArticle, Pair, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("author: {}", tweet.summarize_author());

    println!("--------------------------------");

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

    println!("--------------------------------");

    notify(&article);

    println!("--------------------------------");

    let pair = Pair::new(5, 3);

    pair.cmp_display();
}
