pub mod model;
#[cfg(test)]
mod tests;

use model::{Edit, EditKind, Slice, Snake, SnakeSplit};
use std::collections::HashMap;

#[allow(non_snake_case)]
/// Finds the longest snake (contiguous match) that is roughly an equal edit distance
/// from the start and end of a and b.
/// Returns the regions before and after the snake.
pub fn midsnake<'l>(a: Slice<'l>, b: Slice<'l>) -> SnakeSplit<'l> {
    let N = a.len() as isize;
    let M = b.len() as isize;
    let MAX = ((N + M) as usize).div_ceil(2) as isize;
    let DELTA = N - M;

    let mut forward_reach: HashMap<isize, isize> = HashMap::new();
    forward_reach.insert(1, 0);
    let mut backward_reach: HashMap<isize, isize> = HashMap::new();
    backward_reach.insert(1, 0);

    let mut longest_snake = Snake {
        start: (0, 0),
        end: (0, 0),
    };

    for D in 0..MAX {
        // Start forward search in diagonals -D..=D
        for k in (-D..=D).step_by(2) {
            let mut x = if k == -D {
                forward_reach[&(k + 1)]
            } else if k != D && forward_reach[&(k + 1)] > forward_reach[&(k - 1)] {
                forward_reach[&(k + 1)]
            } else {
                forward_reach[&(k - 1)] + 1
            };

            let mut y = x - k;
            let mut snake_len = 0;
            while (0..N).contains(&x) && (0..M).contains(&y) && a[x as usize] == b[y as usize] {
                x += 1;
                y += 1;
                snake_len += 1;
            }

            let snake = Snake {
                start: (x - snake_len, y - snake_len),
                end: (x, y),
            };

            if longest_snake.len() < snake_len {
                longest_snake = snake.clone();
            }

            forward_reach.insert(k, x);

            // always prefer the longest snake to achieve the cleanest diff
            if DELTA % 2 != 0 && snake_len >= longest_snake.len() {
                // get furthest reaching reverse path in same diagonal
                if let Some(b_x) = backward_reach.get(&(-k + DELTA)) {
                    // combined paths span total length of a
                    if x + *b_x + 1 > N {
                        println!("forward");
                        return snake.split_slices(a, b);
                    }
                }
            }
        }

        // Start backward search in diagonals -D..=D
        for k in (-D..=D).step_by(2) {
            let mut x = if k == -D {
                backward_reach[&(k + 1)]
            } else if k != D && backward_reach[&(k + 1)] > backward_reach[&(k - 1)] {
                backward_reach[&(k + 1)]
            } else {
                backward_reach[&(k - 1)] + 1
            };

            let mut y = x - k;
            let mut snake_len = 0;
            while (0..N).contains(&x)
                && (0..M).contains(&y)
                && a[(N - x - 1) as usize] == b[(M - y - 1) as usize]
            {
                x += 1;
                y += 1;
                snake_len += 1;
            }

            let snake = Snake {
                start: (N - x, M - y),
                end: (N - (x - snake_len), M - (y - snake_len)),
            };

            if longest_snake.len() < snake_len {
                longest_snake = snake.clone();
            }

            backward_reach.insert(k, x);

            // always prefer the longest snake to achieve the cleanest diff
            if DELTA % 2 == 0 && snake_len >= longest_snake.len() {
                // get furthest reaching forward path in same diagonal
                if let Some(f_x) = forward_reach.get(&(-k + DELTA)) {
                    // combined paths span total length of a
                    if *f_x + x + 1 > N {
                        println!("backward");
                        return snake.split_slices(a, b);
                    }
                }
            }
        }
    }

    // At this point both searches have finished and evidently did not meet in a snake
    // Return the longest snake we found, if any
    longest_snake.split_slices(a, b)
}

