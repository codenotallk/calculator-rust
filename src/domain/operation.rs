pub struct Operation {
    name: String,
    value_1: u32,
    value_2: u32,
    result: u32,
}

impl Operation {
    pub fn new(name: String, value_1: u32, value_2: u32) -> Result<Self, &'static str> {
        if Operation::check_operation(&name) {
            Ok(Self {
                name: name.to_lowercase(),
                value_1,
                value_2,
                result: Operation::calculate(&name, value_1, value_2),
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
