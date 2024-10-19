mod model;
#[cfg(test)]
mod tests;

use model::{Edit, Slice, Snake, SnakeSplit};
use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn midsnake<'l>(a: Slice<'l>, b: Slice<'l>) -> SnakeSplit<'l> {
    let N = a.len() as isize;
    let M = b.len() as isize;
    let MAX = ((N + M) as usize).div_ceil(2) as isize;
    let DELTA = N - M;

    let mut forward_reach: HashMap<isize, isize> = HashMap::new();
    forward_reach.insert(1, 0);
    let mut backward_reach: HashMap<isize, isize> = HashMap::new();
    backward_reach.insert(1, 0);

    let mut longest_snake = None;

    for D in 0..(MAX as isize) {
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

            forward_reach.insert(k, x);

            let snake = Snake {
                start: (x - snake_len, y - snake_len),
                end: (x, y),
            };

            if DELTA % 2 != 0 {
                if let Some(b_x) = backward_reach.get(&(-k + DELTA)) {
                    if x + *b_x + 2 > N {
                        return snake.split_slices(a, b);
                    }
                }
            }

            match longest_snake {
                None => {
                    longest_snake = Some(snake);
                }
                Some(ref longest) => {
                    if longest.len() < snake_len {
                        longest_snake = Some(snake);
                    }
                }
            }
        }

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

            backward_reach.insert(k, x);

            if DELTA % 2 == 0 {
                if let Some(f_x) = forward_reach.get(&(-k + DELTA)) {
                    if *f_x + x > N {
                        return snake.split_slices(a, b);
                    }
                }
            }

            match longest_snake {
                None => {
                    longest_snake = Some(snake);
                }
                Some(ref longest) => {
                    if longest.len() < snake_len {
                        longest_snake = Some(snake);
                    }
                }
            }
        }
    }

    if let Some(snake) = longest_snake {
        return snake.split_slices(&a, &b);
    }

    SnakeSplit {
        a_first: a,
        b_first: b,
        a_second: None,
        b_second: None,
        snake_len: 0,
    }
}

/// Returns the edits required to transform a into b.
pub fn diff<'l>(mut a: Slice<'l>, mut b: Slice<'l>) -> Vec<Edit<'l>> {
    let mut edits = vec![];

    while !a.is_empty() && !b.is_empty() && a.first().unwrap() == b.first().unwrap() {
        a = &a[1..];
        b = &b[1..];
    }

    while !a.is_empty() && !b.is_empty() && a.last().unwrap() == b.last().unwrap() {
        a = &a[..a.len() - 1];
        b = &b[..b.len() - 1];
    }

    if a.is_empty() {
        while !b.is_empty() {
            edits.push(Edit::insert(b.first().unwrap()));
            b = &b[1..];
        }
    } else if b.is_empty() {
        while !a.is_empty() {
            edits.push(Edit::delete(a.first().unwrap()));
            a = &a[1..];
        }
    }

    match (a.len(), b.len()) {
        (1, 1) => {
            edits.push(Edit::delete(a.first().unwrap()));
            edits.push(Edit::insert(b.first().unwrap()));
            return edits;
        }
        (1, _) => {
            let a_line = a.first().unwrap();
            let a_pos = b.iter().position(|b_line| a_line == b_line);
            match a_pos {
                None => {
                    edits.push(Edit::delete(a_line));

                    while !b.is_empty() {
                        edits.push(Edit::insert(b.first().unwrap()));
                        b = &b[1..];
                    }
                    return edits;
                }
                Some(a_pos) => {
                    for line in b[..a_pos].iter().chain(&b[a_pos + 1..]) {
                        edits.push(Edit::insert(line));
                    }
                    return edits;
                }
            };
        }
        (_, 1) => {
            let b_line = b.first().unwrap();
            let b_pos = a.iter().position(|a_line| a_line == b_line);
            match b_pos {
                None => {
                    while !a.is_empty() {
                        edits.push(Edit::delete(a.first().unwrap()));
                        a = &a[1..];
                    }

                    edits.push(Edit::insert(b_line));
                    return edits;
                }
                Some(b_pos) => {
                    for line in a[..b_pos].iter().chain(&a[b_pos + 1..]) {
                        edits.push(Edit::delete(line));
                    }
                    return edits;
                }
            }
        }
        (_, _) => {}
    };

    let split = midsnake(a, b);

    if split.snake_len == 0 {
        while a.len() > 0 {
            edits.push(Edit::delete(a.first().unwrap()));
            a = &a[1..];
        }

        while b.len() > 0 {
            edits.push(Edit::insert(b.first().unwrap()));
            b = &b[1..];
        }

        return edits;
    }

    edits.append(&mut diff(split.a_first, split.b_first));

    edits.append(&mut diff(split.a_second.unwrap(), split.b_second.unwrap()));

    return edits;
}
