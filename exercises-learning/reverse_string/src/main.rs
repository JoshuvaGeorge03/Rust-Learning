use unicode_segmentation::UnicodeSegmentation;

fn main() {
    assert_eq!("aeiou", reverse("uoiea"));
    println!("reversed {}", reverse("a̐éö̲\r\n"));
    println!("reversed {}", reverse("man\u{0303}ana"));
}

fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
