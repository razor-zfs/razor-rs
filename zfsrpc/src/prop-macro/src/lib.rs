use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::Ident;

/// Replacing class TokenStream with pascal case TokenStream
/// example: foo_bars -> FooBars
#[proc_macro]
pub fn classcase(ts: TokenStream) -> TokenStream {
    ts.to_string().to_pascal_case().parse().unwrap()
}

/// Replacing the last segment of path TokenStream with pascal case TokenStream
/// example: foo::bar -> foo::Bar
#[proc_macro]
pub fn classcase_path_end(ts: TokenStream) -> TokenStream {
    let mut path = syn::parse_macro_input::parse::<syn::Path>(ts).unwrap();
    let mut last_seg = path.segments.last_mut().unwrap();

    last_seg.ident = Ident::new(
        &last_seg.ident.to_string().to_pascal_case(),
        Span::call_site(),
    );
    path.to_token_stream().into()
}

/// Replacing function TokenStream with snake case only for func name
/// example:
/// This func:
/// snakecase!(
/// fn FooBar(i: u32) {
///     some_func(i);
/// })
///
/// Will vecome this:
/// fn foo_bar(i: u32) {
///     some_func(i);
/// }
#[proc_macro]
pub fn snakecase_fn(ts: TokenStream) -> TokenStream {
    let mut func = syn::parse_macro_input::parse::<syn::ItemFn>(ts).unwrap();

    func.sig.ident = Ident::new(
        &func.sig.ident.to_string().to_snake_case(),
        Span::call_site(),
    );
    func.to_token_stream().into()
}
