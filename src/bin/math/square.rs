#[derive(Debug)]
pub(crate) struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {width, height}
    }
    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::square;

    #[test]
    fn test_get_area() {
        assert_eq!(20, square::Rectangle::new(5, 4).get_area());
    }
}
