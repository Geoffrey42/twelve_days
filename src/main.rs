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

fn print_lyrics(mut stop: i32) {
    while stop >=  0 {
        println!("{}", LYRICS[stop as usize]);
        stop -= 1;
    } 
}

fn print_remaining_verses() {
    let mut index: i32 = 0;

    for day in DAYS.iter() {
        println!("VERSE {}", index + 1);
        println!("On the {} day of Christmas my true love sent to me", day);
        print_lyrics(index);
        println!("A partridge in a pear tree");
        println!("             ***");
        index += 1;
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
