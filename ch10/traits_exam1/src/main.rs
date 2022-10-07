use traits_exam1::tweet;
use traits_exam1::tweet::Summary;

fn main() {
    let tweet = tweet::Tweet {
        username: "geochan".to_string(),
        content: "I begin to learn Rust".to_string(),
        reply: false,
        retweet: false,
    };
    let news = tweet::NewsArticle{
        headline: "Breaking news".to_string(),
        author: "Geochan Choi".to_string(),
        location: "Seoul".to_string(),
        content: "Sports".to_string(),
    };
    println!("새 뉴스 1개: {}", news.summarize());
    println!("{}", news.summarize_author());
    println!("새 트윗 1개: {}", tweet.summarize());
    println!("{}", tweet.summarize_author());
    tweet::notify(news);
    tweet::notify(tweet);

    let tweet2 = tweet::returns_summerizable();
    tweet::notify(tweet2);
}