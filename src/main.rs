mod importer;
mod solver;

const TC : i32 = 7;
fn main() {
    let grid: importer::TestCase = importer::import_tc(TC).expect("Testcase not found");
    let gr = solver::ReducedTC::convert_tc(grid);
    println!("{}", gr.n);
    println!("{:?}", gr.c);
    for i in 0..gr.n{
        println!("{}, {:?}", i, gr.edge[i]);
    }
}
