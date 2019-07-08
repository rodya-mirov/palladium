use super::*;

use resources::*;

pub struct DialogueBuilder {
    main_text: String,
    selected_index: usize,
    options: Vec<DialogueOptionBuilder>,
}

struct DialogueOptionBuilder {
    selected_text: String,
    unselected_text: String,
    callbacks: Vec<DialogueCallback>,
}

impl DialogueBuilder {
    pub fn new(main_text: &str) -> Self {
        DialogueBuilder {
            main_text: wrapped(main_text, LINE_WIDTH),
            selected_index: 0,
            options: Vec::with_capacity(1),
        }
    }

    pub fn with_option(mut self, text: &str, callbacks: Vec<DialogueCallback>) -> Self {
        self.options.push(DialogueOptionBuilder {
            selected_text: text.to_string(),
            unselected_text: text.to_string(),
            callbacks,
        });
        self
    }

    pub fn build(self) -> DialogueState {
        if self.selected_index >= self.options.len() {
            panic!(
                "Cannot initialize this dialogue builder, because the selected index {} is out of range: {}",
                self.selected_index,
                self.options.len()
            );
        }

        DialogueState {
            main_text: self.main_text,
            selected_index: self.selected_index,
            options: self.options.into_iter().map(|opt| opt.build()).collect(),
        }
    }
}

impl DialogueOptionBuilder {
    pub fn build(self) -> DialogueOptionState {
        DialogueOptionState {
            selected_text: self.selected_text,
            unselected_text: self.unselected_text,
            callbacks: self.callbacks,
        }
    }
}

pub fn end_dialogue(focus: &mut KeyboardFocus, dialogue_state: &mut DialogueStateResource) {
    if *focus != KeyboardFocus::Dialogue {
        panic!("Cannot end dialogue when it is not running!");
    }

    *focus = KeyboardFocus::GameMap;
    *dialogue_state = DialogueStateResource {
        is_initialized: InitializationState::NotStarted,
        state: None,
    };
}

pub fn launch_dialogue(builder: DialogueBuilder, focus: &mut KeyboardFocus, dialogue_state: &mut DialogueStateResource) {
    if *focus != KeyboardFocus::GameMap {
        panic!("Can only start dialogue from game map!");
    }

    *focus = KeyboardFocus::Dialogue;
    *dialogue_state = DialogueStateResource {
        is_initialized: InitializationState::NotStarted,
        state: Some(builder.build()),
    };
}

const LINE_WIDTH: usize = 60;

// NB: does not deal well with unicode (but neither does our font so :shrug:)
// Note: this method is very error-prone (yay, string handling) so if you fix a bug
// please add a failing unit test first, then make it pass
fn wrapped(s: &str, line_width: usize) -> String {
    let mut out = Vec::new();
    let mut line = Vec::with_capacity(line_width);

    fn flush_line(out: &mut Vec<char>, line: &mut Vec<char>) {
        while !line.is_empty() && line[line.len() - 1] == ' ' {
            line.pop();
        }

        out.append(line);
    }

    let mut last_space = 0;

    let chars = s.chars().collect::<Vec<char>>();
    let mut is_leading_space = true;
    for i in 0..chars.len() {
        let c = chars[i];

        if c == '\n' {
            flush_line(&mut out, &mut line);
            out.push('\n');
            is_leading_space = true;
            last_space = 0;
            continue;
        }

        if c == ' ' {
            if line.is_empty() && !is_leading_space {
                continue;
            }

            last_space = line.len();
        }

        line.push(c);
        if i < chars.len() - 1 && line.len() >= line_width {
            println!("Killer character was {}", c);
            let (mut new_line, needs_dash) = {
                if chars[i + 1] == ' ' {
                    (line.split_off(line_width), false)
                } else if last_space > 0 {
                    (line.split_off(last_space + 1), false)
                } else {
                    (line.split_off(line_width - 1), true)
                }
            };
            flush_line(&mut out, &mut line);
            if needs_dash {
                out.push('-');
            }
            out.push('\n');
            // these are now "wrapping" spaces; so any spaces before the next
            // not-whitespace character are skippable
            is_leading_space = false;
            line.append(&mut new_line);
            last_space = 0;
        }
    }

    out.append(&mut line);

    out.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(to_wrap: &str, expected: &str, wrap_width: usize) {
        let actual = wrapped(to_wrap, wrap_width);
        let expected = expected.to_owned();

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_wrap_test() {
        do_test("01234567890", "01234567890", 20);
        do_test("01234567890", "01234567890", 120);
        do_test("01234567890", "01234567890", 240);
    }

    #[test]
    fn keep_newlines() {
        do_test("0123\n4567890", "0123\n4567890", 20);
        do_test("0123\n4567890", "0123\n4567890", 120);
        do_test("0123\n4567890", "0123\n4567890", 240);
    }

    #[test]
    fn wrap_no_spaces() {
        do_test("12345", "12-\n345", 3);
        do_test("1234567890\n1234", "1234-\n5678-\n90\n1234", 5);
    }

    #[test]
    fn wrap_exact_width() {
        do_test("1234567890\n12345", "1234-\n5678-\n90\n12345", 5);
        do_test("12345", "12345", 5);
    }

    #[test]
    fn wrap_use_spaces() {
        do_test("abc def ghi", "abc def\nghi", 9);
    }

    #[test]
    fn wrap_check_next_space() {
        do_test("123 567 890", "123 567\n890", 7);
    }

    #[test]
    fn keep_leading_spaces() {
        do_test(" 123456789", " 12345-\n6789", 7);
    }
}
