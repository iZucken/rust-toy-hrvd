
use diff_trait::Diffable;
#[macro_use]
extern crate diff_derive;

#[derive(Diffable, Default)]
struct ExampleStruct {
    bool_field: bool,
    int_field: i32,
    float_field: f32,
}

fn main() {
    let a = ExampleStruct{bool_field: true, int_field: 1, float_field: 1.0};
    println!("{}", a.diff_to(&ExampleStruct{float_field: 3.14, ..ExampleStruct::default()}));
    println!("{}", a.diff_to(&ExampleStruct{int_field: 2, ..ExampleStruct::default()}));
}
