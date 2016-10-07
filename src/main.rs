extern crate kryll;

use std::env;
use kryll::post::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for arg in args {
        println!("{}", arg);
    }

    let test_post_please_ignore = Post::new()
        .content("html/head.html", "head")
        .content("html/header.html", "header")
        .content("html/footer.html", "footer")
        .template("html/post.html")
        .build();

    println!("{:?}", test_post_please_ignore);
}
