// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        fn uppercase(inp: &String) -> String {
            inp.to_uppercase().to_string()
        }

        fn trim(inp: &String) -> String {
            let mut start_word = 0;
            let mut start = false;
            let mut previous = false;
            let mut end_word = inp.chars().count()-1;
            for (i, c) in inp.chars().enumerate() {
                if c != ' ' && !start {
                    start = true;
                    start_word = i;
                }
                if c == ' ' && previous && start {
                    end_word = i - 1;
                    return (&inp[start_word..end_word]).to_string();
                } else if c == ' ' && !previous && start {
                    previous = true;
                } else {
                    previous = false;
                }
            }
            (&inp[start_word..end_word]).to_string()
        }

        fn append(strinp: &String, inp : &usize) -> String {
            let mut temp = strinp.to_string();
            for i in 0..*inp as i32 {
                temp.push_str("bar");
            }
            temp.to_string()
        }

        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    output.push(uppercase(string));
                }
                Command::Trim => {
                    output.push(trim(string));
                }
                Command::Append(word) => {
                    output.push(append(string, word));
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
