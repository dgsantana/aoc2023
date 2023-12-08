#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub left_string: String,
    pub right_string: String,
    pub node_ref: NodeRef,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeRef {
    pub index: usize,
    pub left: usize,
    pub right: usize,
}

impl Node {
    pub fn from_line(input: &str) -> Self {
        let mut parts = input.split(" = ");
        let name = parts.next().unwrap().to_string();
        let to_parse = parts.next().unwrap().to_string();
        let (left_string, right_string) = to_parse.split_once(", ").expect("Invalid input");
        let left_string = left_string[1..].to_string();
        let right_string = right_string[..right_string.len() - 1].to_string();

        Self {
            name,
            left_string,
            right_string,
            node_ref: NodeRef {
                index: 0,
                left: 0,
                right: 0,
            },
        }
    }
}
