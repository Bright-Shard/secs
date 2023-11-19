#![no_std]
extern crate alloc;

use {
    alloc::{
        format,
        str::FromStr,
        string::{String, ToString},
    },
    proc_macro::{TokenStream, TokenTree},
};

#[proc_macro]
pub fn impl_bundle(input: TokenStream) -> TokenStream {
    let mut generics = String::new();
    let mut types = String::new();
    let mut components_impl = String::new();
    let mut current_component = 0;

    for token in input {
        generics += &format!("{token}: Component, ");
        types += &format!("{token},");
        components_impl += &format!("Box::new(self.{current_component}), ");
        current_component += 1;
    }

    TokenStream::from_str(&format!(
        "
        impl<{generics}> Bundle for ({types}) {{
            fn components(self) -> Vec<Box<dyn Component>> {{
                vec![{components_impl}]
            }}
            
            fn components_from_box(self: Box<Self>) -> Vec<Box<dyn Component>> {{
                vec![{components_impl}]
            }}
        }}
        "
    ))
    .unwrap()
}

#[proc_macro]
pub fn impl_system_param_fn(input: TokenStream) -> TokenStream {
    let mut generics = String::from("<Function, ");
    let mut types_tuple = String::from("(");
    let mut fn_args = String::new();
    let mut fn_call_args = String::new();
    let mut variables = String::new();
    let mut releases = String::new();

    for token in input {
        generics += &format!("{token}: SystemParam, ");
        types_tuple += &format!("{token}, ");
        fn_args += &format!("{token}::Fetch<'_>, ");

        let var_name = token.to_string().to_lowercase();
        variables += &format!("let mut {var_name} = {token}::Data::take(world);\n");
        fn_call_args += &format!("{token}::fetch(&mut {var_name}),");
        releases += &format!("{var_name}.release(world);\n");
    }

    generics += ">";
    types_tuple += ")";

    TokenStream::from_str(&format!(
        "
        impl {generics} SystemParamFn<{types_tuple}> for Function
        where
            for <'a> &'a Function: Fn({fn_args}) + Fn{types_tuple},
        {{
            fn execute(&self, world: &mut World) {{
                {variables}
                (&self)({fn_call_args});
                {releases}
            }}
        }}
        "
    ))
    .unwrap()
}

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let mut source = input.into_iter().peekable();

    // Skip over useless tokens
    while let Some(token) = source.peek() {
        match token {
            TokenTree::Punct(_) => source.next(),
            TokenTree::Group(_) => source.next(),
            TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "pub" => source.next(),
                "struct" => source.next(),
                _ => break,
            },
            _ => unreachable!("Unexpected token while deriving Component"),
        };
    }

    // Get the struct's name
    let struct_name = source.next().unwrap().to_string();

    TokenStream::from_str(&format!(
        "
        impl secs::entity::Component for {struct_name} {{
            fn prep_storage(&self, storage: &mut secs::world::storage::Storage) {{
                storage.prep_for::<Self>();
            }}
        }}
        "
    ))
    .unwrap()
}
