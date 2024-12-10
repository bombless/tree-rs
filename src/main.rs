mod rc;

fn main() {
    let tree = rc::big_tree();
    rc::print(&tree);

    let rb = rc::rb_tree();
    rc::print(&rb);
}
