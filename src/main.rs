mod song;

fn main() {
    println!("Hello world!");
}

fn createVerse(from: i32) -> String {
    String::from("some verse")
}

fn createVerses(from: i32, to: i32) -> String {
    String::from("some verse")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_the_first_verse() {
        let mut verse = String::from("99 bottles of milk on the wall, ");

        verse.push_str("99 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("98 bottles of milk on the wall.\n");

        assert_eq!(verse, createVerse(99));
    }

    #[test]
    fn test_the_another_verse() {
        let mut verse = String::from("3 bottles of milk on the wall, ");

        verse.push_str("3 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("2 bottles of milk on the wall.\n");

        assert_eq!(verse, createVerse(3));
    }

    #[test]
    fn test_the_verse_2() {
        let mut verse = String::from("2 bottles of milk on the wall, ");

        verse.push_str("2 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("1 bottle of milk on the wall.\n");

        assert_eq!(verse, createVerse(3));
    }

    #[test]
    fn test_the_verse_1() {
        let mut verse = String::from("1 bottle of milk on the wall, ");

        verse.push_str("1 bottle of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("no more bottles of milk on the wall.\n");

        assert_eq!(verse, createVerse(3));
    }

    #[test]
    fn test_the_verse_0() {
        let mut verse = String::from("No more bottles of milk on the wall, ");

        verse.push_str("no more bottles of milk.\n");
        verse.push_str("Go to the store and buy some more, ");
        verse.push_str("99 bottles of milk on the wall.\n");

        assert_eq!(verse, createVerse(3));
    }

    #[test]
    fn test_a_couple_of_verses() {
        let mut verse = String::from("99 bottles of milk on the wall, ");

        verse.push_str("99 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("98 bottles of milk on the wall.\n");

        verse.push_str("\n");

        verse.push_str("98 bottles of milk on the wall, ");
        verse.push_str("98 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("97 bottles of milk on the wall.\n");

        assert_eq!(verse, createVerses(99, 98));
    }

    #[test]
    fn test_a_few_verses() {
        let mut verse = String::from("2 bottles of milk on the wall, ");

        verse.push_str("2 bottles of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("1 bottle of milk on the wall.\n");

        verse.push_str("\n");

        verse.push_str("1 bottle of milk on the wall, ");
        verse.push_str("1 bottle of milk.\n");
        verse.push_str("Take one down and pass it around, ");
        verse.push_str("no more bottles of milk on the wall.\n");

        verse.push_str("\n");

        verse.push_str("No more bottles of milk on the wall, ");

        verse.push_str("no more bottles of milk.\n");
        verse.push_str("Go to the store and buy some more, ");
        verse.push_str("99 bottles of milk on the wall.\n");

        assert_eq!(verse, createVerses(2, 0));
    }

    #[test]
    fn test_the_whole_song() {
        assert_eq!(song::getLyrics(), createVerses(99, 0));
    }
}
