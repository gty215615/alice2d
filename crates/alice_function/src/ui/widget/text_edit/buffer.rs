use std::ops::Range;






pub(super) trait TextBuffer: AsRef<str> {
    fn is_mutable(&self) -> bool;

    fn as_str(&self) -> &str {
        self.as_ref()
    }

    fn char_range(&self , char_range:Range<usize>) -> &str {
        assert!(char_range.start <= char_range.end);

        let start_byte = self.byte_index_from_char_index(char_range.start);
        let end_byte = self.byte_index_from_char_index(char_range.end);

        &self.as_str()[start_byte..end_byte]
    }

    fn byte_index_from_char_index(&self , char_index:usize) -> usize {
        byte_index_from_char_index(self.as_str(), char_index)
    }

    fn insert_text(&mut self , text:&str , char_index:usize) -> usize;

    fn delete_char_range(&mut self , char_range:Range<usize>);

    fn clear(&mut self) {
        self.delete_char_range(0..self.as_str().len())
    }

    fn replace(&mut self , text:&str) {
        self.clear();
        self.insert_text(text, 0);
    }

    fn take(&mut self) -> String {
        let s = self.as_str().to_owned();
        self.clear();
        s
    }

}


impl TextBuffer for String {
    fn is_mutable(&self) -> bool {
        true
    }

    fn insert_text(&mut self , text:&str , char_index:usize) -> usize {
        let byte_index = self.byte_index_from_char_index(char_index);

        self.insert_str(byte_index, text);

        text.chars().count()
    }

    fn delete_char_range(&mut self , char_range:Range<usize>) {
        assert!(char_range.start <= char_range.end);

        let start_byte = self.byte_index_from_char_index(char_range.start);
        let end_byte = self.byte_index_from_char_index(char_range.end);

        self.drain(start_byte..end_byte);
    }

    fn replace(&mut self , text:&str) {
        *self = text.to_owned()
    }

    fn take(&mut self) -> String {
        std::mem::take(self)
    }
}


impl<'a> TextBuffer for &'a str {
    fn is_mutable(&self) -> bool {
        false
    }

    fn insert_text(&mut self , text:&str , char_index:usize) -> usize {
        0
    }

    fn delete_char_range(&mut self , char_range:Range<usize>) {
        
    }
}

fn byte_index_from_char_index(s:&str,char_index:usize) -> usize {
    for ( ci , (bi, _) ) in s.char_indices().enumerate() {
        if ci == char_index {
            return bi;
        }
    }
    s.len()
}