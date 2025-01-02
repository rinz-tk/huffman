use std::cmp::Ordering;

pub struct Node {
    pub c: Option<u8>,
    pub f: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.f.cmp(&self.f);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.f == other.f;
    }
}

impl Eq for Node {}

impl Node {
    pub fn new() -> Node {
        Node {
            c: None,
            f: 0,
            left: None,
            right: None
        }
    }

    fn print_tree_r(tree: &Box<Node>, prefix: String, is_left: bool) {
        print!("{prefix}");
        print!("{}", if is_left { "├── " } else { "└── " });

        match tree.c {
            Some(ch) => {
                print!("{ch}");
                println!(": {} ", tree.f);
            },

            None => {
                print!("/\\");
                println!(": {} ", tree.f);

                let new_prefix = format!("{}{}", prefix, if is_left {"|   "} else {"    "});
                Self::print_tree_r(tree.left.as_ref().unwrap(), new_prefix.clone(), true);
                Self::print_tree_r(tree.right.as_ref().unwrap(), new_prefix, false);
            }
        }
    }

    pub fn print_tree(tree: &Box<Node>) {
        Self::print_tree_r(tree, String::from(""), false);
    }
}
