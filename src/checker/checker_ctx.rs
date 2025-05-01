// use paparse::Scanner;

// enum LineType {
//     VarDecla,
//     Other,
// }

// struct FuncBlockState {
//     pub brackets_depth: i32,
// }

// struct CheckerCtx {
//     pub prev_line_state: LineType,
//     pub func_block_state: FuncBlockState,
// }

// pub fn is_line_var_decla(line: &str) -> bool {
//     let mut scanner = Scanner::new(line);
//     scanner.skip_whitespace();
//     if !scanner.expect_identifier() {
//         return false;
//     }
//     if !scanner.expect(' ') {
//         return false;
//     }
//     if !scanner.expect_identifier() {
//         return false;
//     }
//     if !scanner.expect_str(" = ") {
//         return false;
//     }
//     return true;
// }

// impl CheckerCtx {
//     #[inline]
//     pub fn in_function_block(&self) -> bool {
//         return self.func_block_state.brackets_depth == 1;
//     }
//     pub fn update(&mut self, line: &str) {
//         for c in line.chars() {
//             if c == '{' {
//                 self.func_block_state.brackets_depth += 1;
//             }
//             if c == '}' {
//                 self.func_block_state.brackets_depth -= 1;
//             }
//         }
//         if self.in_function_block() {
//             if is_line_var_decla(line) {
//                 self.prev_line_state = LineType::VarDecla;
//             } else {
//                 self.prev_line_state = LineType::Other;
//             }
//         }
//     }
// }
