// Globally scoped constant
// const is equivalent to:
// TS: const globalVariable:String = "constant";
// Dart: const final String globalVariable = "constant";
// const GLOBAL_VARIABLE: &str = "global constant";

fn main() {
    // Equivalent to console.log(), printf(), print()
    println!("Hi, Mom!");

    // Equivalent to const (js), final/static (dart)
    // let some_variable = "immutable";

    // Equivalent to let(ts because you can't change the type)
    // let mut another_variable = "mutable";

    // Locally scoped constant
    // const LOCAL_VARIABLE: &str = "local constant";
}
