extern crate proc_macro;

use quote::quote;

#[proc_macro_derive(Diffus)]
pub fn derive_diffus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse2(proc_macro2::TokenStream::from(input)).unwrap();

    let ident = &input.ident;

    match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            let edited_ident = syn::parse_str::<syn::Path>(
                &format!("Edited{}", ident)
            ).unwrap();

            let edit_fields = fields.named.iter().map(|field| {
                let ident = &field.ident;
                let ty = &field.ty;

                quote! {
                    #ident: diffus::Edit<'a, #ty>
                }
            });

            let field_diffs_idents = fields.named.iter().map(|field| &field.ident);
            let field_diffs_copy = fields.named.iter().map(|field| {
                let ident = &field.ident;

                quote!{ #ident @ diffus::Edit::Copy }
            });
            let field_diffs = fields.named.iter().map(|field| {
                let ident = &field.ident;

                quote!{ self.#ident.diff(&other.#ident) }
            });


            proc_macro::TokenStream::from(quote! {
                struct #edited_ident<'a> {
                    #(#edit_fields),*
                }

                impl<'a> diffus::Diffable<'a> for #ident {
                    type D = #edited_ident<'a>;
                    pub fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {

                        match ( #(#field_diffs,)* ) {
                            ( #(#field_diffs_copy,)* ) => Edit::Copy,
                            ( #(#field_diffs_idents,)* ) => Edit::Change(
                                Self::D { #(#field_diffs_idents,)* }
                            )
                        }
                    }
                }
            })
        }
        _ => unreachable!()
    }
}
