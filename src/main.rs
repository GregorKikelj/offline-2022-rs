mod importer;

const TC : i32 = 3;
fn main() {
    let x = importer::import_tc(TC).expect("Testcase not found");
    println!("{}, {}, {}", x.id, x.n, x.col);
}
