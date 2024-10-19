use super::*;
use paste::paste;

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
    split_from_strs!(desired, "aa", "b", "aa", "bb", 2);
    let actual = midsnake(&source, &target);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_1() {
    slice_from_str!(source, "axxaa");
    slice_from_str!(target, "bbxxbb");
    split_from_strs!(desired, "a", "bb", "aa", "bb", 2);
    let actual = midsnake(&source, &target);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_2() {
    slice_from_str!(source, "axa");
    slice_from_str!(target, "bbxbb");
    split_from_strs!(desired, "a", "bb", "a", "bb", 1);
    let actual = midsnake(&source, &target);

    assert_eq!(desired, actual);
}

#[test]
fn test_midsnake_3() {
    slice_from_str!(source, "axa");
    slice_from_str!(target, "bxbbb");
    split_from_strs!(desired, "a", "b", "a", "bbb", 1);
    let actual = midsnake(&source, &target);

    assert_eq!(desired, actual);
}

#[test]
fn test_diff_0() {
    slice_from_str!(source, "sheeptractor");
    slice_from_str!(target, "spaceheater");
    let edits = diff(&source, &target);
    dbg!(edits);
}

#[test]
fn test_diff_1() {
    slice_from_str!(source, "aaaaa");
    slice_from_str!(target, "aabbaa");
    let edits = diff(&source, &target);
    dbg!(edits);
}
