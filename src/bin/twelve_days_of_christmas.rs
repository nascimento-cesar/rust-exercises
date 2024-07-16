fn main() {
    const LYRICS: [&str; 10] = [
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Eight maids a milkin'",
        "Nine pipers pipin'",
        "Ten ladies dancin'",
        "Eleven lords a leapin'",
        "Twelve drummers drummin'",
    ];

    const CHORUS_FIRST: &str =
        "On the first day of Christmas\nMy true love gave to me\nA partridge in a pear tree\n";
    const CHORUS: &str = "On the first day of Christmas\nMy true love gave to me";

    println!("{CHORUS_FIRST}");

    let mut previous_sentence = String::from("Two turtle doves");

    for sentence in LYRICS.iter() {
        previous_sentence = format!("{}\n{}", sentence, previous_sentence);
        println!("{CHORUS}");
        println!("{previous_sentence}");
        println!("And a partridge in a pear tree\n");
    }
}
