use std::fmt::Display;
pub use TreeElement as Tree;

const EMPTY: &str = "   ";
const EDGE: &str = " └─";
const PIPE: &str = " │ ";
const BRANCH: &str = " ├─";

#[derive(Clone)]
pub enum TreeElement {
    Node(Node),
    Leaf(Leaf)
}

impl From<String> for TreeElement {
    fn from(value: String) -> Self {
        Leaf::new(value).into()
    }
}

impl From<&str> for TreeElement {
    fn from(value: &str) -> Self {
        Leaf::new(value).into()
    }
}

impl From<Node> for TreeElement {
    fn from(value: Node) -> Self {
        TreeElement::Node(value)
    }
}

impl From<Leaf> for TreeElement {
    fn from(value: Leaf) -> Self {
        TreeElement::Leaf(value)
    }
}

impl<T: Into<TreeElement>> From<(String, Vec<T>)> for TreeElement {
    fn from(value: (String, Vec<T>)) -> Self {
        Node {
            title: value.0,
            children: value.1.into_iter().map(|v| v.into()).collect()
        }.into()
    }
}

#[derive(Clone)]
pub struct Node {
    pub title: String,
    pub children: Vec<TreeElement>
}

impl Node {
    pub fn new<T: Into<String>>(title: T) -> Node {
        Node {
            title: title.into(),
            children: vec![]
        }
    }

    pub fn push<T: Into<TreeElement>>(&mut self, element: T) {
        self.children.push(element.into());
    }

    pub fn write_tree(&self, f: &mut std::fmt::Formatter<'_>, parts: Vec<&str>) -> std::fmt::Result {
        writeln!(f, " {}", self.title)?;

        let last_pos = match self.children.len() {
            0 => 0,
            len => len - 1
        };
        let formatted_parts = parts.join("");

        for (i, child) in self.children.iter().enumerate() {
            write!(f, "{}", formatted_parts)?;

            let last = last_pos == i;
            let mut next_parts = parts.clone();

            if last {
                next_parts.push(&EMPTY);
                write!(f, "{}", EDGE)?;
            } else {
                next_parts.push(&PIPE);
                write!(f, "{}", BRANCH)?;
            }

            match child {
                TreeElement::Node(node) => {
                    node.write_tree(f, next_parts)?;
                },
                TreeElement::Leaf(leaf) => {
                    for (i, line) in leaf.text.lines().enumerate() {
                        if i != 0 { write!(f, "{}", next_parts.join(""))?; }

                        writeln!(f, " {}", line)?;
                    }
                }
            };
        }

        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_tree(f, vec![])
    }
}

#[derive(Clone)]
pub struct Leaf {
    text: String
}

impl Leaf {
    pub fn new<T: Into<String>>(text: T) -> Leaf {
        Leaf {
            text: text.into()
        }
    }
}

#[test]
fn test() {
    let l1 = Leaf::new("line1\nline2\nline3\nline4");
    let l2 = Leaf::new("only one line");

    let mut n1 = Node::new("node 1");
    n1.push(l1.clone());
    n1.push(l2.clone());

    let mut n2 = Node::new("node 2");
    n2.push(l2.clone());
    n2.push(l1.clone());
    n2.push(l2.clone());

    let mut n3 = Node::new("node 3");
    n3.push(n1.clone());
    n3.push(l1.clone());
    n3.push(l2.clone());

    let mut n4 = Node::new("node 4");
    n4.push(n1);
    n4.push(n2);
    n4.push(n3);

    println!("{}", n4);
}
