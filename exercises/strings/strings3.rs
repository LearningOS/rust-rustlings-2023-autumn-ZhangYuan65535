// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
/*    let bytes = input.as_bytes();
    let size = bytes.len();
    let mut i = 0;
    let mut j = size - 1;
    while bytes[i].is_ascii_whitespace() {
        i = i + 1;
    }
    while bytes[j].is_ascii_whitespace() {
        j = j - 1;
    }
    let s2: String = String::from(&input[i..j+1]);
    s2
*/
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let s = String::new();
    s + input + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
/*    let size = input.len();
    let mut i = 0;
    let mut begin = 0;
    let mut s = String::new();

    while i < size-3 {
        if &input[i..i+4] == "cars"{
            s.push_str(&input[begin..i]);
            s.push_str("balloons");
            i += 4;
            begin = i;
            continue;
        }
        i += 1;
    }
    s.push_str(&input[begin..size]);
    s
*/
    input.to_string().replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
