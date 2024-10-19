#[derive(Debug)]
pub enum EditKind {
    INSERT,
    DELETE,
}

#[derive(Debug)]
pub struct Edit<'a> {
    pub kind: EditKind,
    pub line: &'a str,
    pub pos: usize,
}

pub type Slice<'l> = &'l [&'l str];

#[derive(Debug)]
pub struct SnakeSplit<'l> {
    pub a_first: Slice<'l>,
    pub b_first: Slice<'l>,
    pub a_second: Option<Slice<'l>>,
    pub b_second: Option<Slice<'l>>,
    pub snake_len: usize,
}

fn slice_eq<'l>(a: Slice<'l>, b: Slice<'l>) -> bool {
    for (line_a, line_b) in a.iter().zip(b) {
        if line_a != line_b {
            return false;
        }
    }
    true
}

impl<'l> PartialEq for SnakeSplit<'l> {
    fn eq(&self, other: &Self) -> bool {
        slice_eq(self.a_first, other.a_first)
            && slice_eq(self.b_first, other.b_first)
            && ((self.a_second.is_none() && other.a_second.is_none())
                || (self.a_second.is_some()
                    && other.a_second.is_some()
                    && self.a_second.unwrap() == other.a_second.unwrap()))
            && ((self.b_second.is_none() && other.b_second.is_none())
                || (self.b_second.is_some()
                    && other.b_second.is_some()
                    && self.b_second.unwrap() == other.b_second.unwrap()))
            && self.snake_len == other.snake_len
    }
}
