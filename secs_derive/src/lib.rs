use proc_macro::{TokenStream, TokenTree};
use std::str::FromStr;

#[proc_macro]
pub fn glue_tokens(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    TokenStream::from_str(&format!("{first}{second}")).unwrap()
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
        // fn_call_args += &format!("{token}::new(&mut {var_name}), ");
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
        impl Component for {struct_name} {{
            fn prep_archetype(&self, world: &mut World) {{
                world.prep_archetype::<{struct_name}>();
            }}

            fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {{
                self as Box<dyn std::any::Any>
            }}
        }}
        "
    ))
    .unwrap()
}
