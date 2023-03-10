pub struct Querys {
    params: std::collections::HashMap<String, Vec<String>>,
}

impl Querys {
    pub fn new() -> Self {
        Querys {
            params: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, k: &str) -> Option<&Vec<String>> {
        self.params.get(k)
    }

    pub fn insert(&mut self, k: impl ToString, v: impl ToString) {
        self.params
            .entry(k.to_string())
            .and_modify(|e| e.push(v.to_string()))
            .or_insert(vec![v.to_string()]);
    }

    pub fn remove(&mut self, k: &str) -> Option<Vec<String>> {
        self.params.remove(k)
    }

    pub fn stringify(&self) -> String {
        let mut query = String::new();

        if !self.params.is_empty() {
            let mut count = 0;

            for (k, arr) in &self.params {
                for value in arr {
                    query.push_str(&k);
                    query.push('=');
                    query.push_str(&value);
                    query.push('&');
                }
            }
        }

        query
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn clear(&mut self) {
        self.params.clear()
    }
}

mod tests {
    use super::Querys;

    #[test]
    fn test_querys() {
        let mut query = Querys::new();

        query.insert("k", "v");

        assert_eq!(&query.get("k").unwrap(), &&vec![String::from("v")]);

        assert_eq!(query.stringify(), String::from("k=v&"));
        query.insert("k", "v");
        query.insert("k", "v");

        assert_eq!(query.stringify(), String::from("k=v&k=v&k=v&"));

        assert_eq!(Querys::new().stringify(), "");
    }
}
