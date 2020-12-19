use std::fs::File;
use linereader::LineReader;
use std::time::Instant;
 
type Num = u64; 
 
fn main() {
    let parse_timer = Instant::now();
    let lines = read_file("input");
    let parse_time = parse_timer.elapsed();
    println!("Parse time: {:?}", parse_time);

    let p1_timer = Instant::now();
    let p1_result = part1(&lines);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&lines);
    let p2_time = p2_timer.elapsed();
    println!("Part2 result: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
}


fn part1(lines: &[Vec<u8>]) -> Num {
    let mut sum = 0;
    for line in lines {
        let (val, _ ) = eval1(&line);
        sum += val;
    }
    sum
}
 
fn eval1(line: &[u8]) -> (Num, usize) {
    // Get the initial value/index of the left-hand side
    // (which may either be a scalar or an expression).
    // i always points to the rhs
    let (mut val, mut i) = if line[0] == b'(' { 
            eval1(&line[1..]) 
        } else { 
            (line[0] as Num, 2) 
    };
    loop {
        // Return when we hit the delimiter at the operators position
        if line[i - 1] == b')' { return (val, i + 2); }

        let (rval, offset) = if line[i] == b'(' { 
            eval1(&line[(i + 1)..])
        } else {
            (line[i] as Num, 2)
        };
        if line[i - 1] == b'+' { val += rval; } else { val *= rval; }
        i += offset;
    }
}

fn part2(lines: &[Vec<u8>]) -> Num {
    let mut sum = 0;
    for line in lines {
        let (val, _ ) = eval2(0, &line);
        sum += val;
    }
    sum
}

fn eval2(pos: usize, line: &[u8]) -> (Num, usize) {
    let (lval, mut op_pos) = if line[pos] == b'(' {
        eval2(pos + 1, &line)
    } else {
        (line[pos] as Num, pos + 1)
    };

    // Collects the values to multiply 
    let mut to_mul: [Num; 6] = [1; 6];
    to_mul[0] = lval;
    let mut to_mul_i = 0; 

    loop {
        // Get the value for the right-hand side and the position 
        // of the next operator
        let rpos = op_pos + 1;
        let (rval, new_pos) = if line[rpos] == b'(' {
            eval2(rpos + 1, &line)
        } else {
            (line[rpos] as Num, rpos + 1)
        };
        if line[op_pos] == b'+' {
            to_mul[to_mul_i] += rval;
        } else { 
            to_mul_i += 1;
            to_mul[to_mul_i] = rval;
        }
        op_pos = new_pos;
        if line[op_pos] == b')' { break }
    }
    (to_mul.iter().product(), op_pos + 1)
}


fn read_file(filename: &str) -> Vec<Vec<u8>>  {
    let mut lreader = LineReader::new(File::open(filename).unwrap());
    let mut lines: Vec<Vec<u8>> = Vec::new();

    lreader.for_each(|line| {
        // Read the line as u8, remove white-space and map ascii-digits
        // to int-values. Also add a delimiter to the end. This allows us to treat
        // the top-level expressions the same way as any of its children 
        // during recursion
        let mut tokens: Vec<u8> = line.iter()
            .filter(|chr| !chr.is_ascii_whitespace())
            .map(|chr| if chr.is_ascii_digit() { *chr - 48 } else { *chr })
            .collect();
        tokens.push(b')'); // Add a delimiter :D
        lines.push(tokens);
        Ok(true)
    }).unwrap();
    lines
}

 

