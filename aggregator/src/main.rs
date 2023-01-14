use aggregator::{Tweet, Summary, largest};

fn hello<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    "hello"
}

fn my_len(s1: &str) -> usize {
    s1.len()
}

fn main() {
    let tweet = Tweet {
        username: String::from("emusk"),
        content: String::from("something"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let r;

    {
        let x = 5;
        r = &x;

        println!("r = {r}");
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = largest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    println!("hello = {}", hello("blah", "blorg"));

    println!("my len = {}", my_len("hello, world"));
    
}
