pub fn name() -> &'static str {
    return "global"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(name(), "global");
    }
}
