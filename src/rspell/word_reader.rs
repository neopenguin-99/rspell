pub use self::word_reader::get_words_from_file;
pub mod word_reader {
    use std::io::Read;
    use std::fs::File;
    use unicode_segmentation::UnicodeSegmentation;

    pub fn get_words_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut f = File::open(file_path)?;
        let buf = &mut Default::default();
        let _ = f.read_to_string(buf)?;
        let words_ref = buf.unicode_words().collect::<Vec<&str>>();

        Ok(words_ref.iter().map(|word_ref| word_ref.to_string()).collect())
    }

    pub fn get_words_from_file_as_tree() {
        // we could implement this as a tree, with the depth of an element pertaining how many
        // characters a word has
        todo!("implement this");
    }

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        children: Vec<Node<T>>
    }

    impl<T> Node<T> {
        fn new(data: T) -> Node<T> {
            Node { data, children: vec![] }
        }

        fn add_child(&mut self, child: Node<T>) {
            self.children.push(child);
        }
    }
}
