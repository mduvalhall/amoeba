use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Route {
    segments: Vec<Segment>,
}

#[derive(Debug, PartialEq, Eq)]
enum Segment {
    Plain(String),
    Named(String),
}

impl Route {
    pub fn new(pattern: &str) -> Self {
        Self {
            segments: split(pattern).map(Segment::parse).collect(),
        }
    }

    pub fn extract(&self, path: &str) -> Option<HashMap<String, String>> {
        let parts: Vec<&str> = split(path).collect();
        if parts.len() != self.segments.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (segment, part) in self.segments.iter().zip(parts) {
            match segment {
                Segment::Plain(plain) => {
                    if plain.as_str() != part {
                        return None;
                    }
                }
                Segment::Named(name) => {
                    params.insert(name.clone(), part.to_owned());
                }
            }
        }
        Some(params)
    }
}

impl Segment {
    fn parse(s: &str) -> Self {
        match s.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            Some(name) if !name.is_empty() => Self::Named(name.to_owned()),
            _ => Self::Plain(s.to_owned()),
        }
    }
}

fn split(path: &str) -> impl Iterator<Item = &str> {
    path.split('/').filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let route = Route::new("/this///is/{woah}//{}/{id}/{age}/a/path");
        assert_eq!(
            route.segments,
            vec![
                Segment::Plain("this".to_owned()),
                Segment::Plain("is".to_owned()),
                Segment::Named("woah".to_owned()),
                Segment::Plain("{}".to_owned()),
                Segment::Named("id".to_owned()),
                Segment::Named("age".to_owned()),
                Segment::Plain("a".to_owned()),
                Segment::Plain("path".to_owned()),
            ]
        );
    }

    #[test]
    fn extract() {
        let route = Route::new("/users/{user_id}/notes/{note_id}");
        let params = route.extract("/users/1234/notes/abcd").unwrap();
        assert_eq!(
            params,
            HashMap::from([
                ("user_id".to_owned(), "1234".to_owned()),
                ("note_id".to_owned(), "abcd".to_owned()),
            ])
        );
        let user_id: usize = params["user_id"].parse().unwrap();
        assert_eq!(user_id, 1234);
    }

    #[test]
    fn no_match() {
        let route = Route::new("/users/{id}");
        assert_eq!(route.extract("/posts/5"), None);
        assert_eq!(route.extract("/users/5/extra"), None);
        assert_eq!(route.extract("/users"), None);
    }
}
