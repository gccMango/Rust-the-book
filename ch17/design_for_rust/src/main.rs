use design_for_rust::Post;

fn main() {
    // let mut post = Post::new();

    // post.add_text("나는 오늘 도서관에 갔다.");
    // assert_eq!("",post.content());
    // 초고나 리뷰 중인 포스트의 콘텐츠가 
    // 빈 문자열임을 검증 할 수도 없고 그럴 필요도 없다.
    // 그런 포스트의 콘텐츠에 접근하려고 하면 코드가 컴파일 되지 
    // 않기 때문이다. 
    // 수정본
    
    let mut post = Post::new();
    
    post.add_text("나는 오늘 도서관에 갔다.");
    
    let post = post.request_review();
    
    let post = post.approve();
    
    assert_eq!("나는 오늘 도서관에 갔다.",post.content());
}