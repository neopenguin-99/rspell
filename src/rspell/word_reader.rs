pub use self::word_reader::get_words_from_file;
pub use self::word_reader::Node;
pub mod word_reader {
    use std::borrow::Borrow;
    use std::io::Read;
    use std::fs::File;
    use itertools::Itertools;
    use unicode_segmentation::UnicodeSegmentation;
    use std::rc::Rc;
    use std::cmp;
    use std::cmp::Ordering;

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
        let mut root_node = Node::new(String::new());
        let mut word_so_far: &str;
        for word in words {
            word_so_far = "";
            for r#char in word.chars() {
                let word_to_add = format!("{}{}", word_so_far, r#char);
                root_node.add_child(Node::new(word_to_add))
            }
        }
        return Ok(root_node);
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
    pub struct Node<T> {
        pub data: T,
        pub children: Vec<Node<T>>
    }

    impl<T: PartialEq + Clone> Node<T> {
        fn new(data: T) -> Node<T> {
            Node { data, children: vec![] }
        }

        fn add_child(&mut self, child: Node<T>) {
            let data = child.data.clone();
            if self.children.iter().all(move |n| {
                match &n.data {
                    n if *n == data => false,
                    _ => true,
                }
            }) {
                self.children.push(child);
            }
        }
    }


    fn get_substring(node: Node<String>, mut word_to_search: String) -> String {
        let max_index = 0;
        for child_node in &node.children {
            traverse(&child_node, word_to_search);
            for (i, r#char) in word_to_search.char_indices() {
                if r#char == child_node.data.get(i..i+1).unwrap().chars().nth(0).unwrap() {
                    word_to_search = String::from(get_substring(child_node.clone(), word_to_search.clone()););
                }
            }

        }
        String::new()
    }

    fn traverse(node: Node<String>, word_to_search: String, depth: u32) {
        // for every child in node, check if word_to_search[depth] == node.data[depth]
        for child_node in node.children { 
            child_node.data.contains('a');
            if child_node.data.get(depth..depth+1).unwrap() == node.data.(depth as u32) {
                traverse(child_node, word_to_search, depth + 1)
            }
        }
    }

}
