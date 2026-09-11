// An async function is defined as a non_async function

#![allow(unused)]

fn main() {
	extern crate trpl; // required for mdbook test
	use std::future::Future;
	use trpl::Html;
	
	fn page_title(url: &str) -> impl Future<Output = Option<String>> {
	    async move {
	        let text = trpl::get(url).await.text().await;
	        Html::parse(&text)
	            .select_first("title")
	            .map(|title| title.inner_html())
	    }
	}
}