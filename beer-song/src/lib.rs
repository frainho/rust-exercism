const NO_BOTTLES: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const ONE_BOTTLE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const TWO_BOTTLES: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    match n {
        0 => NO_BOTTLES.to_string(),
        1 => ONE_BOTTLE.to_string(),
        2 => TWO_BOTTLES.to_string(),
        3..=99 => {
            let next_number_of_beers = n - 1;
            format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, next_number_of_beers)
        },
        _ => panic!("It's the 99 bottles of beer song")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = Vec::new();
    for number in (end..=start).rev() {
        song.push(verse(number))
    }
    song.join("\n")
}
