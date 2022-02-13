use std::fs;

#[derive(Debug)]

pub struct TestCase {
    pub id: i32,
    pub n: usize,
    pub col: i32,
    pub data: Vec<Vec<i32>>, //x, y koordinate
}

pub fn import_tc(tc: i32) -> Option<TestCase> {
    if tc <= 0 || tc > 30 {
        return None;
    }
    let filename = "poplavljanje-unix.in";
    let contents = fs::read_to_string(filename).expect("File not found");
    let lines: Vec<&str> = contents.trim_end_matches("\n").split("\n").collect();
    let mut iter = lines.iter();
    iter.next();
    iter.next();
    for i in 1..tc + 1 {
        iter.next();
        let actual_tc: i32 = iter
            .next()
            .expect("File ended before tc was found")
            .trim()
            .parse()
            .unwrap();
        if actual_tc != i {
            panic!("Test case doesn't match expected test case");
        }
        let desc = iter.next().expect("File ended on description line").trim();
        let ddesc: Vec<&str> = desc.split(" ").collect();
        let w: usize = ddesc[0].parse().unwrap();
        let h: usize = ddesc[1].parse().unwrap();
        let col: i32 = ddesc[2].parse().unwrap();
        let mut vec: Vec<Vec<i32>> = vec![vec![0; h]; w];
        for y in 0..h {
            let line: Vec<&str> = iter
                .next()
                .expect("Parsing rows, out of lines")
                .trim()
                .split(" ")
                .collect();
            for x in 0..w {
                vec[x][y] = line[x].parse().unwrap();
            }
        }

        if actual_tc == tc {
            return Some(TestCase {
                id: tc,
                n: h as usize,
                col: col,
                data: vec,
            });
        }
    }

    return None;
}
