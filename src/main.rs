extern crate kryll;

use std::env;
use kryll::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for arg in args {
        println!("{}", arg);
    }

//    let test_page = Page::new()
//        .content("html/head.html", "head")
//        .content("html/header.html", "header")
//        .content("html/footer.html", "footer")
//        .finalize("html/post.html");
//
//    let test_post = Post::new("README.md", "test/README.html");
//    test_post.format().unwrap();
    
    let config = Config::new();
    println!("{:#?}", config);
}
