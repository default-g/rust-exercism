use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i32>,
    reserved_words: HashMap<String, String>,
    basic_operations: HashMap<String, fn(&mut Self) -> Result>,
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
        let mut forth = Forth {
            stack: Vec::new(),
            reserved_words: HashMap::new(),
            basic_operations: HashMap::new(),
        };

        forth.basic_operations.insert("+".to_string(), Forth::plus);
        forth.basic_operations.insert("*".to_string(), Forth::mul);
        forth.basic_operations.insert("-".to_string(), Forth::minus);
        forth.basic_operations.insert("/".to_string(), Forth::div);

        forth.basic_operations.insert("dup".to_string(), Forth::dup);
        forth
            .basic_operations
            .insert("drop".to_string(), Forth::drop);
        forth
            .basic_operations
            .insert("swap".to_string(), Forth::swap);
        forth
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let operations_string = String::from(input).to_lowercase();

        for (key, value) in self.reserved_words.iter() {
            operations_string.replace(&key[..], &value[..]);
        }

        let operations = operations_string.split(" ");

        for operation in operations.into_iter() {
            if self.basic_operations.contains_key(operation) {
                let result = self.basic_operations.get(operation).unwrap()(self);
                if result.is_err() {
                    return result;
                }

                continue;
            }

            let result = operation.parse::<i32>();

            match result {
                Ok(value) => self.stack.push(value),
                Err(_) => return Err(Error::UnknownWord),
            }
        }

        Ok(())
    }

    fn plus(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let first_value = self.stack.pop().unwrap();
        let second_value = self.stack.pop().unwrap();

        let new_value = first_value + second_value;

        self.stack.push(new_value);

        Ok(())
    }

    fn minus(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let first_value = self.stack.pop().unwrap();
        let second_value = self.stack.pop().unwrap();

        let new_value = first_value - second_value;

        self.stack.push(new_value);

        Ok(())
    }

    fn mul(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let first_value = self.stack.pop().unwrap();
        let second_value = self.stack.pop().unwrap();

        let new_value = first_value * second_value;

        self.stack.push(new_value);

        Ok(())
    }

    fn div(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let first_value = self.stack.pop().unwrap();
        let second_value = self.stack.pop().unwrap();

        if second_value == 0 {
            return Err(Error::DivisionByZero);
        }

        let new_value = first_value / second_value;

        self.stack.push(new_value);

        Ok(())
    }

    fn dup(&mut self) -> Result {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }

        let last_value = self.stack[self.stack.len() - 1];
        self.stack.push(last_value);

        Ok(())
    }

    fn drop(&mut self) -> Result {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }

        self.stack.pop();

        Ok(())
    }

    fn swap(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }

        let first_value = self.stack.pop().unwrap();
        let second_value = self.stack.pop().unwrap();

        self.stack.push(first_value);
        self.stack.push(second_value);

        Ok(())
    }

    fn over(&mut self) -> Result {
        todo!()
    }
}
