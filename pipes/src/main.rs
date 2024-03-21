use std::collections::HashMap;

use parser::{ReplyInput, ReplyOutput};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pipes {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl From<&String> for Pipes {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => unreachable!(),
        }
    }
}

impl Pipes {
    pub fn path(&self, from: Direction) -> Option<Direction> {
        match self {
            Self::A if from == Direction::Right => Some(Direction::Up),
            Self::A if from == Direction::Down => Some(Direction::Left),
            Self::B if from == Direction::Right => Some(Direction::Down),
            Self::B if from == Direction::Up => Some(Direction::Left),
            Self::C if from == Direction::Left => Some(Direction::Down),
            Self::C if from == Direction::Up => Some(Direction::Right),
            Self::D if from == Direction::Down => Some(Direction::Right),
            Self::D if from == Direction::Left => Some(Direction::Up),
            Self::E if from == Direction::Down => Some(Direction::Down),
            Self::E if from == Direction::Up => Some(Direction::Up),
            Self::F if from == Direction::Left => Some(Direction::Left),
            Self::F if from == Direction::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_in(
        &self,
        (x, y): (usize, usize),
        (width, height): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Self::Up if y != 0 => Some((x, y - 1)),
            Self::Down if y != height - 1 => Some((x, y + 1)),
            Self::Left if x != 0 => Some((x - 1, y)),
            Self::Right if x != width - 1 => Some((x + 1, y)),
            _ => None,
        }
    }
}

fn connect_pipes(
    map: Vec<Vec<String>>,
    head: (usize, usize),
    dest: (usize, usize),
    direction: Direction,
    tiles: HashMap<Pipes, u32>,
    wrong_direction: &[bool],
) -> Option<Vec<Vec<String>>> {
    let mut dx = (dest.1 - head.1) as f32;
    let dy = (dest.0 - head.0) as f32;

    if dx == 0. {
        dx = 0.01;
    }
    let gradient = dy / dx;

    let mut placements = Vec::new();
    for (tile, count) in tiles.iter() {
        if *count == 0 {
            continue;
        }

        let new_direction = if let Some(new) = tile.path(direction) {
            new
        } else {
            continue;
        };

        let wrong_directions = wrong_direction.iter().filter(|b| **b).count();
        let mut wrong_direction = wrong_direction.to_vec();
        if new_direction == Direction::Up || new_direction == Direction::Right {
            wrong_direction.push(true);
            if wrong_directions > 8 {
                continue;
            }
        } else {
            wrong_direction.push(false);
        }

        let new_position = if let Some(new) = new_direction.move_in(head, (map[0].len(), map.len()))
        {
            new
        } else {
            continue;
        };

        if map[new_position.1][new_position.0] != "0" && map[new_position.1][new_position.0] != "X"
        {
            continue;
        }

        let mut tiles = tiles.clone();
        let mut map = map.clone();

        *tiles.get_mut(tile).unwrap() -= 1;
        map[head.1][head.0] = format!("{tile:?}");

        if wrong_direction.len() > 12 {
            wrong_direction.remove(0);
        }

        if new_position == dest {
            return Some(map);
        }

        placements.push((map, new_position, new_direction, tiles, wrong_direction));
    }

    if gradient > 0. && gradient < 1. {
        if dx > 0. {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 0,
                        Direction::Up => 1,
                        Direction::Down => 2,
                        Direction::Left => 3,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        } else {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 0,
                        Direction::Up => 1,
                        Direction::Down => 2,
                        Direction::Left => 3,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        }
    } else if gradient > 1. {
        if dx > 0. {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 1,
                        Direction::Up => 0,
                        Direction::Down => 3,
                        Direction::Left => 2,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        } else {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 1,
                        Direction::Up => 0,
                        Direction::Down => 3,
                        Direction::Left => 2,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        }
    } else if gradient < 0. && gradient > -1. {
        if dx > 0. {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 0,
                        Direction::Up => 2,
                        Direction::Down => 1,
                        Direction::Left => 3,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        } else {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 0,
                        Direction::Up => 2,
                        Direction::Down => 1,
                        Direction::Left => 3,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        }
    } else {
        if dx > 0. {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 1,
                        Direction::Up => 3,
                        Direction::Down => 0,
                        Direction::Left => 2,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        } else {
            placements.sort_by(|this, other| {
                fn map_num(direction: Direction) -> u8 {
                    match direction {
                        Direction::Right => 1,
                        Direction::Up => 3,
                        Direction::Down => 0,
                        Direction::Left => 2,
                    }
                }

                map_num(this.2).cmp(&map_num(other.2))
            });
        }
    }

    for (new_map, new_position, new_direction, tiles, wrong_direction) in placements {
        if let Some(res) = connect_pipes(new_map, new_position, dest, new_direction, tiles, &wrong_direction) {
            return Some(res)
        }
    }

    return None;
}

fn main() {
    let input = ReplyInput::load(
        "./case.txt".into(),
        Box::new(|meta| (meta[0] as usize + 1, HashMap::new())),
    );

    let res = input
        .cases
        .into_iter()
        .map(|mut case| {
            let tiles: HashMap<Pipes, u32> = HashMap::from_iter(
                case.data
                    .remove(0)
                    .chunks(2)
                    .map(|arr| (Pipes::from(&arr[0]), arr[1].parse().unwrap())),
            );

            let map = case.data;

            let dest = map
                .iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .find_map(|(x, cell)| {
                            if cell == "X" {
                                return Some(x);
                            } else {
                                return None;
                            }
                        })
                        .map(|found| (found, y))
                })
                .unwrap();

            if map[1][0] == "0" {
                if let Some(connected) = connect_pipes(
                    map.clone(),
                    (0, 1),
                    dest,
                    Direction::Down,
                    tiles.clone(),
                    &[],
                ) {
                    return format!(
                        "\n{}",
                        connected
                            .into_iter()
                            .map(|row| row.join(" "))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
            }

            if map[0][1] == "0" {
                if let Some(connected) = connect_pipes(
                    map.clone(),
                    (1, 0),
                    dest,
                    Direction::Right,
                    tiles.clone(),
                    &[],
                ) {
                    return format!(
                        "\n{}",
                        connected
                            .into_iter()
                            .map(|row| row.join(" "))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
            }

            unreachable!()
        })
        .collect::<Vec<_>>();

    ReplyOutput(res).save("output.txt".into())
}
