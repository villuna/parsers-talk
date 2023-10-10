trait Parser<'a, T> {
    fn parse(&mut self, _input: &'a str) -> Option<(T, &'a str)>;
}

impl<'a, T, P: FnMut(&'a str) -> Option<(T, &'a str)>> Parser<'a, T> for P {
    fn parse(&mut self, input: &'a str) -> Option<(T, &'a str)> {
        self(input)
    }
}

fn char(input: &str) -> Option<(char, &str)> {
    if input.len() == 0 {
        None
    } else {
        Some(input.chars.first(), input
    }
}
