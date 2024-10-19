mod model;
#[cfg(test)]
mod tests;

use model::{Edit, EditKind, Slice, SnakeSplit};
use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn midsnake<'l>(mut a: Slice<'l>, mut b: Slice<'l>) -> SnakeSplit<'l> {
    let N = a.len() as isize;
    let M = b.len() as isize;
    let MAX = ((N + M) as usize).div_ceil(2) as isize;
    let DELTA = N - M;

    let mut forward_reach: HashMap<isize, isize> = HashMap::new();
    forward_reach.insert(1, 0);
    let mut backward_reach: HashMap<isize, isize> = HashMap::new();
    backward_reach.insert(1, 0);

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
                println!(
                    "forward snake! x: {x}, y: {y}, a: {}, b: {}, k: {k}",
                    a[x as usize], b[y as usize]
                );
                println!("-------------\n\n");
                x += 1;
                y += 1;
                snake_len += 1;
            }

            forward_reach.insert(k, x);

            if DELTA % 2 != 0 {
                println!("{}", k + DELTA);
                dbg!(&backward_reach);
                if let Some(b_x) = backward_reach.get(&(-k + DELTA)) {
                    if x + *b_x + 2 > N {
                        println!("forward");
                        println!("{x} + {b_x} > {N} | k: {k}");
                        dbg!(backward_reach);
                        let pre_snake_x = (x - snake_len) as usize;
                        let pre_snake_y = (y - snake_len) as usize;
                        let post_snake_x = x as usize;
                        let post_snake_y = y as usize;
                        return SnakeSplit {
                            a_first: &a[..pre_snake_x],
                            b_first: &b[..pre_snake_y],
                            a_second: Some(&a[post_snake_x..]),
                            b_second: Some(&b[post_snake_y..]),
                            snake_len: snake_len as usize,
                        };
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
            println!("\n------------");
            dbg!(&x);
            dbg!(&y);
            dbg!(&k);
            let mut snake_len = 0;
            while (0..N).contains(&x)
                && (0..M).contains(&y)
                && a[(N - x - 1) as usize] == b[(M - y - 1) as usize]
            {
                println!(
                    "back snake! x: {x}, y: {y}, a: {}, b: {}, k: {k}",
                    a[(N - x) as usize],
                    b[(M - y) as usize]
                );
                println!("------ \n\n");
                x += 1;
                y += 1;
                snake_len += 1;
            }

            backward_reach.insert(k, x);

            if DELTA % 2 == 0 {
                if let Some(f_x) = forward_reach.get(&(-k + DELTA)) {
                    if *f_x + x > N {
                        println!("backward");
                        println!("{f_x} + {x} > {N} | k: {k}, y: {y}");
                        dbg!(&forward_reach);
                        let pre_snake_x = (N - x) as usize;
                        let pre_snake_y = (M - y) as usize;
                        let post_snake_x = (N - (x - snake_len)) as usize;
                        let post_snake_y = (M - (y - snake_len)) as usize;
                        return SnakeSplit {
                            a_first: &a[..pre_snake_x],
                            b_first: &b[..pre_snake_y],
                            a_second: Some(&a[post_snake_x..]),
                            b_second: Some(&b[post_snake_y..]),
                            snake_len: snake_len as usize,
                        };
                    }
                }
            }
        }
    }

    SnakeSplit {
        a_first: a,
        b_first: b,
        a_second: None,
        b_second: None,
        snake_len: 0,
    }
}

/// Returns the edits required to tranform a into b.
pub fn diff<'l>(mut a: Slice<'l>, mut b: Slice<'l>) -> Vec<Edit<'l>> {
    let mut edits = vec![];

    let mut pos = 0;
    while !a.is_empty() && !b.is_empty() && a.first().unwrap() == b.first().unwrap() {
        a = &a[1..];
        b = &b[1..];
        pos += 1;
    }

    while !a.is_empty() && !b.is_empty() && a.last().unwrap() == b.last().unwrap() {
        a = &a[..a.len() - 1];
        b = &b[..b.len() - 1];
    }

    if a.is_empty() {
        while !b.is_empty() {
            edits.push(Edit {
                kind: EditKind::INSERT,
                line: b.first().unwrap(),
                pos,
            });

            pos += 1;
            b = &b[1..];
        }
    } else if b.is_empty() {
        while !a.is_empty() {
            edits.push(Edit {
                kind: EditKind::DELETE,
                line: a.first().unwrap(),
                pos,
            });

            pos += 1;
            a = &a[1..];
        }
    }

    match (a.len(), b.len()) {
        (1, 1) => {
            edits.push(Edit {
                kind: EditKind::DELETE,
                line: a.first().unwrap(),
                pos,
            });
            edits.push(Edit {
                kind: EditKind::INSERT,
                line: b.first().unwrap(),
                pos,
            });
            return edits;
        }
        (1, _) => {
            let a_line = a.first().unwrap();
            let a_pos = b.iter().position(|b_line| a_line == b_line);
            match a_pos {
                None => {
                    edits.push(Edit {
                        kind: EditKind::DELETE,
                        line: a_line,
                        pos,
                    });

                    while !b.is_empty() {
                        edits.push(Edit {
                            kind: EditKind::INSERT,
                            line: b.first().unwrap(),
                            pos,
                        });

                        pos += 1;
                        b = &b[1..];
                    }
                    return edits;
                }
                Some(a_pos) => {
                    for line in &b[..a_pos] {
                        edits.push(Edit {
                            kind: EditKind::INSERT,
                            line,
                            pos,
                        });
                        pos += 1;
                    }
                    pos += 1;
                    for line in &b[a_pos + 1..] {
                        edits.push(Edit {
                            kind: EditKind::INSERT,
                            line,
                            pos,
                        });
                        pos += 1;
                    }
                    return edits;
                }
            };
        }
        (_, 1) => {
            let b_line = a.first().unwrap();
            let b_pos = a.iter().position(|a_line| a_line == b_line);
            match b_pos {
                None => {
                    while !a.is_empty() {
                        edits.push(Edit {
                            kind: EditKind::DELETE,
                            line: a.first().unwrap(),
                            pos,
                        });

                        pos += 1;
                        a = &a[1..];
                    }

                    edits.push(Edit {
                        kind: EditKind::INSERT,
                        line: b_line,
                        pos,
                    });

                    return edits;
                }
                Some(b_pos) => {
                    for line in &a[..b_pos] {
                        edits.push(Edit {
                            kind: EditKind::DELETE,
                            line,
                            pos,
                        });
                        pos += 1;
                    }
                    pos += 1;
                    for line in &a[b_pos + 1..] {
                        edits.push(Edit {
                            kind: EditKind::DELETE,
                            line,
                            pos,
                        });
                        pos += 1;
                    }
                    return edits;
                }
            }
        }
        (_, _) => {}
    };

    return edits;
}
