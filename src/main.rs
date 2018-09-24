use std::collections::HashMap;

extern crate fluent;
use fluent::bundle::FluentBundle;

struct LocalesRegistry<'a> {
    locales: HashMap<String, FluentBundle<'a>>,
}

impl<'a> LocalesRegistry<'a> {
    pub fn load_from_folder(folder_path: &str) -> LocalesRegistry {
        let mut locales = HashMap::new();

        let mut bundle = FluentBundle::new(&[folder_path]);
        bundle.add_messages("hello-world = Hello, world!").unwrap();

        locales.insert(String::from("en-US"), bundle);
        LocalesRegistry { locales: locales }
    }

    pub fn names(&self) -> Vec<&str> {
        self.locales.keys().map(|k| k.as_ref()).collect::<Vec<&str>>()
    }

    pub fn translate(&self, locale: &str, key: &str) -> String {
      let bundle = self.locales.get(locale).unwrap();
      let (value, _errors) = bundle.format(key, None).unwrap();
      value
    }
}

fn main() {
    let registry = LocalesRegistry::load_from_folder("whatever");
    println!("Have locales: {:?}", registry.names());
    let translation = registry.translate("en-US", "hello-world");
    println!("Translation: {}", translation);
}
