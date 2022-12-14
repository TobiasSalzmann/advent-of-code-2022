use std::collections::HashMap;
use itertools::Itertools;
use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day17.txt");

    println!("Day 17, Part 1: {:?}", height_after(input[0].clone(), 2022));
    println!("Day 17, Part 2: {:?}", height_after(input[0].clone(), 1_000_000_000_000));
}

fn height_after(wind: String, number_of_rocks: usize) -> usize {
    let mut tower = vec![];
    let rocks = shapes();
    let mut wind_it = wind.chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => panic!("parse error")
        })
        .cycle();
    let mut record: HashMap<String, (usize, usize)> = HashMap::new();

    for rock_idx in 0..number_of_rocks {
        simulate_rock_fall(&mut tower, &rocks[rock_idx % rocks.len()], &mut wind_it);
        let x = 100;
        if tower.len() >= x {
            let tower_top = tower[(tower.len() - x)..tower.len()].iter()
                .rev()
                .map(|row| row.iter().map(|x| if x.clone() {'#'} else {'.'}).join("") )
                .join("\n");
            let current_tower_height = tower.len();
            if record.contains_key(&tower_top) {
                let (prefix_rocks, prefix_height) = record.get(&tower_top).unwrap().clone();
                let cycle_rocks = rock_idx - prefix_rocks;
                let cycle_height = current_tower_height - prefix_height;

                let number_of_cycles = (number_of_rocks - prefix_rocks) / cycle_rocks;
                let remaining_rocks = (number_of_rocks - prefix_rocks) % cycle_rocks;
                let height_adjustment = number_of_cycles * cycle_height;

                let prefix_suffix_height = height_after(wind.clone(), prefix_rocks + remaining_rocks);
                return prefix_suffix_height + height_adjustment;
            } else {
                record.insert(tower_top, (rock_idx, current_tower_height));
            }
        }
    }

    tower.len()
}

fn simulate_rock_fall(stack: &mut Vec<Vec<bool>>, rock: &Shape, wind: &mut dyn Iterator<Item=i32>) {
    let mut y = stack.len() as i32 + 3;
    let mut x = 2;
    loop {
        let wind_modifier = wind.next().unwrap();
        let collision_after_wind = detect_collision(stack, rock, x + wind_modifier, y);
        if !collision_after_wind { x += wind_modifier }
        let collision_after_fall = detect_collision(stack, rock, x, y - 1);
        if collision_after_fall { break; }
        y -= 1
    }
    place_rock(stack, rock, x, y);
}

fn detect_collision(stack: &Vec<Vec<bool>>, rock: &Shape, x: i32, y: i32) -> bool {
    // println!("{}, {}", x, y);
    for (shape_x, shape_y) in rock.clone() {
        let moved_x = shape_x + x;
        let moved_y = shape_y + y;
        let hits_wall = moved_x < 0 || moved_x >= 7;
        let hits_floor = moved_y < 0;
        if hits_wall || hits_floor {
            return true;
        }
        let hits_tower = moved_y < (stack.len() as i32) && stack[moved_y as usize][moved_x as usize];
        if hits_wall || hits_tower {
            return true;
        }
    }
    false
}

fn place_rock(stack: &mut Vec<Vec<bool>>, rock: &Shape, x: i32, y: i32) {
    for (shape_x, shape_y) in rock {
        let moved_x = shape_x + x;
        let moved_y = shape_y + y;
        while (stack.len() as i32) < moved_y + 1 {
            stack.push(vec![false, false, false, false, false, false, false])
        }
        stack[moved_y as usize][moved_x as usize] = true
    }
}

type Shape = Vec<(i32, i32)>;

fn shapes() -> Vec<Shape> {
    let minus = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus = vec![
        /*    */(1, 2),
        (0, 1), (1, 1), (2, 1),
        /*    */(1, 0),
    ];
    let j = vec![
        /*            */(2, 2),
        /*            */(2, 1),
        (0, 0), (1, 0), (2, 0),
    ];
    let i = vec![
        (0, 3),
        (0, 2),
        (0, 1),
        (0, 0),
    ];
    let square = vec![
        (0, 1), (1, 1),
        (0, 0), (1, 0),
    ];
    vec![minus, plus, j, i, square]
}

#[cfg(test)]
mod tests {
    use crate::day17::*;

    #[test]
    fn should_solve_part_1_on_example() {
        let wind = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        let rocks = 2022;
        assert_eq!(height_after(wind, rocks), 3068);
    }

    #[test]
    fn should_solve_using_cycles() {
        let wind = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        let rocks = 1_000_000_000_000;
        assert_eq!(height_after(wind, rocks), 1514285714288);
    }
}
