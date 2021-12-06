use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut east = 0;
        let mut north = 0;
        let mut turn = 0;
        for line in input.lines() {
            let param = &line[1..].parse::<i32>().unwrap();
            match line.chars().next().unwrap() {
                'N' => north += param,
                'S' => north -= param,
                'E' => east += param,
                'W' => east -= param,
                'L' => turn += param,
                'R' => turn += 360 - param,
                'F' => match turn % 360 {
                    0 => east += param,
                    90 => north += param,
                    180 => east -= param,
                    270 => north -= param,
                    _ => panic!("bad angle"),
                },
                _ => panic!(),
            }
        }
        (east.abs() + north.abs()).into()
    }

    fn part2(&self, input: Input) -> Output {
        use vector2d::Vector2D;

        let mut way = Vector2D::new(10_f32, 1_f32);
        let mut pos: Vector2D<f32> = Vector2D::new(0_f32, 0_f32);
        for line in input.lines() {
            let param = &line[1..].parse::<f32>().unwrap();
            let mut rads = param / 180_f32 * std::f32::consts::PI;
            match line.chars().next().unwrap() {
                'N' => way.y += param,
                'S' => way.y -= param,
                'E' => way.x += param,
                'W' => way.x -= param,
                'L' => {
                    way = Vector2D::new(
                        way.x * rads.cos() - way.y * rads.sin(),
                        way.x * rads.sin() + way.y * rads.cos(),
                    )
                }
                'R' => {
                    rads = -rads;
                    way = Vector2D::new(
                        way.x * rads.cos() - way.y * rads.sin(),
                        way.x * rads.sin() + way.y * rads.cos(),
                    );
                }
                'F' => {
                    pos += way * *param;
                }
                _ => panic!(),
            }
        }
        (pos.x.abs() + pos.y.abs()).into()
    }
}
