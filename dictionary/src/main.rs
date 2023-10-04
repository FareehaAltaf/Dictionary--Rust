mod dictionary;

fn main() {
    use crate::dictionary::dictionary::Dictionary;
    let mut dict = Dictionary::new();

    dict.add_word_def("apple", "A delicious fruit");
    dict.search_word("apple");
    println!("         ~Dictionary Words~ ");
    dict.list_words();
}
