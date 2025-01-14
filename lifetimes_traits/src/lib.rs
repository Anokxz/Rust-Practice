pub mod aggergator {
    pub trait Summarize {
        fn summarize(&self) -> String;
    }

    pub struct NewsArtical {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarize for NewsArtical {
        fn summarize(&self) -> String {
            format!(
                "Headline: {}, by {} ({})",
                self.headline, self.author, self.content
            )
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarize for Tweet {
        fn summarize(&self) -> String {
            format!("{} tweeted {}", self.username, self.content)
        }
    }
}
