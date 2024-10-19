#[derive(Debug)]
pub enum EditKind {
    INSERT,
    DELETE,
}

#[derive(Debug)]
pub struct Edit<'a> {
    kind: EditKind,
    line: &'a str,
    pos: usize,
}

pub type Slice<'l> = &'l [&'l str];

#[derive(Debug)]
pub struct SnakeSplit<'l> {
    a_first: Slice<'l>,
    b_first: Slice<'l>,
    a_second: Option<Slice<'l>>,
    b_second: Option<Slice<'l>>,
    snake_len: usize,
}
