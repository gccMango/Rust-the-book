use design_ex::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("나는 오늘 도서관에 갔다.");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());
    
    post.approve();
    assert_eq!("나는 오늘 도서관에 갔다.",post.content());
}