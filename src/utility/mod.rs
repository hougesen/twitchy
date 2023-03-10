pub(crate) mod query;

pub(crate) fn build_query_params<I, S>(items: I, seperator: &str) -> String
where
    I: IntoIterator<Item = S>,
    S: ToString,
{
    items
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(seperator)
}

#[cfg(test)]
mod test_build_query_params {
    use super::build_query_params;

    #[test]
    fn empty_iter() {
        let v: Vec<String> = Vec::new();

        assert_eq!(build_query_params(v, "&"), String::new())
    }

    #[test]
    fn str_items() {
        let items = ["mads", "matilde", "morten"];

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("mads&and=matilde&and=morten")
        )
    }

    #[test]
    fn string_items() {
        let items = vec![
            "mads".to_string(),
            "matilde".to_string(),
            "morten".to_string(),
        ];

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("mads&and=matilde&and=morten")
        )
    }

    #[test]
    fn u8_items() {
        let items = std::collections::LinkedList::from([1u8, 2u8, 3u8]);

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("1&and=2&and=3")
        )
    }

    #[test]
    fn u16_items() {
        let items = std::collections::VecDeque::from([1u16, 2u16, 3u16]);

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("1&and=2&and=3")
        )
    }

    #[test]
    fn u32_items() {
        let items = [1u32, 2u32, 3u32];

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("1&and=2&and=3")
        )
    }

    #[test]
    fn u64_items() {
        let items = [1u64, 2u64, 3u64];

        assert_eq!(
            build_query_params(items, "&and="),
            String::from("1&and=2&and=3")
        )
    }
}
