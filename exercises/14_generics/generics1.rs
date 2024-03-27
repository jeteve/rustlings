// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let milk: String = String::from("millkkk");
    let mut shopping_list: Vec<_> = Vec::new();
    shopping_list.push("milk");
    {
        shopping_list.push(&milk);
    }

    for s in shopping_list.iter(){
        println!("Stuff {}" , s);
    }
    println!("End of scope??");
}
