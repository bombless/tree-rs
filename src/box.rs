use Tree::*;
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Color {Red, Black}

#[derive(Clone)]
pub enum Tree<T: Clone> {
    Leaf,
    Node(T, Option<Box<Tree<T>>>, Option<Box<Tree<T>>>),
}

impl<T: Clone> Tree<T> {
    fn depth(&self) -> u32 {
        match self {
            Leaf => 0,
            Node(_, Some(l), Some(r)) => 1 + u32::max(l.depth(), r.depth()),
            Node(_, Some(l), _) => 1 + l.depth(),
            Node(_, _, Some(r)) => 1 + r.depth(),
            _ => 1,
        }
    }
    fn left(&self) -> Option<&Tree<T>> {
        match self {
            Node(_, Some(node), _) => Some(&**node),
            _ => None,
        }
    }
    fn right(&self) -> Option<&Tree<T>> {
        match self {
            Node(_, _, Some(node)) => Some(&**node),
            _ => None,
        }
    }
}

impl<T: MakeRB + Clone> Tree<T> {
    fn is_red(&self) -> bool {
        match self {
            Node(x, _, _) => x.is_red(),
            _ => false,
        }
    }
}

fn get_first_level_2<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    match t {
        Node(_, Some(level2left), _) => Some(&*level2left),
        _ => None,
    }
}

fn get_first_level_3<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    let first_level_2 = get_first_level_2(t);
    match first_level_2 {
        Some(Node(_, Some(level3), _)) => Some(&*level3),
        _ => None,
    }
}

fn get_second_level_3<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    let first_level_2 = get_first_level_2(t);
    match first_level_2 {
        Some(Node(_, _, Some(level3))) => Some(&*level3),
        _ => None,
    }
}

fn get_second_level_2<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    match t {
        Node(_, _, Some(level2right)) => Some(&*level2right),
        _ => None,
    }
}

fn get_third_level_3<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    let second_level_2 = get_second_level_2(t);
    match second_level_2 {
        Some(Node(_, Some(level3), _)) => Some(&*level3),
        _ => None,
    }
}

fn get_fourth_level_3<T: Clone>(t: &Tree<T>) -> Option<&Tree<T>> {
    let second_level_2 = get_second_level_2(t);
    match second_level_2 {
        Some(Node(_, _, Some(level3))) => Some(&*level3),
        _ => None,
    }
}

pub trait MakeRB {
    type Content;
    fn red(_: Self::Content) -> Self;
    fn black(_: Self::Content) -> Self;
    fn is_red(&self) -> bool;
    fn value(&self) -> &Self::Content;
    fn take_value(self) -> Self::Content;
}

impl<T: MakeRB + Clone> Tree<T> {
    fn unwrap(&self) -> &T::Content {
        match self {
            Node(v, _, _) => v.value(),
            _ => panic!(),
        }
    }
}

fn box_subtree<T: Clone>(x: &Tree<T>) -> Option<Box<Tree<T>>> where Tree<T>: Clone {
    Some(Box::new(x.clone()))
}

fn place<T: MakeRB + Clone>((x, y, z): (T::Content, T::Content, T::Content), (a, b, c, d): (&Tree<T>, &Tree<T>, &Tree<T>, &Tree<T>)) -> Tree<T> {
    Node(T::red(y),
         Some(Box::new(Node(T::black(x), box_subtree(a), box_subtree(b)))),
         Some(Box::new(Node(T::black(z), box_subtree(c), box_subtree(d)))))
}

trait IsRed {
    fn is_red(&self) -> bool;
}

impl<'a, T: Clone + MakeRB> IsRed for Option<&'a Tree<T>> where Tree<T>: Clone {
    fn is_red(&self) -> bool {
        match self {
            Some(x) => x.is_red(),
            _ => false
        }
    }
}

trait Unwrap {
    type Output;
    fn unwrap_left(&self) -> &Self::Output;
    fn unwrap_right(&self) -> &Self::Output;
}

impl<'a, T: Clone> Unwrap for Option<&'a Tree<T>> {
    type Output = Tree<T>;

    fn unwrap_left(&self) -> &Self::Output {
        match self {
            Some(Node(_, Some(l), _)) => &*l,
            _ => panic!()
        }
    }

