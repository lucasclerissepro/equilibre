use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CLI {
    pub name: String
}

impl Display for CLI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        Ok(())
    }
}

fn main() {
    let cli = CLI {
        name: "Lucas".to_string()
    };

    println!("Hello, {}", cli);
}
