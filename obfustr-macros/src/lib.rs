use quote::quote;

use proc_macro2::{Span, TokenStream, TokenTree};

#[proc_macro]
pub fn obfuscate_raw(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	match obfuscate(tokens.into()) {
		Ok(x) => x.into(),
		Err(e) => e.to_compile_error().into(),
	}
}

fn obfuscate(tokens: TokenStream) -> Result<TokenStream, syn::Error> {
	let mut tokens = tokens.into_iter();
	let lit = expect_literal(&mut tokens)?;
	expect_empty(&mut tokens)?;
	match &lit {
		syn::Lit::Str(lit) => Ok(obfuscate_str(lit)),
		syn::Lit::ByteStr(lit) => Ok(obfuscate_byte_str(lit)),
		syn::Lit::CStr(lit) => Ok(obfuscate_cstr(lit)),
		lit => Err(syn::Error::new(lit.span(), "expected a string, byte string or C string literal")),
	}
}

fn obfuscate_str(lit: &syn::LitStr) -> TokenStream {
	let value = lit.value();
	let mut obfuscated = Vec::with_capacity(value.len());
	for byte in value.as_bytes() {
		let pad: u8 = rand::random();
		obfuscated.push(u16::from_le_bytes([byte ^ pad, pad]));
	}

	quote! {
		{
			const DATA: &[u16] = &[#(#obfuscated,)*];
			(::core::marker::PhantomData::<str>, DATA)
		}
	}
}

fn obfuscate_byte_str(lit: &syn::LitByteStr) -> TokenStream {
	let value = lit.value();
	let mut obfuscated = Vec::with_capacity(value.len());
	for byte in value {
		let pad: u8 = rand::random();
		obfuscated.push(u16::from_le_bytes([byte ^ pad, pad]));
	}

	quote! {
		{
			const DATA: &[u16] = &[#(#obfuscated,)*];
			(::core::marker::PhantomData::<[u8]>, DATA)
		}
	}
}

fn obfuscate_cstr(lit: &syn::LitCStr) -> TokenStream {
	let value = lit.value();
	let mut obfuscated = Vec::with_capacity(value.as_bytes_with_nul().len());
	for byte in value.as_bytes_with_nul() {
		let pad: u8 = rand::random();
		obfuscated.push(u16::from_le_bytes([byte ^ pad, pad]));
	}

	quote! {
		{
			const DATA: &[u16] = &[#(#obfuscated,)*];
			(::core::marker::PhantomData::<::core::ffi::CStr>, DATA)
		}
	}
}

fn expect_literal(tokens: &mut proc_macro2::token_stream::IntoIter) -> Result<syn::Lit, syn::Error> {
	let tree = tokens.next()
		.ok_or_else(|| syn::Error::new(Span::call_site(), "unexpected end of arguments, expected a literal"))?;
	match tree {
		TokenTree::Literal(lit) => Ok(syn::Lit::new(lit)),
		TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::None => {
			let mut inner = group.stream().into_iter();
			let literal = expect_literal(&mut inner)?;
			expect_empty(&mut inner)?;
			Ok(literal)
		}
		tree => {
			Err(syn::Error::new(tree.span(), "expected a literal"))
		}
	}
}

fn expect_empty(tokens: &mut proc_macro2::token_stream::IntoIter) -> Result<(), syn::Error> {
	if let Some(tree) = tokens.next() {
		Err(syn::Error::new(tree.span(), "unexpected tokens"))
	} else {
		Ok(())
	}
}
