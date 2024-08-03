pub trait StringExtensions {
    fn char_at(&self, index: usize) -> char;
    fn substring(&self, start: usize, end: usize) -> String;
}

impl StringExtensions for String{
    fn char_at(&self, index: usize) -> char{
        self.chars().nth(index).unwrap_or_default()
    }

    fn substring(&self, start: usize, end: usize) -> String{
        self.chars().skip(start).take(end-start).collect()
    }
}