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
        println!("{}", String::from_utf8(line).unwrap());
    }
}



fn read_file(filename: &str) -> Vec<Vec<u8>>  {
    let mut lreader = LineReader::new(File::open(filename).unwrap());
    let mut lines: Vec<Vec<u8>> = Vec::new();

    lreader.for_each(|line| {
        let tokens = line.into_iter()
            .filter(|chr| !chr.is_ascii_whitespace())
            .map(|chr| *chr)
            .collect();
        lines.push(tokens);
        Ok(true)
    });
    lines
}