    fn unwrap_right(&self) -> &Self::Output {
        match self {
            Some(Node(_, _, Some(r))) => &*r,
            _ => panic!()
        }
    }
}

fn balance<T: MakeRB>(t: &Tree<T>) -> Tree<T> where T::Content: Clone, T: Clone {
    let first_level_2 = get_first_level_2(t);
    let second_level_2 = get_second_level_2(t);
    let first_level_3 = get_first_level_3(t);
    let second_level_3 = get_second_level_3(t);
    let third_level_3 = get_third_level_3(t);
    let fourth_level_4 = get_fourth_level_3(t);
    if first_level_2.is_red() && first_level_3.is_red() {
        let a = first_level_3.unwrap_left();
        let b = first_level_3.unwrap_right();
        let c = first_level_2.unwrap_right();
        let d = match t.right() {
            Some(x) => &*x,
            None => &Leaf
        };
        let x = first_level_3.unwrap().unwrap().clone();
        let y = first_level_2.unwrap().unwrap().clone();
        let z = t.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if first_level_2.is_red() && second_level_3.is_red() {
        let a = first_level_2.unwrap_left();
        let b = second_level_3.unwrap_left();
        let c = second_level_3.unwrap_right();
        let d = match t.right() {
            Some(x) => &*x,
            None => &Leaf,
        };
        let x = first_level_2.unwrap().unwrap().clone();
        let y = second_level_3.unwrap().unwrap().clone();
        let z = t.unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if second_level_2.is_red() && fourth_level_4.is_red() {
        let a = first_level_2.unwrap();
        let b = third_level_3.unwrap();
        let c = fourth_level_4.unwrap_left();
        let d = fourth_level_4.unwrap_right();
        let x = t.unwrap().clone();
        let y = second_level_2.unwrap().unwrap().clone();
        let z = fourth_level_4.unwrap().unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else if second_level_2.is_red() && third_level_3.is_red() {
        let a = first_level_2.unwrap();
        let b = third_level_3.unwrap_left();
        let c = third_level_3.unwrap_right();
        let d = second_level_2.unwrap_right();
        let x = t.unwrap().clone();
        let y = third_level_3.unwrap().unwrap().clone();
        let z = second_level_2.unwrap().unwrap().clone();
        place((x, y, z), (a, b, c, d))
    }
    else {
        match t {
            Leaf => Leaf,
            Node(v, l, r) => Node(v.clone(), l.clone(), r.clone())
        }
    }
}

fn black_root<T: MakeRB + Clone>(t: Tree<T>) -> Tree<T> where T::Content: Clone {
    match t {
        Leaf => Leaf,
        Node(v, l, r) => Node(T::black(v.take_value()), l, r),
    }
}

fn leaf_subtree<T: Clone>() -> Option<Box<Tree<T>>> {
    Some(Box::new(Leaf))
}

fn insert_aux<T: MakeRB>(v: T::Content, t: &Tree<T>) -> Tree<T> where T::Content: Ord + Clone, T: Clone {
    match t {
        Leaf => Node(T::red(v), leaf_subtree(), leaf_subtree()),
        Node(x, lt, rt) => {
            let c = x.is_red();
            let nv = x.value();
            let make = if c {
                T::red
            } else {
                T::black
            };
            if &v == nv {
                Node(make(nv.clone()), Option::clone(lt), Option::clone(rt))
            }
            else if &v < nv {
                let lt = match lt {
                    Some(x) => &*x,
                    _ => &Leaf,
                };
                Node(make(nv.clone()), Some(Box::new(balance(&insert_aux(v, lt)))), Option::clone(rt))
            } else {
                let rt = match rt {
                    Some(x) => &*x,
                    _ => &Leaf,
                };
                Node(make(nv.clone()), Option::clone(lt), Some(Box::new(balance(&insert_aux(v, rt)))))
            }
        }
    }
}

fn insert<T: MakeRB>(v: T::Content, t: &Tree<T>) -> Tree<T> where T::Content: Ord + Clone, T: Clone {
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
            let l = match l {
                Some(x) => &*x,
                _ => &Leaf,
            };
            let left = get_padding_tree(l, depth - 1, true, is_leftmost);
            let r = match r {
                Some(x) => &*x,
                _ => &Leaf,
            };
            let right = get_padding_tree(r, depth - 1, false, false);
            Node((padding, v.clone()), Some(Box::new(left)), Some(Box::new(right)))
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

trait IsVisualLeaf {
    fn is_visual_leaf(&self) -> bool;
}

impl<T: Clone> IsVisualLeaf for Option<Box<Tree<(u32, Option<T>)>>> {
    fn is_visual_leaf(&self) -> bool {
        match self {
            None => true,
            Some(x) => match &**x {
                Leaf => true,
                Node((_, Some(_)), _, _) => false,
                Node((_, None), _, _) => true,
            }
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
        Leaf if depth > 0 => Node (None, Some(Box::new(as_full_tree(&Leaf, depth - 1))), Some(Box::new(as_full_tree(&Leaf, depth - 1)))),
        Leaf => Leaf,
        Node(v, l, r) => {
            let l = match l {
                Some(x) => &*x,
                _ => &Leaf,
            };
            let r = match r {
                Some(x) => &*x,
                _ => &Leaf,
            };
            Node(Some(v.clone()), Some(Box::new(as_full_tree(l, depth - 1))), Some(Box::new(as_full_tree(r, depth - 1))))
        }

    }
}

fn children_of_nodes<T: Clone>(lst: Vec<&Tree<T>>) -> Vec<&Tree<T>> {
    let mut ret = Vec::new();
    for x in lst {
        if let Node(_, l, r) = x {
            if let Some(x) = l {
                ret.push(&**x);
            }
            if let Some(x) = r {
                ret.push(&**x);
            }
        }
    }
    ret
}

fn list_of_nodes<T: Clone>(lst: Vec<&Tree<T>>, depth: u32) -> Vec<Vec<&Tree<T>>> {
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
            VirtualNode => {
                for _ in 0 .. n + 3 {
                    print!(" ");
                }
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



mod rb {
    use std::fmt::{Debug, Display, Formatter};
    use super::Color::{*};
    use super::MakeRB;

    #[derive(Clone, Debug)]
    pub struct RB<T: Clone + Ord>(pub(super) super::Color, pub(super) T);
    impl<T: Clone + Ord> MakeRB for RB<T> {
        type Content = T;

        fn red(v: Self::Content) -> Self {
            RB(Red, v)
        }

        fn black(v: Self::Content) -> Self {
            RB(Black, v)
        }

        fn is_red(&self) -> bool {
            self.0 == Red
        }

        fn value(&self) -> &Self::Content {
            &self.1
        }

        fn take_value(self) -> Self::Content {
            self.1
        }
    }
    impl Display for RB<char> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if self.0 == Black {
                return write!(f, "|{}|", self.1)
            }
            let red_text = "\x1b[31m";
            let reset_color = "\x1b[0m";
            write!(f, "|{red_text}{}{reset_color}|", self.1)
        }
    }
}

pub fn rb_tree() -> Tree<rb::RB<char>> {
    let insert = insert::<rb::RB<char>>;
    let t = Leaf;
    let t = insert('A', &t);
    let t = insert('C', &t);
    let t = insert('H', &t);
    t
}

pub mod rb_set {
    use super::Tree::{self, *};
    pub use super::rb::RB;
    pub fn empty<T: Clone>() -> Tree<T> {
        Leaf
    }
    pub fn insert<T: Ord + Clone>(v: T, t: &Tree<RB<T>>) -> Tree<RB<T>> {
        let insert = super::insert::<RB<T>>;
        insert(v, t)
    }
    pub fn mem<'a, 'b, T: Ord + Clone>(v: &'a T, t: &'b Tree<RB<T>>) -> bool {
        match t {
            Node(RB(_, nv), l, r) => if v == nv {
                true
            } else {
                if v < nv {
                    let l = match l {
                        Some(x) => &**x,
                        _ => return false,
                    };
                    mem(v, l)
                } else {
                    let r = match r {
                        Some(x) => &**x,
                        _ => return false,
                    };
                    mem(v, r)
                }
            }
            _ => false
        }
    }
}