use fem_comp;
fn main() {
    let mut test = fem_comp::create_obj_from_xlsx("test2.xlsx").expect("Create object error");
    test.c_s();
    print!("{:?}",test.s)
}
