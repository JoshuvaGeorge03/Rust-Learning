fn main() {

    const DAYS: [&str; 12] = ["first", "second", "third", "foruth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"];

    const INITIAL_STANCE_OF_SONG: [&str; 2] =
        ["On the {} day of Christmas", "my true love gave to me"];

    // let one = INITIAL_STANCE_OF_SONG[0];

    // let one = one.replace("{}", "first");

    // let one = one.as_str();

    // println!("{}", one);    



    let mut twelve_days_of_gifts: [&str; 12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree!",
    ];

    twelve_days_of_gifts.reverse();

    let mut index = 0;

    while index < DAYS.len() {

        let day_replaced_line = INITIAL_STANCE_OF_SONG[0].replace("{}", DAYS[index]);

             println!("{day_replaced_line}");
        println!("{}", INITIAL_STANCE_OF_SONG[1]);

        // let (mut first_part, second_part) = twelve_days_of_gifts.split_at(index + 1);

        // first_part.reverse();

        // for line in first_part {
        //     println!("{}", line);
        //     println!("");
        // }

        let mut inner_index: usize = index;

        // while inner_index >= 0 {

        //     let new_index = inner_index;

        //     println!("{}", twelve_days_of_gifts[new_index]);

        //     inner_index -= 1;
        // }

        loop {
            if inner_index == 0 {
                println!("{}", twelve_days_of_gifts[inner_index]);
                break;
            }
            println!("{}", twelve_days_of_gifts[inner_index]);
            inner_index -= 1;
        }
        println!("");
 
        index +=1;
    }

    // for day in DAYS {
    //     let day_replaced_line = INITIAL_STANCE_OF_SONG[0].replace("{}", day);
    //     println!("{day_replaced_line}");
    //     println!("{}", INITIAL_STANCE_OF_SONG[1]);

    // }

    // let mut index = 0;

    // while index < twelve_days_of_gifts.len() {
        
    // }
}
