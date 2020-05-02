pub fn create_verse(from: i32) -> String {
  let lyric = match from {
    0 => String::from(
      "No more bottles of milk on the wall, no more bottles of milk.
Go to the store and buy some more, 99 bottles of milk on the wall.\n",
    ),
    1 => String::from(
      "1 bottle of milk on the wall, 1 bottle of milk.
Take it down and pass it around, no more bottles of milk on the wall.\n",
    ),
    2 => String::from(
      "2 bottles of milk on the wall, 2 bottles of milk.
Take one down and pass it around, 1 bottle of milk on the wall.\n",
    ),
    _ if from % 6 == 0 => format!(
      "{} bottles of milk on the wall, {} bottles of milk.
Take one down and pass it around, {} bottles of milk on the wall.\n",
      from,
      from,
      from - 1
    ),
    _ => format!(
      "{} bottles of milk on the wall, {} bottles of milk.
Take one down and pass it around, {} bottles of milk on the wall.\n",
      from,
      from,
      from - 1
    ),
  };

  return lyric;
}

pub fn create_verses(from: i32, to: i32) -> String {
  let verses: Vec<String> = (to..from + 1).map(create_verse).rev().collect();

  verses.join("")
}
