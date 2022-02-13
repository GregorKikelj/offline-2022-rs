mod importer;
mod solver;
use solver::ReducedTC;

const TC: i32 = 7;

fn main() {
    let grid: importer::TestCase = importer::import_tc(TC).expect("Testcase not found");
    let gr: ReducedTC = ReducedTC::convert_tc(grid);
    println!("{}", gr.n);
    println!("{:?}", gr.c);
    println!("{:?}", gr.col);
    for i in 0..gr.n {
        println!("{}, {:?}", i, gr.edge[i]);
    }
    let cols = solver::random_move_solver(gr);
    println!("{:?}", cols);
}
