pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // takes a mutable reference to self because we're changing the post instance
    // that we are calling add_text on.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text); // text to be saved in content
    }

    pub fn content(&self) -> &str {
        //  if the state is Published, we want to return the value in the post’s content field;
        // otherwise, we want to return an empty string slice
        self.state.as_ref().unwrap().content(self)
        // We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value
        // state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
    }

    pub fn request_review(&mut self) {
        // changes state from draft to pending_review
        // To consume the old state, the request_review method needs to take ownership of the state value.
        // This is where the Option in the state field of Post comes in: 
        // we call the take method to take the Some value out of the state field and leave a None in its place, 
        // because Rust doesn’t let us have unpopulated fields in structs. 
        // This lets us move the state value out of Post rather than borrowing it. 
        // Then we’ll set the post’s state value to the result of this operation.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()) // add request_review method to State trait
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // here self: Box<Self> means the method is only valid when called on a Box holding the type
    // this syntax takes ownership of Box<Self>
    // invalidating the old state so the state value of the Post can transform into a new state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}