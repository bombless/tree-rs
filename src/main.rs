mod rc;
mod test;

fn main() {
    use rc::rb_set::{empty, insert, mem};
    let tree = rc::big_tree();
    rc::print(&tree);

    let rb = rc::rb_tree();
    rc::print(&rb);
    println!("'A' inside: {}", mem(&'A', &rb));
    println!("'B' inside: {}", mem(&'B', &rb));

    test::run_test(10_000, empty(), insert, mem);

    test::run_test(100_000, empty(), insert, mem);

    test::run_test(1_000_000, empty(), insert, mem);
}
