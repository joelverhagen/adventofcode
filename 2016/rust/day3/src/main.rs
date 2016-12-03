use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::SplitWhitespace;

#[derive(Debug)]
struct Triangle {
    side_0: i32,
    side_1: i32,
    side_2: i32,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        // Find the largest side.
        let (mut a, mut b, mut c) = (self.side_0, self.side_1, self.side_2);

        if a < c {
            let temp = a;
            a = c;
            c = temp;
        }

        if a < b {
            let temp = a;
            a = b;
            b = temp;
        }

        // At this point, a contains the largest side. If the sum of the two smaller sides is larger than the largest
        // side, this is a valid triangle.

        b + c > a
    }
}

#[derive(Debug)]
enum TriangleParseError {
    CouldNotOpenFile,
    NotEnoughSides(i32, i32),
    InvalidSide(i32, i32, String),
}

fn parse_triangle_side(line_index: i32, pieces: &mut SplitWhitespace, side_index: i32) -> Result<i32, TriangleParseError> {
    let unparsed_side = match pieces.next() {
        None    => return Err(TriangleParseError::NotEnoughSides(line_index, side_index)),
        Some(s) => s,
    };

    match unparsed_side.parse::<i32>() {
        Err(_)   => Err(TriangleParseError::InvalidSide(line_index, side_index, unparsed_side.to_string())),
        Ok(side) => Ok(side),
    }
}

fn parse_triangle_line(line_index: i32, line: &str) -> Result<Triangle, TriangleParseError> {
    let mut pieces = line.split_whitespace();

    let side_0 = parse_triangle_side(line_index, &mut pieces, 0)?;
    let side_1 = parse_triangle_side(line_index, &mut pieces, 1)?;
    let side_2 = parse_triangle_side(line_index, &mut pieces, 2)?;

    Ok(Triangle {
        side_0: side_0,
        side_1: side_1,
        side_2: side_2,
    })
}

fn parse_triangle_file(path: &str) -> Result<Vec<Triangle>, TriangleParseError> {
    let file = match File::open(path) {
        Err(_)   => return Err(TriangleParseError::CouldNotOpenFile),
        Ok(file) => file,
    };

    let file_reader = BufReader::new(file);
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut line_index = 0;

    for line_result in file_reader.lines() {
        let line = match line_result {
            Err(_)   => return Err(TriangleParseError::CouldNotOpenFile),
            Ok(line) => line,
        };

        let triangle = parse_triangle_line(line_index, &line)?;
        triangles.push(triangle);

        line_index += 1;
    }

    Ok(triangles)
}

fn get_part_1_result(triangles: &Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|&t| t.is_valid())
        .count()
}

fn main() {
    let triangles = parse_triangle_file("input.txt").unwrap();

    let part_1_result = get_part_1_result(&triangles);    
    println!("Part 1 result: {}", part_1_result);
}
