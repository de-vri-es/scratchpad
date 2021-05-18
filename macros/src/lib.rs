#[proc_macro]
pub fn foo(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	foo_impl(tokens.into()).into()
}

fn foo_impl(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	quote::quote! {
		pub mod foo {
			pub fn foo() -> #tokens {
				todo!();
			}
		}
	}
}
