use crate::common::time::now;

#[derive(Clone)]
pub struct Operation {
    name: String,
    value_1: u32,
    value_2: u32,
    result: u32,
    create_at: u64,
}

impl Operation {
    pub fn new(name: String, value_1: u32, value_2: u32) -> Result<Self, &'static str> {
        if Operation::check_operation(&name) {
            Ok(Self {
                name: name.to_lowercase(),
                value_1,
                value_2,
                result: Operation::calculate(&name, value_1, value_2),
                create_at: now(),
            })
        } else {
            Err("Operation not found")
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value_1(&self) -> u32 {
        self.value_1
    }

    pub fn value_2(&self) -> u32 {
        self.value_2
    }

    pub fn result(&self) -> u32 {
        self.result
    }

    pub fn create_at(&self) -> u64 {
        self.create_at
    }

    fn check_operation(name: &String) -> bool {
        Operation::is_sum(name)
            || Operation::is_subb(name)
            || Operation::is_multiply(name)
            || Operation::is_divide(name)
    }

    fn is_sum(name: &String) -> bool {
        name.eq_ignore_ascii_case("sum")
    }

    fn is_subb(name: &String) -> bool {
        name.eq_ignore_ascii_case("subb")
    }

    fn is_multiply(name: &String) -> bool {
        name.eq_ignore_ascii_case("multiply")
    }

    fn is_divide(name: &String) -> bool {
        name.eq_ignore_ascii_case("divide")
    }

    fn calculate(name: &String, value_1: u32, value_2: u32) -> u32 {
        match name.as_str() {
            "sum" => value_1 + value_2,
            "subb" => value_1 - value_2,
            "multiply" => value_1 * value_2,
            "divide" => {
                let value = if value_2 == 0 { 0 } else { value_1 / value_2 };

                value
            }
            _ => 0,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value_1: Default::default(),
            value_2: Default::default(),
            result: Default::default(),
            create_at: Default::default(),
        }
    }
}

pub struct OperationBuilder(Operation);

impl OperationBuilder {
    pub fn new() -> Self {
        Self(Operation::default())
    }

    pub fn with_operation(mut self, operation: String) -> Self {
        self.0.name = operation;
        self
    }

    pub fn with_value_1(mut self, value: u32) -> Self {
        self.0.value_1 = value;
        self
    }

    pub fn with_value_2(mut self, value: u32) -> Self {
        self.0.value_2 = value;
        self
    }

    pub fn with_result(mut self, value: u32) -> Self {
        self.0.result = value;
        self
    }

    pub fn with_epoch(mut self, value: u64) -> Self {
        self.0.create_at = value;
        self
    }

    pub fn build(self) -> Operation {
        Operation {
            name: self.0.name,
            value_1: self.0.value_1,
            value_2: self.0.value_2,
            result: self.0.result,
            create_at: self.0.create_at,
        }
    }
}
