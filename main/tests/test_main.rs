use add::add;
mod tests {
    use super::*;
    fn test1() {
        assert_eq!(add(&1, &2), 3);
    }
    fn test2() {
        assert_eq!(add(&0, &0), 0);
    }
    fn test3() {
        assert_eq!(add(&-1, &-1), -2);
    }
}