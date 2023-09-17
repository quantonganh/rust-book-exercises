fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[5];
    // let does_not_exist = v.get(5);
    // println!("{:?}", does_not_exist);

    let first = &v[0];

    // v.push(6);
    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 10;
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(4.5),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for r in &row {
        println!("{:?}", r);
    }
}
