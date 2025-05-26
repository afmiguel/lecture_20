enum LinkedListNode {
    Nil,
    Node(i32, Box<LinkedListNode>),
}

impl LinkedListNode {
    fn display_list(&self) {
        match self {
            Self::Nil => println!("Nil"),
            Self::Node(v, tail) => {
                print!("{}->", v);
                tail.display_list();
            }
        }
    }
}

fn main() {
    let mut lista = 
        LinkedListNode::Node(10, 
            Box::new(LinkedListNode::Nil)
        );
    lista.display_list();
}
