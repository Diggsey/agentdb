use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, AttributeArgs, Item, ItemImpl, Path,
    PathArguments,
};

#[derive(Debug, FromMeta)]
struct AgentArgs {
    name: String,
}

fn agent_impl(parsed_attrs: AttributeArgs, parsed_item: Item) -> Result<TokenStream2, Error> {
    let AgentArgs { name } = AgentArgs::from_list(&parsed_attrs)?;

    let type_ident = match &parsed_item {
        Item::Enum(x) => &x.ident,
        Item::Struct(x) => &x.ident,
        Item::Type(x) => &x.ident,
        Item::Union(x) => &x.ident,
        _ => {
            return Err(Error::custom(
                "`#[agent]` can only be applied to items which define a type.",
            ))
        }
    };

    Ok(quote! {
        #parsed_item

        ::agentdb_system::declare_agent!(
            #name => #type_ident
        );
    })
}

#[proc_macro_attribute]
pub fn agent(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_attrs = parse_macro_input!(attrs as AttributeArgs);
    let parsed_item = parse_macro_input!(item as Item);

    TokenStream::from(match agent_impl(parsed_attrs, parsed_item) {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    })
}

#[derive(Debug, FromMeta)]
struct MessageArgs {
    name: String,
}

fn message_impl(parsed_attrs: AttributeArgs, parsed_item: Item) -> Result<TokenStream2, Error> {
    let MessageArgs { name } = MessageArgs::from_list(&parsed_attrs)?;

    let type_ident = match &parsed_item {
        Item::Enum(x) => &x.ident,
        Item::Struct(x) => &x.ident,
        Item::Type(x) => &x.ident,
        Item::Union(x) => &x.ident,
        _ => {
            return Err(Error::custom(
                "`#[message]` can only be applied to items which define a type.",
            ))
        }
    };

    Ok(quote! {
        #parsed_item

        ::agentdb_system::declare_message!(
            #name => #type_ident
        );
    })
}

#[proc_macro_attribute]
pub fn message(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_attrs = parse_macro_input!(attrs as AttributeArgs);
    let parsed_item = parse_macro_input!(item as Item);

    TokenStream::from(match message_impl(parsed_attrs, parsed_item) {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    })
}

fn path_name_is(path: &Path, name: &str) -> bool {
    if let Some(segment) = path.segments.last() {
        segment.ident == name
    } else {
        false
    }
}

#[derive(Debug, FromMeta)]
struct ConstructorArgs {}

fn constructor_impl(parsed_attrs: AttributeArgs, parsed_item: Item) -> Result<TokenStream2, Error> {
    let ConstructorArgs {} = ConstructorArgs::from_list(&parsed_attrs)?;

    let type_ =
        match &parsed_item {
            Item::Impl(ItemImpl {
                trait_: Some((None, path, _)),
                self_ty: x,
                ..
            }) if path_name_is(path, "Construct") => x,
            _ => return Err(Error::custom(
                "`#[constructor]` can only be applied to implementations of the `Construct` trait.",
            )),
        };

    Ok(quote! {
        #[::agentdb_system::hidden::async_trait]
        #parsed_item

        ::agentdb_system::declare_constructor!(
            #type_
        );
    })
}

#[proc_macro_attribute]
pub fn constructor(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_attrs = parse_macro_input!(attrs as AttributeArgs);
    let parsed_item = parse_macro_input!(item as Item);

    TokenStream::from(match constructor_impl(parsed_attrs, parsed_item) {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    })
}

#[derive(Debug, FromMeta)]
struct HandlerArgs {}

fn handler_impl(parsed_attrs: AttributeArgs, parsed_item: Item) -> Result<TokenStream2, Error> {
    let HandlerArgs {} = HandlerArgs::from_list(&parsed_attrs)?;

    let (type_, path, generic) = match &parsed_item {
        Item::Impl(ItemImpl {
            trait_: Some((None, path, _)),
            self_ty: x,
            ..
        }) if path_name_is(path, "Handle") => (x, path, true),
        Item::Impl(ItemImpl {
            trait_: Some((None, path, _)),
            self_ty: x,
            ..
        }) if path_name_is(path, "HandleDyn") => (x, path, false),
        _ => {
            return Err(Error::custom(
                "`#[handler]` can only be applied to implementations of the `Handle<M>` trait.",
            ))
        }
    };

    if generic {
        let last_segment = path.segments.last().unwrap();
        let message_ty =
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    &args[0]
                }
                _ => return Err(Error::custom(
                    "`#[handler]` can only be applied to implementations of the `Handle<M>` trait.",
                )),
            };

        Ok(quote! {
            #[::agentdb_system::hidden::async_trait]
            #parsed_item

            ::agentdb_system::declare_handler!(
                #type_ [#message_ty]
            );
        })
    } else {
        Ok(quote! {
            #[::agentdb_system::hidden::async_trait]
            #parsed_item

            ::agentdb_system::declare_dyn_handler!(
                #type_
            );
        })
    }
}

#[proc_macro_attribute]
pub fn handler(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_attrs = parse_macro_input!(attrs as AttributeArgs);
    let parsed_item = parse_macro_input!(item as Item);

    TokenStream::from(match handler_impl(parsed_attrs, parsed_item) {
        Ok(v) => v,
        Err(e) => e.write_errors(),
    })
}
