// dictionary.rs is module here
//usage of clone methods to avoid ownership and borrowing issues.
//When you return a reference to a string that is stored within the Word struct,
//you would need to ensure that the lifetime of that reference is valid for the entire duration of its usage.
pub mod dictionary{
    #[derive()]
    pub struct Word {
        pub word: String,
        pub definition: String,
    }

    #[derive()]
    pub struct Dictionary {
       pub words: Vec<Word>, // store a collection of words in a vector ; Vec is a growable array;
    }

    impl Dictionary{
        pub fn new() -> Dictionary { //empty parameters to create a new instance of the Dictionary struct, allows def intialization
            Dictionary { words: Vec::new() }
        }

        pub fn add_word_def(&mut self, word:&str, definition:&str) { //Add a word and its definition to the dictionary
        // &str is a string slice, a reference to a string
        // since `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable hence mut even though we are not changing the value of self
            let new_word = Word {
                word: word.to_string(),
                definition: definition.to_string(),
            };
            self.words.push(new_word);
        }

        pub fn search_word(&mut self, the_word:&str) ->  Option<String>{
        // Search for a word and return its definition 
        // Option is an enum that can either be Some or None,why option? cuz we may not find the word,
        // why not just use if else and user return keyword? we want to return a reference to the definition of the word, not the definition itself
            for w in &self.words{
                if w.word == the_word{
                    println!("Found your word, here's your defintion: {} ", w.definition);
                    return Some(w.definition.clone()); //some = exists, clone = copy the value of the definition
                }
            }
            return None; // in case nothing to iterate over AND if no match found
        }

        pub fn list_words(&self){ // List all words and their definitions
            for w in &self.words{
                println!("{}: {}", w.word , w.definition);
            }
        }
    }
}

/*  if u wnated to use hashmaps here example: but try not making it hard coded.
 use std::collections::HashMap
 fn main() {
     // Create an empty dictionary (hash map)
     let mut dictionary = HashMap::new()
     // Add words and their definitions to the dictionary
     dictionary.insert("apple", "a fruit");
     dictionary.insert("car", "a vehicle");
     dictionary.insert("book", "a written or printed work")
     // Retrieve and print definitions
     if let Some(definition) = dictionary.get("apple") {
         println!("Definition of 'apple': {}", definition);
     } else {
         println!("'apple' not found in the dictionary.");
     
     if let Some(definition) = dictionary.get("banana") {
         println!("Definition of 'banana': {}", definition);
     } else {
         println!("'banana' not found in the dictionary.");
     }
 }
*/