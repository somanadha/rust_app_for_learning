fn main() {
    let s2 = String::from("Shorter String");
    let s1 = String::from("Longer String");
    let result: &str;
    {
        result = find_longest(&s1, &s2);
    }
    println!("Longest \"{}\"", result);

    println!("{}", retur_a_reference("mytest"));

    let book = Book {
        title: "The Rust Programming Language",
        author: "Steve Klabnik and Carol Nichols",
    };

    let mut combined = String::new();
    let result = concatenate(book.title, book.author, &mut combined);
    println!("Concatenated: {}", result);
}

fn find_longest<'a>(rs1: &'a str, rs2: &'a str) -> &'a str {
    if rs1.len() > rs2.len() {
        rs1
    } else {
        rs2
    }
}

fn retur_a_reference<'a>(s: &'a str) -> &'a str {
    s
}

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

fn concatenate<'a>(s1: &'a str, s2: &'a str, result: &'a mut String) -> &'a str {
    result.clear();
    result.push_str(s1);
    result.push_str(s2);
    result.as_str()
}
