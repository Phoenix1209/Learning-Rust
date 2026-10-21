use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
	post.add_text("; ignored text");	// Isn't Draft, so get's ignored
    assert_eq!("", post.content());

	// reject: PendingReview -> Draft
	post.reject();
	post.add_text("!");	// again on Draft, get's accepted
    post.request_review();

	// Two approvals required
    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today!", post.content());
}