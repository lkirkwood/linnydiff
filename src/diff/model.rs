#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EditKind {
    Insert,
    Delete,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edit<'l> {
    pub kind: EditKind,
    pub line: &'l str,
    pub pos: usize,
}

impl<'l> Edit<'l> {
    pub fn insert(line: &'l str, pos: usize) -> Edit<'l> {
        Edit {
            kind: EditKind::Insert,
            line,
            pos,
        }
    }

    pub fn delete(line: &'l str, pos: usize) -> Edit<'l> {
        Edit {
            kind: EditKind::Delete,
            line,
            pos,
        }
    }
}

pub type Slice<'l> = &'l [&'l str];

#[derive(Debug, Clone)]
pub struct Snake {
    pub start: (isize, isize),
    pub end: (isize, isize),
}

impl Snake {
    pub fn len(&self) -> isize {
        self.end.0 - self.start.0
    }

    pub fn split_slices<'l>(&self, a: Slice<'l>, b: Slice<'l>) -> SnakeSplit<'l> {
        SnakeSplit {
            a_first: &a[..self.start.0 as usize],
            b_first: &b[..self.start.1 as usize],
            a_second: Some(&a[self.end.0 as usize..]),
            b_second: Some(&b[self.end.1 as usize..]),
            snake_len: self.len() as usize,
        }
    }
}

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
