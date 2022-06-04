struct StringHello {
    name: String,
}
impl StringHello {
    fn new(name: String) -> StringHello {
        StringHello { name }
    }
    fn add_name(&self, name: String) -> StringHello {
        StringHello {
            name: format!("{} {}", self.name, name),
        }
    }
}
fn main() {
    let hello = StringHello::new("world".to_string());
    println!("Hello, {}!", hello.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hello() {
        let hello = StringHello::new("world".to_string());
        assert_eq!(hello.name, "world");
    }
}
