use super::*;
use paste::paste;
use std::fs;

macro_rules! slice_from_str {
    ($name:ident, $string:expr) => {
        paste! {
            let [<$name _vec>] = $string
                .chars()
                .into_iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<_>>();

            let $name = [<$name _vec>].iter().map(|string| string.as_ref()).collect::<Vec<&str>>();
        }
    };
}

macro_rules! split_from_strs {
    ($name:ident, $a1:expr, $b1:expr, $a2:expr, $b2:expr, $len:expr) => {
        paste! {
            let [<$name _a1>] = $a1.chars().map(|ch| ch.to_string()).collect::<Vec<_>>();
            let [<$name _b1>] = $b1.chars().map(|ch| ch.to_string()).collect::<Vec<_>>();
            let [<$name _a1_>] = [<$name _a1>].iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let [<$name _b1_>] = [<$name _b1>].iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let [<$name _a2>] = $a2.chars().map(|ch| ch.to_string()).collect::<Vec<_>>();
            let [<$name _b2>] = $b2.chars().map(|ch| ch.to_string()).collect::<Vec<_>>();
            let [<$name _a2_>] = [<$name _a2>].iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let [<$name _b2_>] = [<$name _b2>].iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let $name = SnakeSplit {
                a_first: &[<$name _a1_>],
                b_first: &[<$name _b1_>],
                a_second: Some(&[<$name _a2_>]),
                b_second: Some(&[<$name _b2_>]),
                snake_len: $len
            };
        }
    };
}

#[test]
fn test_midsnake_0() {
    slice_from_str!(source, "aaxxaa");
    slice_from_str!(target, "bxxbb");
    let actual = midsnake(&source, &target);
    split_from_strs!(desired, "aa", "b", "aa", "bb", 2);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_1() {
    slice_from_str!(source, "axxaa");
    slice_from_str!(target, "bbxxbb");
    let actual = midsnake(&source, &target);
    split_from_strs!(desired, "a", "bb", "aa", "bb", 2);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_2() {
    slice_from_str!(source, "axa");
    slice_from_str!(target, "bbxbb");
    let actual = midsnake(&source, &target);
    split_from_strs!(desired, "a", "bb", "a", "bb", 1);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_3() {
    slice_from_str!(source, "axa");
    slice_from_str!(target, "bxbbb");
    let actual = midsnake(&source, &target);
    split_from_strs!(desired, "a", "b", "a", "bbb", 1);

    assert_eq!(desired, actual);
}

#[test]
fn test_diff_0() {
    slice_from_str!(source, "aaaaa");
    slice_from_str!(target, "aabbaa");
    let edits = diff(&source, &target);
    let desired = vec![
        Edit::delete("a", 2),
        Edit::insert("b", 2),
        Edit::insert("b", 3),
    ];

    assert_eq!(desired, edits);
}

#[test]
fn test_diff_1() {
    slice_from_str!(source, "foobarbazbar");
    slice_from_str!(target, "foobarbar");
    let edits = diff(&source, &target);
    let desired = vec![
        Edit::delete("z", 8),
        Edit::delete("b", 9),
        Edit::delete("a", 10),
    ];

    assert_eq!(desired, edits);
}

#[test]
fn test_diff_2() {
    slice_from_str!(source, "spaceheater");
    slice_from_str!(target, "sheeptractor");
    let edits = diff(&source, &target);
    let desired = vec![
        Edit::delete("p", 1),
        Edit::delete("a", 2),
        Edit::delete("c", 3),
        Edit::delete("e", 4),
        Edit::insert("e", 3),
        Edit::insert("p", 4),
        Edit::insert("t", 5),
        Edit::insert("r", 6),
        Edit::insert("c", 8),
        Edit::delete("e", 9),
        Edit::insert("o", 10),
    ];

    assert_eq!(desired, edits);
}

#[test]
fn test_diff_3() {
    slice_from_str!(source, "sheeptractor");
    slice_from_str!(target, "spaceheater");
    let edits = diff(&source, &target);
    let desired = vec![
        Edit::insert("p", 1),
        Edit::insert("a", 2),
        Edit::insert("c", 3),
        Edit::insert("e", 4),
        Edit::delete("e", 3),
        Edit::delete("p", 4),
        Edit::delete("t", 5),
        Edit::delete("r", 6),
        Edit::delete("c", 8),
        Edit::delete("o", 10),
        Edit::insert("e", 9),
    ];

    assert_eq!(desired, edits);
}

#[test]
fn test_diff_4() {
    let source = fs::read_to_string("test/notes-1.org").unwrap();
    let source_lines = source.lines().collect::<Vec<_>>();
    let target = fs::read_to_string("test/notes-2.org").unwrap();
    let target_lines = target.lines().collect::<Vec<_>>();
    let edits = diff(&source_lines, &target_lines);
    let desired = vec![
        Edit::delete("def diff(a: str, b: str):", 17),
        Edit::delete("    N, M = len(a), len(b)", 18),
        Edit::delete("    for D in range(0, M + N + 1):", 19),
        Edit::delete("        for k in range(-D, D + 1):", 20),
        Edit::delete(
            "            # Find the endpoint of the furthest reaching D-path in diagonal k",
            21,
        ),
        Edit::delete("            Dpath, endpoint = ...", 22),
        Edit::delete("            if endpoint == (N, M):", 23),
        Edit::delete("                return Dpath", 24),
        Edit::delete("#+end_src", 25),
        Edit::delete("", 26),
        Edit::delete("#+begin_src python :results silent", 27),
        Edit::insert("#+end_src", 24),
        Edit::delete("class Edit:", 36),
        Edit::delete("    index: int", 37),
        Edit::delete("    kind: EditKind", 38),
        Edit::delete("    char: Optional[str]", 39),
        Edit::delete("#+end_src", 40),
        Edit::insert("** Adding some text", 26),
        Edit::insert("    for D in range(0, M + N + 1):", 27),
        Edit::insert("        for k in range(-D, D + 1, 2):", 28),
        Edit::insert(
            "            if k == -D or (k != D and V.get(k - 1, -1) < V.get(k + 1, -1)):",
            29,
        ),
        Edit::insert("                x = V[k + 1]", 30),
        Edit::insert("            else:", 31),
        Edit::insert("                x = V[k - 1] + 1", 32),
        Edit::insert("", 33),
        Edit::insert("            y = x - k", 34),
    ];

    assert_eq!(desired, edits);
}

#[test]
fn test_diff_5() {
    let source = fs::read_to_string("test/sample-1.org").unwrap();
    let source_lines = source.lines().collect::<Vec<_>>();
    let target = fs::read_to_string("test/sample-2.org").unwrap();
    let target_lines = target.lines().collect::<Vec<_>>();
    let edits = diff(&source_lines, &target_lines);
    let desired = vec![
        Edit::delete("Here is a line that will be deleted.", 2),
        Edit::insert("This one was added!", 3),
        Edit::delete("This is another deleted line.", 7),
        Edit::delete("*** This subheading will be deleted too", 8),
        Edit::delete("Along with all its content", 9),
        Edit::delete("", 10),
        Edit::delete("def foobar():", 18),
        Edit::delete("    return \"bazbar\"", 19),
        Edit::insert("def bazbar():", 14),
        Edit::insert("    return \"foobar\"", 15),
        Edit::insert("", 17),
        Edit::insert("* This is a new heading", 18),
        Edit::insert("This wasn't in the other file.", 19),
    ];

    assert_eq!(desired, edits);
}
