// cd p_17_opp_blog
// https://rinthel.github.io/rust-lang-book-ko/ch17-03-oo-design-patterns.html

extern crate p_17_opp_blog;
use p_17_opp_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}