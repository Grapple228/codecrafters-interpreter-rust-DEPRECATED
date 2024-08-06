pub trait StringExtensions {
    fn substring(&self, start: usize, end: usize) -> String;
    fn char_at(&self, index: usize) -> char;
}

impl StringExtensions for String {
    fn substring(&self, start: usize, end: usize) -> String{
        self.chars().skip(start).take(end-start).collect()
    }

    fn char_at(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or_default()
    }
}
