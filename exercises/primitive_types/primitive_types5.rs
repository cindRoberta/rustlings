// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

// ✓ I AM NOT DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    //let /* your pattern here */ = cat;
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

// $ rustlings hint primitive_types5
/*
Take a look at the Data Types -> The Tuple Type section of the book:
https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
Particularly the part about destructuring (second to last example in the section).
You'll need to make a pattern to bind `name` and `age` to the appropriate parts
of the tuple. You can do it!!
*/