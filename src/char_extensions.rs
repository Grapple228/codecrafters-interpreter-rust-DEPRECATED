pub trait CharExtensions {
    fn is_alpha_numeric(&self) -> bool;
    fn is_alpha(&self) -> bool;
}

impl CharExtensions for char {
    fn is_alpha_numeric(&self) -> bool {
        self.is_alpha() || self.is_digit(10)
    }

    fn is_alpha(&self) -> bool {
        self >= &'a' && self <= &'z' ||
        self >= &'A' && self <= &'Z' ||
        self == &'_'
    }
}