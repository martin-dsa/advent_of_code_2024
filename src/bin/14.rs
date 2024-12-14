use std::{collections::HashMap, str::FromStr};

use advent_of_code::Vec2;
use fancy_regex::Regex;
use image::{ImageBuffer, RgbImage};
use itertools::Itertools;
use lazy_static::lazy_static;

advent_of_code::solution!(14);

static X: usize = 101;
static Y: usize = 103;

lazy_static! {
    static ref RE: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

#[derive(Debug)]
struct Robot {
    position: Vec2<usize>,
    velocity: Vec2<i32>,
}

impl FromStr for Robot {
    type Err = fancy_regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = RE.captures(s).map(|c| {
            let c = c.unwrap();

            let px = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let py = c.get(2).unwrap().as_str().parse::<usize>().unwrap();

            let vx = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let vy = c.get(4).unwrap().as_str().parse::<i32>().unwrap();

            Self {
                position: Vec2 { x: px, y: py },
                velocity: Vec2 { x: vx, y: vy },
            }
        });
        a
    }
}

fn move_robot(r: &mut Robot) {
    let new_x = (((r.position.x as i32 + X as i32) + r.velocity.x) % X as i32) as usize;
    let new_y = (((r.position.y as i32 + Y as i32) + r.velocity.y) % Y as i32) as usize;
    r.position.x = new_x;
    r.position.y = new_y;
}

fn quadrant(pos: &Vec2<usize>) -> Option<usize> {
    match pos {
        p if p.x < (X / 2) && p.y < (Y / 2) => Some(1),
        p if p.x < (X / 2) && p.y > (Y / 2) => Some(2),
        p if p.x > (X / 2) && p.y < (Y / 2) => Some(3),
        p if p.x > (X / 2) && p.y > (Y / 2) => Some(4),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = input
        .lines()
        .map(|l| l.parse::<Robot>().unwrap())
        .collect_vec();

    for _ in 0..100 {
        for r in robots.iter_mut() {
            move_robot(r);
        }
    }

    let quadrants = robots.iter().filter_map(|x| quadrant(&x.position)).fold(
        HashMap::new(),
        |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        },
    );

    let res = quadrants.values().product::<usize>();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = input
        .lines()
        .map(|l| l.parse::<Robot>().unwrap())
        .collect_vec();

    for i in 0..10000 {
        let mut image: RgbImage = ImageBuffer::new(X as u32, Y as u32);

        for r in robots.iter_mut() {
            move_robot(r);
            *image.get_pixel_mut(r.position.x as u32, r.position.y as u32) =
                image::Rgb([255, 255, 255]);
        }
        image.save(format!("img/{}.png", i + 1)).unwrap();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
