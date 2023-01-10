use darling::FromDeriveInput;
use syn::{punctuated::Punctuated, Token, TypeParam};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(arbitrary))]
pub struct ContainerAttributes {
    #[darling(default)]
    pub bound: Option<Punctuated<TypeParam, Token![,]>>,
}
