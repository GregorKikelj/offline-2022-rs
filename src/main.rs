mod importer;
mod solver;
mod output;
use solver::ReducedTC;

const TC: i8 = 7;

fn main() {
    let grid: importer::TestCase = importer::import_tc(TC).expect("Testcase not found");
    let gr: ReducedTC = ReducedTC::convert_tc(grid);
    let cols = solver::random_move_solver(gr);
    println!("{}", cols.len());
    output::write_output(&TC, &cols);
}


