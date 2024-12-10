use Color::*;
use Tree::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use crate::rc::Element::{VirtualNode, VisibleNode};

#[derive(Copy, Clone)]
enum Color {Red, Black}

pub enum Tree<T> {
    Leaf,
    Node(T, Rc<Tree<T>>, Rc<Tree<T>>),
}

impl<T> Tree<(Color, T)> {
    fn unwrap(&self) -> &T {
        match self { Node((_, v), _, _) => v, Leaf => panic!() }
    }
}

impl<T> Tree<T> {
    fn depth(&self) -> u32 {
        match self {
            Leaf => 0,
            Node(_, l, r) => 1 + u32::max(l.depth(), r.depth()),
        }
    }
    fn left(&self) -> Rc<Tree<T>> {
        match self {
            Node(_, node, _) => node.clone(),
            _ => Rc::new(Leaf),
        }
    }
    fn right(&self) -> Rc<Tree<T>> {
        match self {
            Node(_, _, node) => node.clone(),
            _ => Rc::new(Leaf),
        }
    }
    fn set_left(&mut self, node: Rc<Tree<T>>) {
        match self {
            Leaf => panic!(),
            Node(_, ptr, _) => *ptr = node,
        }
    }
    fn set_right(&mut self, node: Rc<Tree<T>>) {
        match self {
            Leaf => panic!(),
            Node(_, _, ptr) => *ptr = node,
        }
    }
}

trait IsRed {
    fn is_red(&self) -> bool;
}

impl<T> IsRed for Option<Rc<Tree<(Color, T)>>> {
    fn is_red(&self) -> bool {
        match self {
            Some(x) => x.is_red(),
            None => false,
        }
    }
}

impl<T> Tree<(Color, T)> {
    fn is_red(&self) -> bool {
        match self {
            Node((Red, _), _, _) => true,
            _ => false,
        }
    }
}

fn as_ref<T>(rc: &Option<Rc<Tree<T>>>) -> Option<&Tree<T>> {
    let optional_rc = rc.as_ref();
    match optional_rc {
        Some(r) => Some(&*r),
        _ => None,
    }
}

fn get_first_level_2<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    match t {
        Node(_, level2left, _) => level2left.clone(),
        _ => Rc::new(Leaf),
    }
}

fn get_first_level_3<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    let first_level_2 = get_first_level_2(t);
    match &*first_level_2 {
        Node(_, level3, _) => level3.clone(),
        _ => Rc::new(Leaf),
    }
}

fn get_second_level_3<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    let first_level_2 = get_first_level_2(t);
    match &*first_level_2 {
        Node(_, _, level3) => level3.clone(),
        _ => Rc::new(Leaf),
    }
}

fn get_second_level_2<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    match t {
        Node(_, _, level2right) => level2right.clone(),
        _ => Rc::new(Leaf),
    }
}

fn get_third_level_3<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    let second_level_2 = get_second_level_2(t);
    match &*second_level_2 {
        Node(_, level3, _) => level3.clone(),
        _ => Rc::new(Leaf),
    }
}

fn get_fourth_level_3<T>(t: &Tree<T>) -> Rc<Tree<T>> {
    let second_level_2 = get_second_level_2(t);
    match &*second_level_2 {
        Node(_, _, level3) => level3.clone(),
        _ => Rc::new(Leaf),
    }
}

fn place<T>((x, y, z): (T, T, T), (a, b, c, d): (Rc<Tree<(Color, T)>>, Rc<Tree<(Color, T)>>, Rc<Tree<(Color, T)>>, Rc<Tree<(Color, T)>>)) -> Tree<(Color, T)> {
    Node((Red, y), Rc::new(Node((Black, x), a, b)), Rc::new(Node((Black, z), c, d)))
}