/// Returns the edits required to transform a into b.
pub fn diff<'l>(mut a: Slice<'l>, mut b: Slice<'l>) -> Vec<Edit<'l>> {
    let mut edits = vec![];
    let mut a_pos = 0;
    let mut b_pos = 0;

    while !a.is_empty() && !b.is_empty() && a.first().unwrap() == b.first().unwrap() {
        a = &a[1..];
        b = &b[1..];
        a_pos += 1;
        b_pos += 1;
    }

    while !a.is_empty() && !b.is_empty() && a.last().unwrap() == b.last().unwrap() {
        a = &a[..a.len() - 1];
        b = &b[..b.len() - 1];
    }

    if a.is_empty() {
        while !b.is_empty() {
            edits.push(Edit::insert(b.first().unwrap(), b_pos));
            b = &b[1..];
            b_pos += 1;
        }
        return edits;
    } else if b.is_empty() {
        while !a.is_empty() {
            edits.push(Edit::delete(a.first().unwrap(), a_pos));
            a = &a[1..];
            a_pos += 1;
        }
        return edits;
    }

    match (a.len(), b.len()) {
        (1, 1) => {
            edits.push(Edit::delete(a.first().unwrap(), a_pos));
            edits.push(Edit::insert(b.first().unwrap(), b_pos));
            return edits;
        }
        (1, _) => {
            let a_line = a.first().unwrap();
            let a_in_b_ = b.iter().position(|b_line| a_line == b_line);
            match a_in_b_ {
                None => {
                    edits.push(Edit::delete(a_line, a_pos));

                    while !b.is_empty() {
                        edits.push(Edit::insert(b.first().unwrap(), b_pos));
                        b = &b[1..];
                        b_pos += 1;
                    }
                    return edits;
                }
                Some(a_in_b) => {
                    for line in &b[..a_in_b] {
                        edits.push(Edit::insert(line, b_pos));
                        b_pos += 1;
                    }

                    for line in &b[a_in_b + 1..] {
                        edits.push(Edit::insert(line, b_pos));
                        b_pos += 1;
                    }

                    return edits;
                }
            };
        }
        (_, 1) => {
            let b_line = b.first().unwrap();
            let b_in_a_ = a.iter().position(|a_line| a_line == b_line);
            match b_in_a_ {
                None => {
                    while !a.is_empty() {
                        edits.push(Edit::delete(a.first().unwrap(), a_pos));
                        a = &a[1..];
                        a_pos += 1;
                    }

                    edits.push(Edit::insert(b_line, b_pos));
                    return edits;
                }
                Some(b_pos) => {
                    for line in &a[..b_pos] {
                        edits.push(Edit::delete(line, a_pos));
                        a_pos += 1;
                    }

                    for line in &a[b_pos + 1..] {
                        edits.push(Edit::delete(line, a_pos));
                        a_pos += 1;
                    }
                    return edits;
                }
            }
        }
        (_, _) => {}
    };

    let split = midsnake(a, b);

    if split.snake_len == 0 && split.a_second.is_none() && split.b_second.is_none() {
        while !a.is_empty() {
            edits.push(Edit::delete(a.first().unwrap(), a_pos));
            a = &a[1..];
            a_pos += 1;
        }

        while !b.is_empty() {
            edits.push(Edit::insert(b.first().unwrap(), b_pos));
            b = &b[1..];
            b_pos += 1;
        }

        return edits;
    }

    edits.extend(&mut diff(split.a_first, split.b_first).into_iter().map(
        |mut edit| match edit.kind {
            EditKind::Delete => {
                edit.pos += a_pos;
                edit
            }
            EditKind::Insert => {
                edit.pos += b_pos;
                edit
            }
        },
    ));

    edits.extend(
        &mut diff(split.a_second.unwrap(), split.b_second.unwrap())
            .into_iter()
            .map(|mut edit| match edit.kind {
                EditKind::Delete => {
                    edit.pos += a_pos + split.a_first.len() + split.snake_len;
                    edit
                }
                EditKind::Insert => {
                    edit.pos += b_pos + split.b_first.len() + split.snake_len;
                    edit
                }
            }),
    );

    edits
}
