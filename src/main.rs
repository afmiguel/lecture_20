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

    pub fn push_back(&mut self, value: i32){
        match self{
            Self::Nil => *self = Self::Node(value,
                Box::new(Self::Nil)),
            Self::Node(_, tail) =>
                tail.push_back(value),
        }
    }
}

fn main() {
    let mut lista = LinkedListNode::Nil;
    lista.display_list();
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);
    lista.display_list();
}
