use blog::Post;

fn main() {
	let mut post = Post::new();
	post.add_text("I ate a salad for lunch today");

	let post = post.request_review();

	// reject: PrendingReview -> Draft
	let mut post = post.reject();
	post.add_text("!");	// again in Draft: can write

	let post = post.request_review();

	// Two approves required
	let post = post.approve();
	let post = post.approve();

	assert_eq!("I ate a salad for lunch today!", post.content());

	// let post = post.reject();
	// let post = post.approve();
	// let mut post = post.reject();
	// post.add_text(" So delicious");
	// let post = post.request_review();
	// let post = post.approve();
	// let post = post.approve();

	// assert_eq!("I ate a salad for lunch today! So delicious", post.content())
}