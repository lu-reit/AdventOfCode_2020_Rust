use std::fs::File;
use linereader::LineReader;
use std::time::Instant;

 
type Num = u64; 
 
enum Op {
    Scalar(Num),
    Add(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>)
}


fn main() {
    let lines = read_file("input");

    let p1_timer = Instant::now();
    let p1_result = part1(&lines);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);
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
    // (which may either be a scalar or the result of an
    // expression)
    let (mut val, mut i) = if line[0] == b'(' { 
            eval1(&line[1..]) 
        } else { 
            (line[0] as Num, 2) 
    };
    loop {
        if line[i - 1] == b')' { return (val, i + 2); }

        let (rval, new_i) = if line[i] == b'(' { 
            eval1(&line[(i + 1)..])
        } else {
            (line[i] as Num, 2)
        };
        if line[i - 1] == b'+' { val += rval; } else { val *= rval; }
        i += new_i;
    }
}

fn eval2(line: &[u8]) -> (Num, usize) {

}


fn read_file(filename: &str) -> Vec<Vec<u8>>  {
    let mut lreader = LineReader::new(File::open(filename).unwrap());
    let mut lines: Vec<Vec<u8>> = Vec::new();

    lreader.for_each(|line| {
        let mut tokens: Vec<u8> = line.into_iter()
            .filter(|chr| !chr.is_ascii_whitespace())
            .map(|chr| if chr.is_ascii_digit() { *chr - 48 } else { *chr })
            .collect();
        tokens.push(b')'); // Cough Cough :D
        lines.push(tokens);
        Ok(true)
    });
    lines
}


