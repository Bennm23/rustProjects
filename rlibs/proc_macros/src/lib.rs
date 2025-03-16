
use iter_comprehension::Composition;
use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, ItemFn};
use quote::quote;

mod iter_comprehension;

/// Iterable Comprehention
/// 
/// comp: mapping for_if_clause
/// mapping: expression
/// for_if_clause:
///     'for' pattern 'in' sequence ('if' expression)*
/// 
/// pattern: name (, name)*
/// 
/// # Example Python
/// ```
///     [2 * x for x in xs if x < 5 if x > 2]
/// ```
/// # Example Rust
/// ```
///     let xs = vec![2, 4, 6, 8, 10];
///     let comprehension = comp![2 * x for x in xs if x < 5 if x > 2];
/// 
///     let on_set = comp!(x for x in set);
/// 
///     let on_range = comp!(x for x in (0..12).rev());
/// ```
#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let c = parse_macro_input!(input as Composition);
    quote! { #c }.into()
}

struct TimeUnit(syn::ExprAssign);

impl Parse for TimeUnit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

/// Benchmark Macro Attribute
///
/// Attribute for function that will execute the body and print
/// the time elapsed in the given unit.
/// 
/// Default time unit is microseconds
/// 
/// # Example
/// ```
///     #[benchmark]
///     fn do_work() {
///         for _ in 0..5 {
///             thread::sleep(Duration::from_micros(200));
///         }
///     }
///
///     #[benchmark(unit="millis")]
///     fn do_work() {
///         for _ in 0..5 {
///             thread::sleep(Duration::from_micros(200));
///         }
///     }
/// ```
/// 
#[proc_macro_attribute]
pub fn benchmark(attrs: proc_macro::TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_string = fn_name.to_string();
    let fn_body = &input_fn.block;

    let time_condition = if !attrs.is_empty() {
        let args = parse_macro_input!(attrs as TimeUnit);
        let right = args.0.right;
        quote! {
            if #right == "millis" {
                println!("{} Took {} ms", #fn_string, now.elapsed().as_millis());
            } else {
                println!("{} Took {} us", #fn_string, now.elapsed().as_micros());
            }
        }
    } else {
        quote! {
            println!("{} Took {} us", #fn_string, now.elapsed().as_micros());
        }
    };

    let expanded = quote! {
        use std::time::Instant;

        fn #fn_name() {

            let now = Instant::now();

            #fn_body

            #time_condition
        }
    };

    TokenStream::from(expanded)
}