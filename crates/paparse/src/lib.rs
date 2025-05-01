pub struct Scanner {
    pos: usize,
    chars: Vec<char>,
    save: usize,
}

type ScannerFuncCmp = fn(char) -> bool;

impl Scanner {
    pub fn new(content: &str) -> Scanner {
        Scanner {
            pos: 0,
            chars: content.chars().collect(),
            save: 0,
        }
    }

    #[inline]
    pub fn cursor(&self) -> usize {
        return self.pos;
    }

    #[inline]
    pub fn unsafe_peek(&self) -> char {
        return self.chars[self.pos];
    }

    #[inline]
    pub fn unsafe_pop(&mut self) -> char {
        let c = self.chars[self.pos];
        self.advance();
        return c;
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        return self.pos >= self.chars.len();
    }

    #[inline]
    pub fn peek(&self) -> Option<char> {
        if self.is_done() {
            return None;
        }
        return Some(self.unsafe_peek());
    }

    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        if self.is_done() {
            return None;
        }
        let c = Some(self.unsafe_peek());
        self.advance();
        return c;
    }

    #[inline]
    pub fn peek_n(&self, n: usize) -> Option<char> {
        if self.pos + n <= self.chars.len() {
            return Some(self.chars[self.pos + n]);
        }
        return None;
    }

    #[inline]
    pub fn save(&mut self) {
        self.save = self.pos;
    }

    #[inline]
    pub fn erase_save(&mut self) {
        self.save = 0;
    }

    #[inline]
    pub fn restore(&mut self) {
        self.erase_save();
        self.pos = self.save;
    }

    // Navigation
    #[inline]
    pub fn advance(&mut self) {
        self.pos += 1;
    }

    #[inline]
    pub fn backs_up(&mut self) {
        self.pos -= 1;
    }

    #[inline]
    pub fn skip_while(&mut self, cmp: ScannerFuncCmp) {
        while !self.is_done() && cmp(self.unsafe_peek()) {
            self.advance();
        }
    }

    #[inline]
    pub fn rewind(&mut self, n: usize) {
        if self.pos as i32 - n as i32 >= 0 {
            self.pos -= n;
        }
    }

    #[inline]
    pub fn skip_whitespace(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }

    // Take
    #[inline]
    pub fn take_while(&mut self, cmp: ScannerFuncCmp) -> Option<String> {
        let mut str = String::new();

        while !self.is_done() && cmp(self.unsafe_peek()) {
            str.push(self.unsafe_pop());
        }

        if str.len() > 0 {
            return Some(str);
        }
        return None;
    }

    pub fn take_between(&mut self, open: char, close: char) -> Option<String> {
        if !self.expect(open) {
            return None;
        }

        let mut content = String::new();
        let mut depth = 1;

        while let Some(c) = self.peek() {
            self.advance();
            if c == open {
                depth += 1;
                content.push(c);
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return Some(content);
                }
                content.push(c);
            } else {
                content.push(c);
            }
        }
        None
    }

    pub fn take_string(&mut self) -> Option<String> {
        if !self.expect('"') {
            return None;
        }

        let mut result = String::new();
        while let Some(c) = self.pop() {
            match c {
                '\\' => {
                    if let Some(next) = self.pop() {
                        match next {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            'n' => result.push('\n'),
                            't' => result.push('\t'),
                            'r' => result.push('\r'),
                            other => {
                                result.push('\\');
                                result.push(other);
                            }
                        }
                    } else {
                        break;
                    }
                }
                '"' => return Some(result),
                _ => result.push(c),
            }
        }
        None
    }

    // Validation
    #[inline]
    pub fn expect(&mut self, c: char) -> bool {
        if self.is_done() {
            return false;
        }
        if self.unsafe_peek() == c {
            self.advance();
            return true;
        }
        return false;
    }

    pub fn take_identifier(&mut self) -> Option<String> {
        self.save();
        let first = self.peek()?;
        if first.is_alphabetic() || first == '_' {
            let mut ident = String::new();
            ident.push(self.unsafe_pop());
            let rest = self
                .take_while(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or_default();
            ident.push_str(&rest);
            self.erase_save();
            return Some(ident);
        }
        self.restore();
        None
    }

    #[inline]
    pub fn expect_identifier(&mut self) -> bool {
        self.take_identifier().is_some()
    }

    #[inline]
    pub fn expect_str(&mut self, str: &str) -> bool {
        self.save();
        for c in str.chars() {
            if !self.expect(c) {
                self.restore();
                return false;
            }
        }
        self.erase_save();
        return true;
    }

    // parsing
    #[inline]
    pub fn take_number(&mut self) -> Option<f64> {
        let result = self.take_while(|c| c.is_numeric() || c == '.');
        if let Some(content) = result {
            return content.parse().ok();
        }
        return None;
    }
}
