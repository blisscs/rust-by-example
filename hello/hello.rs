/*
 * Usage
 *
 * > rustc hello.rs
 * > ./hello
 *
 */
fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced, Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying in the format character
    // after a `:`.
    println!("Base 10: {}", 793); // 793
    println!("Base 2 (binary): {:b}", 793); // 110001100
    println!("Base 8 (octal): {:o}", 793); // 1431
    println!("Base 16 (hexadecimal): {:x}", 793); // 319

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond"); FIXME ^ Add the missing argument: "James"
    println!("My name is {0}, {1}, {0}", "James", "Bond");

    /* Only types that implement fmt::Display can be formatted with `{}`. User-
     * defined types do not implement fmt::Display by default.
     *
     */

    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    /* Exercise
     * Add a println! macro call that prints: Pi is roughly 3.142
     * by controlling the number of decimal places shown.
     * For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt
     * documentation for setting the number of decimals to display)
     */
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
