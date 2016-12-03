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
    InvalidNumberOfTriangles,
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

fn parse_triangle_file_line_by_line(path: &str) -> Result<Vec<Triangle>, TriangleParseError> {
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

fn get_column_triangles_at(index: usize, triangles: &Vec<Triangle>) -> (Triangle, Triangle, Triangle)  {
    let a = &triangles[index];
    let b = &triangles[index + 1];
    let c = &triangles[index + 2];

    (
        Triangle {
            side_0: a.side_0,
            side_1: b.side_0,
            side_2: c.side_0,
        },
        Triangle {
            side_0: a.side_1,
            side_1: b.side_1,
            side_2: c.side_1,
        },
        Triangle {
            side_0: a.side_2,
            side_1: b.side_2,
            side_2: c.side_2,
        }
    )
}

fn get_triangles_by_column(triangles_by_line: &Vec<Triangle>) -> Result<Vec<Triangle>, TriangleParseError> {
    if triangles_by_line.len() % 3 != 0 {
        return Err(TriangleParseError::InvalidNumberOfTriangles);
    }

    let mut output: Vec<Triangle> = Vec::new();

    for i in 0..(triangles_by_line.len() / 3) {
        let (a, b, c) = get_column_triangles_at(3 * i, &triangles_by_line);
        output.push(a);
        output.push(b);
        output.push(c);
    }

    Ok(output)
}

fn get_count_of_valid_triangles(triangles: &Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|&t| t.is_valid())
        .count()
}

fn main() {
    let triangles_by_line = parse_triangle_file_line_by_line("input.txt").unwrap();
    let part_1_result = get_count_of_valid_triangles(&triangles_by_line); 
    println!("Part 1 result: {}", part_1_result);

    let triangles_by_column = get_triangles_by_column(&triangles_by_line).unwrap();
    let part_2_result = get_count_of_valid_triangles(&triangles_by_column); 
    println!("Part 2 result: {}", part_2_result);
}