fn balance<T: Clone>(t: &Tree<(Color, T)>) -> Tree<(Color, T)> {
    let first_level_2 = get_first_level_2(t);
    let second_level_2 = get_second_level_2(t);
    let first_level_3 = get_first_level_3(t);
    let second_level_3 = get_second_level_3(t);
    let third_level_3 = get_third_level_3(t);
    let fourth_level_4 = get_fourth_level_3(t);
    if first_level_2.is_red() && first_level_3.is_red() {
        let a = first_level_3.left();
        let b = first_level_3.right();
        let c = first_level_2.right();
        let d = t.right();
        let x = first_level_3.unwrap().clone();
        let y = first_level_2.unwrap().clone();
        let z = t.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if first_level_2.is_red() && second_level_3.is_red() {
        let a = first_level_2.left();
        let b = second_level_3.left();
        let c = second_level_3.right();
        let d = t.right();
        let x = first_level_2.unwrap().clone();
        let y = second_level_3.unwrap().clone();
        let z = t.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if second_level_2.is_red() && fourth_level_4.is_red() {
        let a = first_level_2;
        let b = third_level_3;
        let c = fourth_level_4.left();
        let d = fourth_level_4.right();
        let x = t.unwrap().clone();
        let y = second_level_2.unwrap().clone();
        let z = fourth_level_4.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if second_level_2.is_red() && third_level_3.is_red() {
        let a = first_level_2;
        let b = third_level_3.left();
        let c = third_level_3.right();
        let d = second_level_2.right();
        let x = t.unwrap().clone();
        let y = third_level_3.unwrap().clone();
        let z = second_level_2.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else {
        match t {
            Leaf => Leaf,
            Node(v, l, r) => Node(v.clone(), l.clone(), r.clone())
        }
    }
}

fn black_root<T>(t: Tree<(Color, T)>) -> Tree<(Color, T)> {
    match t {
        Leaf => Leaf,
        Node((_, v), l, r) => Node((Black, v), l, r),
    }
}

fn insert_aux<T: Ord + Clone>(v: T, t: &Tree<(Color, T)>) -> Tree<(Color, T)> {
    match t {
        Leaf => Node((Red, v), Rc::new(Leaf), Rc::new(Leaf)),
        Node((c, nv), lt, rt) => {
            if &v == nv {
                Node((*c, nv.clone()), lt.clone(), rt.clone())
            }
            else if &v < nv {
                Node((*c, nv.clone()), Rc::new(balance(&insert_aux(v, lt))), rt.clone())
            } else {
                Node((*c, nv.clone()), lt.clone(), Rc::new(balance(&insert_aux(v, rt))))
            }
        }
    }
}

fn insert<T: Ord + Clone>(v: T, t: &Tree<(Color, T)>) -> Tree<(Color, T)> {
    black_root(insert_aux(v, t))
}

fn height_of_depth(n: u32) -> u32 {
    match n {
        _ if n <= 1 => 0,
        2 => 2,
        3 => 3,
        _ => 1 + 2 * height_of_depth(n - 1),
    }
}

fn leftmost_space_of_depth(n: u32) -> u32 {
    match n {
        _ if n <= 1 => 0,
        2 => 2,
        _ => leftmost_space_of_depth(n - 1) + height_of_depth(n) + 1,
    }
}

fn normal_space_of_depth(n: u32) -> u32 {
    leftmost_space_of_depth(n + 1) - 1
}

fn get_padding_tree<T: Clone>(t: &Tree<T>, depth: u32, is_left: bool, is_leftmost: bool) -> Tree<(u32, T)> {
    match t {
        Leaf => Leaf,
        Node(v, l, r) => {
            let padding = if is_leftmost {
                leftmost_space_of_depth(depth)
            } else {
                if is_left && depth == 1 { 1 } else { normal_space_of_depth(depth) }
            };
            let left = get_padding_tree(l, depth - 1, true, is_leftmost);
            let right = get_padding_tree(r, depth - 1, false, false);
            Node((padding, v.clone()), Rc::new(left), Rc::new(right))
        }
    }
}

#[derive(Clone, Debug)]
enum Element<T> {
    VirtualLeft,
    VirtualRight,
    VirtualNode,
    VisibleNode(T, bool, bool),
    VisibleLeft,
    VisibleRight,
}
use Element::*;

impl<T> Tree<(u32, Option<T>)> {
    fn is_visual_leaf(&self) -> bool {
        match self {
            Leaf => true,
            Node((_, Some(_)), _, _) => false,
            Node((_, None), _, _) => true,
        }
    }
}

fn generate_first_line<T: Clone>(lst: Vec<&Tree<(u32, Option<T>)>>) -> Vec<(u32, Element<T>)> {
    let mut ret = Vec::new();
    for x in lst {
        match x {
            Node ((p, Some(c)), l, r) => {
                ret.push((*p, VisibleNode(c.clone(), l.is_visual_leaf(), r.is_visual_leaf())))
            }
            Node((p, None), _, _) => {
                ret.push((*p, VirtualNode))
            }
            Leaf => {}
        }
    }
    ret
}

fn generate_next_line<T>(line: Vec<(u32, Element<T>)>) -> Vec<(u32, Element<T>)> {
    let mut first = true;
    let mut ret = Vec::new();
    for (n, c) in line {
        let offset = if first { 1 } else { 2 };
        first = false;
        match c {
            VisibleLeft => ret.push((n - offset, VisibleLeft)),
            VirtualLeft => ret.push((n - offset, VirtualLeft)),
            VisibleRight => ret.push((n + offset, VisibleRight)),
            VirtualRight => ret.push((n + offset, VirtualRight)),
            VisibleNode(_, left_is_leaf, right_is_leaf) => {
                let left_bar = if left_is_leaf {
                    VirtualLeft
                } else {
                    VisibleLeft
                };
                let right_bar = if right_is_leaf {
                    VirtualRight
                } else {
                    VisibleRight
                };
                ret.push((n, left_bar));
                ret.push((1, right_bar));
            }
            VirtualNode => {
                ret.push((n, VirtualLeft));
                ret.push((1, VirtualRight));
            }
        }
    }
    ret
}

fn generate_lines<T: Clone + Debug>(count_down: u32, mut nodes: Vec<(u32, Element<T>)>) -> Vec<Vec<(u32, Element<T>)>> {
    let mut ret = Vec::new();
    for _ in 0 .. count_down {
        ret.push(nodes.clone());
        nodes = generate_next_line(nodes);
    }
    ret.push(nodes);
    ret
}

fn as_full_tree<T: Clone>(t: &Tree<T>, depth: u32) -> Tree<Option<T>> {
    match t {
        Leaf if depth > 0 => Node (None, Rc::new(as_full_tree(&Leaf, depth - 1)), Rc::new(as_full_tree(&Leaf, depth - 1))),
        Leaf => Leaf,
        Node(v, l, r) => Node(Some(v.clone()), Rc::new(as_full_tree(l, depth - 1)), Rc::new(as_full_tree(r, depth - 1))),
    }
}

fn children_of_nodes<T>(lst: Vec<&Tree<T>>) -> Vec<&Tree<T>> {
    let mut ret = Vec::new();
    for x in lst {
        if let Node(_, l, r) = x {
            ret.push(&**l);
            ret.push(&**r);
        }
    }
    ret
}

fn list_of_nodes<T>(lst: Vec<&Tree<T>>, depth: u32) -> Vec<Vec<&Tree<T>>> {
    let mut ret = vec![lst.clone()];
    let children = children_of_nodes(lst);
    if depth < 1 { return ret }
    ret.extend(list_of_nodes(children, depth - 1));
    ret
}

fn lines_of_nodes<T: Clone + Debug>(x: Vec<&Tree<(u32, Option<T>)>>) -> Vec<Vec<(u32, Element<T>)>> {
    let depth = x[0].depth();
    if depth > 1 {
        let diff = leftmost_space_of_depth(depth) - leftmost_space_of_depth(depth - 1);
        let count_down = if diff > 1 { diff - 1 } else { 0 };
        if count_down <= 0 { return vec![generate_first_line(x)] }
        generate_lines(count_down, generate_first_line(x))
    } else {
        vec![generate_first_line(x)]
    }
}

fn get_lines<T: Clone + Debug>(t: &Tree<T>) -> Vec<Vec<(u32, Element<T>)>> {
    let depth = t.depth();
    let full_tree = as_full_tree(t, depth);
    let padding_tree = get_padding_tree(&full_tree, depth, true, true);
    let lists = list_of_nodes(vec![&padding_tree], depth);
    lists.into_iter().map(lines_of_nodes).collect::<Vec<_>>().concat()
}

fn print_line<T: Display + Clone>(line: Vec<(u32, Element<T>)>) {
    for (n, elem) in line {
        match elem {
            VisibleNode(c, _, _) => {
                for _ in 0 .. n {
                    print!(" ");
                }
                print!("{c}");
            }
            VisibleLeft => {
                for _ in 0 .. n {
                    print!(" ");
                }
                print!("/");
            }
            VisibleRight => {
                for _ in 0 .. n {
                    print!(" ");
                }
                print!("\\");
            }
            _ => {
                for _ in 0 ..= n {
                    print!(" ");
                }
            }
        }
    }
    println!()
}

pub fn print<T: Display + Clone + Debug>(t: &Tree<T>) {
    for line in get_lines(t) {
        print_line(line)
    }
}

mod char {
    use std::fmt::{Debug, Display, Formatter};
    use std::rc::Rc;
    use super::Tree::{*,self};

    pub fn leaf() -> Rc<Tree<C>> {
        Rc::new(Leaf)
    }

    pub fn node(c: char, l: Rc<Tree<C>>, r: Rc<Tree<C>>) -> Tree<C> {
        Node(C(c), l, r)
    }

    #[derive(Clone)]
    pub struct C(char);
    impl Display for C {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "|{}|", self.0)
        }
    }
    impl Debug for C {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "'{}'", self.0)
        }
    }
}

fn rc<T>(v: T) -> Rc<T> {
    Rc::new(v)
}


pub fn big_tree() -> Tree<char::C> {
    use char::{leaf, node};
    let a = node('A', leaf(), leaf());
    let c = node('C', leaf(), leaf());
    let e = node('E', leaf(), leaf());
    let g = node('G', leaf(), leaf());
    let b = node('B', rc(a), rc(c));
    let f = node('F', rc(e), rc(g));
    let d = node('D', rc(b), rc(f));
    d
}