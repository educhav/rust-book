fn main() {
    println!("Hello, world!");
}

// What vec looks like
// Declarative Macros - pattern matching
// Only 1 valid pattern to match here, any other pattern will cause a compile-time error
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural Macros
// 3 kinds: Custom Derive, Attribute-like, Function-like
//

// Attribute-like macros
// #[route(GET, "/")]
// fn index() {}
// Defined as such
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// Function-like macros
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {

