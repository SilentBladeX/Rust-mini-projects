fn main() {
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    let suffixe = ["st", "nd", "rd"];

    for outer in 1..=12 {
        let suffix = if outer == 1 {
            suffixe[0]
        } else if outer == 2 {
            suffixe[1]
        } else if outer == 3 {
            suffixe[2]
        } else {
            "th"
        };

        println!("\nOn the {}{} day of Christmas, my true love sent to me", outer,suffix);
        for inner in (0..outer).rev(){
            if outer>1 && inner==0{
             println!("AND, {:?}",gifts[inner]);   
            } 
           else
           { println!("{:?}",gifts[inner]);
        }
        }
    }
}
