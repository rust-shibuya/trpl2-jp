use std::string::String;

fn main() {
    let mut gift = "".to_string();
    gift = song("first", "A partridge in a pear tree.".to_string() + &gift);
    gift = song("second", "Two turtle doves, And ".to_string() + &gift);
    gift = song("third", "Three French hens, ".to_string() + &gift);
    gift = song("fourth", "Four calling birds, ".to_string() + &gift);
    gift = song("fifth", "Five golden rings! ".to_string() + &gift);
    gift = song("sixth", "Six geese a-laying, ".to_string() + &gift);
    gift = song("seventh", "Seven swans a-swimming, ".to_string() + &gift);
    gift = song("eighth", "Eight maids a-milking, ".to_string() + &gift);
    gift = song("ninth", "Nine ladies dancing, ".to_string() + &gift);
    gift = song("tenth", "Ten lords a-leaping, ".to_string() + &gift);
    gift = song("eleventh", "Eleven pipers piping, ".to_string() + &gift);
    song("twelfth", "Twelve drummers drumming, ".to_string() + &gift);
}

fn song(days: &str, gift: String) -> String {
    println!(
        "On the {} day of Christmas, My true love sent to me {}",
        days, gift
    );
    gift
}
