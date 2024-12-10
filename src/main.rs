mod rc;
mod test;
mod r#box;

fn main() {


    println!("******* Testing module rc ********");

    let tree = rc::big_tree();
    rc::print(&tree);

    let rb = rc::rb_tree();
    rc::print(&rb);
    println!("'A' inside: {}", rc::rb_set::mem(&'A', &rb));
    println!("'B' inside: {}", rc::rb_set::mem(&'B', &rb));

    test::run_test(10_000, rc::rb_set::empty(), rc::rb_set::insert, rc::rb_set::mem);

    test::run_test(100_000, rc::rb_set::empty(), rc::rb_set::insert, rc::rb_set::mem);

    test::run_test(1_000_000, rc::rb_set::empty(), rc::rb_set::insert, rc::rb_set::mem);


    println!("******* Testing module box ********");


    let rb = r#box::rb_tree();
    r#box::print(&rb);
    println!("'A' inside: {}", r#box::rb_set::mem(&'A', &rb));
    println!("'B' inside: {}", r#box::rb_set::mem(&'B', &rb));

    test::run_test(10_000, r#box::rb_set::empty(), r#box::rb_set::insert, r#box::rb_set::mem);

    test::run_test(100_000, r#box::rb_set::empty(), r#box::rb_set::insert, r#box::rb_set::mem);

    test::run_test(1_000_000, r#box::rb_set::empty(), r#box::rb_set::insert, r#box::rb_set::mem);
}
