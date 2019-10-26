use std::convert::TryInto;

const LYRICS: [&str; 11] = [
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "10 lords a-leaping",
    "11 pipers piping",
    "12 drummers drumming",
];

const DAYS: [&str; 11] = [
   "second",
   "third",
   "fourth",
   "fifth",
   "sixth",
   "seventh",
   "eigth",
   "ninth",
   "10th",
   "11th",
   "12th"
];

fn print_lyrics(mut stop: usize) {
    let mut index: i32 = stop.try_into().unwrap();
    while index >=  0 {
        println!("{}", LYRICS[stop]);
        index -= 1;
        if stop == 0 {
            break;
        } else {
            stop -= 1;
        }
    } 
}

fn print_remaining_verses() {
    for (index, day) in DAYS.iter().enumerate() {
        println!("VERSE {}", index + 1);
        println!("On the {} day of Christmas my true love sent to me", day);
        print_lyrics(index);
        println!("A partridge in a pear tree");
        println!("             ***");
    }
}

fn print_first_verse() {
    println!("VERSE 1");
    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree");
    println!("             ***");
}

fn main() {
    print_first_verse();
    print_remaining_verses();
}
