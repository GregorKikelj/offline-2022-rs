mod importer;

const TC : i32 = 2;
fn main() {
    let x = importer::import_tc(TC);
    dbg!(x);
}
