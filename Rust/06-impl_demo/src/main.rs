#[derive(Debug)]
struct FsItem {
    name: String,
    size: usize,
}

impl FsItem {
    fn new() -> Self {
        Self {
            name: String::from(""),
            size: 0usize,
        }
    }

    fn create(name: String, size: usize) -> FsItem {
        FsItem { name, size }
    }

    fn name(&self) -> String {
        String::from(self.name.as_str())
    }

    fn size(&self) -> usize {
        self.size
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut fsitem1 = FsItem::new();
    let fsitem2 = FsItem::create(String::from("/etc/fsitem2"), 16);

    println!("{fsitem1:#?}");
    println!("{fsitem2:#?}");

    println!("fsitem1 = {{ {}, {} }}", fsitem1.name(), fsitem1.size());

    fsitem1.rename(String::from("/etc/fsitem1"));

    println!("fsitem1 = {{ {}, {} }}", fsitem1.name(), fsitem1.size());
}
