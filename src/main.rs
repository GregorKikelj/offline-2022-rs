#![allow(dead_code)]
mod importer;
mod solver;
mod output;
mod mcts;
use solver::ReducedTC;

const TC: i8 = 15;

fn main() {
    let grid: importer::TestCase = importer::import_tc(TC).expect("Testcase not found");
    let gr: ReducedTC = ReducedTC::convert_tc(grid);
    let cols = mcts::solve(gr);
    println!("{}", cols.len());
    // output::write_output(&TC, &cols);
}


