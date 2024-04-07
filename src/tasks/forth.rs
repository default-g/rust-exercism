pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: std::collections::HashMap<String, String>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}
impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input_lowercase = input.to_ascii_lowercase();
        let commands = input_lowercase.split_whitespace();
        let mut iterator = commands.into_iter();
        while let Some(command) = iterator.next() {
            println!("{}", command);

            match command {
                ":" => {
                    let word = iterator.next().ok_or(Error::InvalidWord)?;
                    word.parse::<Value>().err().ok_or(Error::InvalidWord)?;

                    let mut new_definition: Vec<&str> = Vec::new();
                    loop {
                        match iterator.next() {
                            Some(";") => break,
                            Some(symbol) => new_definition.push(symbol),
                            None => return Err(Error::InvalidWord),
                        }
                    }

                    let definition_string: String = new_definition.join(" ");

                    if let Some(_) = self.words.insert(word.to_string(), definition_string) {
                        for definition in self.words.values_mut() {
                            *definition = definition.replace(word, definition);
                        }
                    }
                }

                _ if self.words.contains_key(command) => {
                    self.eval(&self.words[command].clone())?;
                }
                "+" => self.plus()?,
                "*" => self.multiply()?,
                "-" => self.minus()?,
                "/" => self.div()?,
                "drop" => self.drop()?,
                "dup" => self.dup()?,
                "over" => self.over()?,
                "swap" => self.swap()?,
                _ => self
                    .stack
                    .push(command.parse::<Value>().or(Err(Error::UnknownWord))?),
            };
        }
        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn plus(&mut self) -> Result {
        let second_value = self.pop()?;
        let first_value = self.pop()?;

        Ok(self.stack.push(first_value + second_value))
    }

    fn minus(&mut self) -> Result {
        let second_value = self.pop()?;
        let first_value = self.pop()?;

        Ok(self.stack.push(first_value - second_value))
    }

    fn multiply(&mut self) -> Result {
        let first_value = self.pop()?;
        let second_value = self.pop()?;

        Ok(self.stack.push(first_value * second_value))
    }

    fn div(&mut self) -> Result {
        let second_value = self.pop()?;
        let first_value = self.pop()?;

        if second_value == 0 {
            return Err(Error::DivisionByZero);
        }

        Ok(self.stack.push(first_value / second_value))
    }

    fn drop(&mut self) -> Result {
        self.pop()?;

        Ok(())
    }

    fn dup(&mut self) -> Result {
        let value = self.pop()?;

        self.stack.push(value);
        self.stack.push(value);

        Ok(())
    }

    fn over(&mut self) -> Result {
        let second_value = self.pop()?;
        let first_value = self.pop()?;

        self.stack.push(first_value);
        self.stack.push(second_value);
        self.stack.push(first_value);

        Ok(())
    }

    fn swap(&mut self) -> Result {
        let second_value = self.pop()?;
        let first_value = self.pop()?;

        self.stack.push(second_value);
        self.stack.push(first_value);

        Ok(())
    }
}
