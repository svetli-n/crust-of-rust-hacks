#[derive(Debug)]
struct StrSplit<'a, D> {
    haystack: &'a str,
    delim: D,
}

impl<'a, D: Delimiter> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delim: D) -> Self {
        Self {haystack, delim}
    }
}

impl<'a, D: Delimiter> Iterator for StrSplit<'a, D> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let delimeter_position = self.delim.find_next(self.haystack);
        if let Some((start, end)) = delimeter_position {
            let res = &self.haystack[..start];
            self.haystack = &self.haystack[end..];
            Some(res)
        } else if !self.haystack.is_empty() {
            let res = self.haystack;
            self.haystack = "";
            Some(res)
        }
        else {
            None
        }
    }
}

trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start+self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, _)| (start, start+self.len_utf8()))
    }
}

fn get_first<D: Delimiter>(s: &str, delim: D) -> &str {
    StrSplit::new(s, delim).next().expect("we always get at least 1")
}


#[cfg(test)]
mod test {
    use crate::str_split::{get_first, StrSplit};

    #[test]
    fn it_get_first_works_with_char() {
        let haystack = "a b c";
        let res = get_first(haystack, ' ');
        assert_eq!(res, "a");
    }

    #[test]
    fn it_get_first_works_with_str_ref() {
        let haystack = "a b c";
        let res = get_first(haystack, " ");
        assert_eq!(res, "a");
    }

    #[test]
    fn it_works_with_empty() {
        let haystack = "";
        let str_split = StrSplit::new(haystack, " ");
        let res: Vec<&str> = str_split.into_iter().collect();
        assert!(res.is_empty())
    }

    #[test]
    fn it_works_with_normal_input() {
        let haystack = "a b c ";
        let str_split = StrSplit::new(haystack, " ");
        let res: Vec<&str> = str_split.into_iter().collect();
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn it_works_with_missing_delim() {
        let haystack = "abc";
        let str_split = StrSplit::new(haystack, " ");
        let res: Vec<_> = str_split.into_iter().collect();
        assert_eq!(res, vec!["abc"]);
    }
}
