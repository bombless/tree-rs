use Tree::*;
use Color::*;

enum Tree<T, L, R> {
    Leaf,
    Node(T, L, R)
}
enum Color {
    Black, Red
}

type TreeLevel2<T, L> = Tree<T, Tree<T, L, L>, Tree<T, L, L>>;
type TreeLevel3<T, L> = Tree<T, TreeLevel2<T, L>, TreeLevel2<T, L>>;
type TreeLevel4<T, L> = TreeLevel2<T, L>;

fn balance<T, L5>(components:  ((Color, T), TreeLevel3<(Color, T), L5>, TreeLevel3<(Color, T), L5>)) -> TreeLevel3<(Color, T), Tree<(Color, T), L5, L5>> {
    match components {
        | ((Black, z), Node((Red, y), Node((Red, x), a, b), c), d)
        => place_l4_a_to_l3((x, y, z), a, b, c, d),
        | ((Black, z), Node ((Red, x), a, Node((Red, y), b, c)), d)
        => Node ((Red, y), Node((Black, x), a, b), Node((Black, z), c, d)),
        | ((Black, x), a, Node((Red, y), b, Node((Red, z), c, d)))
        => Node ((Red, y), Node((Black, x), a, b), Node((Black, z), c, d)),
        | ((Black, x), a, Node((Red, z), Node((Red, y), b, c), d))
        => Node ((Red, y), Node((Black, x), a, b), Node((Black, z), c, d)),
        | (n, l, r) => Node(n, l, r)
    }
}

fn place_l4_a_to_l3<T, L4>((x, y, z): (T, T, T), a: Tree<(Color, T), L4, L4>, b: TreeLevel2<(Color, T), L4>, c: TreeLevel3<(Color, T), L4>, d: TreeLevel3<(Color, T), L4>) -> TreeLevel2<(Color, T), L4> {
    Node ((Red, y), Node((Black, x), a, b), Node((Black, z), c, d))
}

fn place_l3_a_to_l3<T, L4>((x, y, z): (T, T, T), a: TreeLevel3<T, L4>, b: TreeLevel2<T, L4>, c: TreeLevel3<T, L4>, d: TreeLevel3<T, L4>) -> TreeLevel2<T, L4> {
    Node ((Red, y), Node((Black, x), a, b), Node((Black, z), c, d))
}

fn black_root<T, L>(x: TreeLevel3<(Color, T), L>) -> TreeLevel3<(Color, T), L> {
    match x {
        Leaf => Leaf,
        Node((_, v), l, r) => Node((Black, v), l, r)
    }
}
fn insert_aux<T, L>(v: T, x: TreeLevel4<(Color, T), L>) -> TreeLevel4<(Color, T), L> {
    match x {
        Leaf => Node((Red, x), Leaf, Leaf),
        Node ((c, nv), lt, rt) => if v == nv {
            Node ((c, nv), lt, rt)
        } else {
            if v < nv {
                balance(((c, nv), insert_aux(v, lt), rt))
            } else {
                balance(((c, nv), lt, insert_aux(v, rt)))
            }
        }
    }
}
fn insert<T, L>(v: T, x: TreeLevel4<(Color, T), L>) -> TreeLevel4<(Color, T), L> {
    black_root(insert_aux(v, x))
}
fn print_node(n: (Color, char)) {
    let red_text = "\027[31m";
    let reset_color = "\027[0m";
    match n.0 {
        Red => print!("{red_text}{}{reset_color}", n.1),
        _ => {}
    }
}