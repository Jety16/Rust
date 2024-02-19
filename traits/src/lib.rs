 // Trait definitions are a way to group method signatures together to define a set of behaviors         necessary to accomplish some purpose
 
 pub trait Summary {
     fn summarize(&self) -> String;
 }
 
 // Implementing a trait on a type is similar to implementing regular methods. 
 // The difference is that after impl, we put the trait name we want to implement, 
 // then use the for keyword, and then specify the name of the type we want to implement the trait for
 
 pub struct NewsArticle {
     pub headline: String,
     pub location: String,
     pub author: String,
     pub content: String,
 }
 
 impl Summary for NewsArticle {
     fn summarize(&self) -> String {
         format!("{}, by {} ({})", self.headline, self.author, self.location)
     }
 }
 
 pub struct Tweet {
     pub username: String,
     pub content: String,
     pub reply: bool,
     pub retweet: bool,
 }
 
 impl Summary for Tweet {
     fn summarize(&self) -> String {
         format!("{}: {}", self.username, self.content)
     }
