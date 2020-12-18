use std::fs::File;
use linereader::LineReader;

 
type Num = u32; 
 
enum Op {
    Scalar(Num),
    Add(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>)
}


fn main() {
    let lines = read_file("test");

    for line in lines {
        println!("{:?}", line);
    }
}

fn part1(lines: &[Vec<u8>]) -> Num {
    let mut sum = 0;
    for line in lines {
    }

    sum
}
 
fn to_num(ascii: u8) -> Num {
    ascii as Num - 48
}

fn eval1(line: &[u8]) -> (Num, usize) {
    let (mut val, mut i) = if line[0] == b'(' { eval1(line[1..]) } else { (line, 1) }



fn read_file(filename: &str) -> Vec<Vec<u8>>  {
    let mut lreader = LineReader::new(File::open(filename).unwrap());
    let mut lines: Vec<Vec<u8>> = Vec::new();

    lreader.for_each(|line| {
        let tokens = line.into_iter()
            .filter(|chr| !chr.is_ascii_whitespace())
            .map(|chr| if chr.is_ascii_digit() { *chr - 48 } else { *chr })
            .collect();
        lines.push(tokens);
        Ok(true)
    });
    lines
}


