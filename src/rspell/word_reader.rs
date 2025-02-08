pub use self::word_reader::get_words_from_file;
pub mod word_reader {
    use std::io::Read;
    use std::fs::File;
    use itertools::Itertools;
    use unicode_segmentation::UnicodeSegmentation;
    use std::rc::Rc;
    use std::cmp;

    pub fn get_words_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut f = File::open(file_path)?;
        let buf = &mut Default::default();
        let _ = f.read_to_string(buf)?;
        let words_ref = buf.unicode_words().collect::<Vec<&str>>();

        Ok(words_ref.iter().map(|word_ref| word_ref.to_string()).collect())
    }

    pub fn get_words_from_file_as_tree(file_path: &str) -> Result<Node<String>, Box<dyn std::error::Error>> {
        // we could implement this as a tree, with the depth of an element pertaining how many
        // characters a word has
        let words = get_words_from_file(file_path)?;
        let mut root_node = Node::new(None);
        let mut word_so_far;
        for word in words {
            word_so_far = Box::new(String::new());
            for r#char in word.chars() {
                *word_so_far += stringify!(r#char.clone());
                root_node.add_child(Node::new(Some(*word_so_far.clone())))
            }
        }
        return Ok(root_node);
    }

    #[derive(Debug)]
    pub struct Node<T> {
        data: Option<T>,
        children: Vec<Node<T>>
    }

    impl<T: PartialEq> Node<T> {
        fn new(data: Option<T>) -> Node<T> {
            Node { data, children: vec![] }
        }

        fn add_child(&mut self, child: Node<T>) {
            let data = child.data.unwrap(); // todo fix
            if self.children.iter().any(move |n| {
                match &n.data {
                    Some(n) if *n == data => true,
                    Some(_) => false,
                    None => false
                }
            }) {
                // self.children.push(child);
            }
        }
    }
}
