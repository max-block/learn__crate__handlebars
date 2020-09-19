#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use serde::Serialize;
use std::fs;

use handlebars::Handlebars;

#[derive(Serialize)]
struct Item {
    name: String,
    value: i64,
}

fn main() {
    handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));

    let mut reg = Handlebars::new();
    reg.register_helper("hex", Box::new(hex));

    let template = fs::read_to_string("data/t1.hbs").unwrap();

    let items = vec![
        Item {
            name: "n1".to_string(),
            value: 1,
        },
        Item {
            name: "n2".to_string(),
            value: 2,
        },
    ];
    let data = json!({"title": "My Data", "items": items});
    let result = reg.render_template(template.as_str(), &data).unwrap();
    println!("{}", result);
}
