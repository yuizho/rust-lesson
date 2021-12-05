trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    fn summarize(&self) -> String {
        // this is default implementation
        String::from("(Read more...)")
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewArticle {
    //fn summarize(&self) -> std::string::String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}
}
impl Message for NewArticle {
    fn message(&self) -> String {
        String::from("this is message from article!!!")
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> std::string::String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("yuizho"),
        content: String::from("aaaa"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    let article = NewArticle {
        headline: String::from("head line !!"),
        location: String::from("japan"),
        author: String::from("yuizho"),
        content: String::from(
            "this is test \
            hi!!!",
        ),
    };
    println!("{}", article.summarize());
    notify(&article);
    // tweet can not pass to notify. because it doesn't impl Message trait.
    //notify(&tweet);
}

fn get_price<T: Fruits>(fruits: T) {
    println!("price is : {}", fruits.price());
}

// impl key word can specify multiple trait
// this aruguments is guaranteed to implement A and B type.
fn notify(item: &(impl Summary + Message)) {
    println!("Breaking news!!: {}", item.summarize());
    println!("Message! {}", item.message())
}
