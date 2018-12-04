extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::*;
use quote::quote;

#[proc_macro_derive(ReParse, attributes(re_parse))]
pub fn re_parse_macro_derive(item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as DeriveInput);

    let regex = item.attrs.iter()
        .flat_map(|x| x.parse_meta() )
        .filter_map(|x| match x { Meta::List(y) => Some(y), _ => None })
        .filter(|x| x.ident == "re_parse" )
        .flat_map(|x| x.nested.into_iter() )
        .filter_map(|x| match x { NestedMeta::Meta(y) => Some(y), _ => None })
        .filter_map(|x| match x { Meta::NameValue(y) => Some(y), _ => None })
        .find(|x| x.ident == "regex" )
        .and_then(|x| match x.lit { Lit::Str(y) => Some(y.value()), _ => None })
        .unwrap();

    let item_ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let impl_inner = quote! { 
        impl #impl_generics std::str::FromStr for #item_ident #ty_generics #where_clause {
            type Err = _re_parse::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                _re_parse::lazy_static! {
                    static ref RE: _re_parse::Regex = Regex::new(#regex).unwrap();
                }

                Ok(_re_parse::with_pattern_from_str(&*RE, s)?)
            }
        }
    };

    // panic!("{}", &impl_inner);

    let dummy_const = Ident::new(
        &format!("_IMPL_FromStr_FOR_{}", item.ident.to_string()),
        Span::call_site()
    );

    let out = quote!{
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            #[allow(unknown_lints)]
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            extern crate re_parse as _re_parse;
            #impl_inner
        };
    };

    out.into()
}
