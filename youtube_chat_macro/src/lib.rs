use itertools::Itertools;
use proc_macro2;
use quote::{format_ident, quote};

static TYPE_VARS: [&str; 4] = ["SF", "ENF", "CF", "ERF"];
static FN_FIELD_NAMES: [&str; 4] = ["on_start", "on_end", "on_chat", "on_error"];
static EMPTY_TYPE: &str = "()";

#[proc_macro]
pub fn gen_builder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_builder_impl(tokens.into()).into()
}

fn gen_builder_impl(_tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let types: Vec<proc_macro2::TokenStream> = TYPE_VARS
        .into_iter()
        .map(|name| name.parse().unwrap())
        .collect();
    let field_values: Vec<proc_macro2::TokenStream> = FN_FIELD_NAMES
        .into_iter()
        .map(|name| name.parse().unwrap())
        .map(|token: proc_macro2::TokenStream| quote!(self.#token))
        .collect();
    let empty_type: proc_macro2::TokenStream = EMPTY_TYPE.parse().unwrap();
    let n_types = types.len();
    let mut tokens = proc_macro2::TokenStream::new();
    for bit in 0..(1 << n_types) {
        let flags: Vec<bool> = (0..n_types).map(|x| (bit & (1 << x)) != 0).collect();
        let types: Vec<(
            proc_macro2::TokenStream,
            proc_macro2::TokenStream,
            proc_macro2::TokenStream,
            bool, // flag
        )> = flags
            .iter()
            .enumerate()
            .map(|(idx, &flag)| {
                if flag {
                    let ty = types[idx].clone();
                    let where_clause = type_var_to_where_constraint(&ty);
                    (ty, field_values[idx].clone(), where_clause, flag)
                } else {
                    (
                        empty_type.clone(),
                        empty_type.clone(),
                        proc_macro2::TokenStream::new(),
                        flag,
                    )
                }
            })
            .collect();
        let type_vars: proc_macro2::TokenStream = flags
            .iter()
            .enumerate()
            .filter_map(|(idx, &flag)| {
                if flag {
                    Some(types[idx].0.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
            .parse()
            .unwrap();
        let is_all_empty_type = types
            .iter()
            .map(|(_, _, _, flag)| flag)
            .fold(true, |acc, is_not_empty| acc && !is_not_empty);
        let (SF, ENF, CF, ERF) = (
            types[0].0.clone(),
            types[1].0.clone(),
            types[2].0.clone(),
            types[3].0.clone(),
        );
        let (on_start, on_end, on_chat, on_error) = (
            types[0].1.clone(),
            types[1].1.clone(),
            types[2].1.clone(),
            types[3].1.clone(),
        );
        let where_clauses: proc_macro2::TokenStream = flags
            .iter()
            .enumerate()
            .filter_map(|(idx, &flag)| {
                if flag {
                    Some(types[idx].2.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
            .parse()
            .unwrap();
        let build_impl = if is_all_empty_type {
            quote!(
                impl LiveChatClientBuilder<String, #SF, #ENF, #CF, #ERF> {
                    pub fn build(self) -> LiveChatClient<#SF, #ENF, #CF, #ERF> {
                        LiveChatClient {
                            live_url: self.live_url,
                            on_start: #on_start,
                            on_end: #on_end,
                            on_chat: #on_chat,
                            on_error: #on_error,
                            options: None,
                        }
                    }
                }
            )
        } else {
            quote!(
                impl<#type_vars> LiveChatClientBuilder<String, #SF, #ENF, #CF, #ERF>
                where
                    #where_clauses
                {
                    pub fn build(self) -> LiveChatClient<#SF, #ENF, #CF, #ERF> {
                        LiveChatClient {
                            live_url: self.live_url,
                            on_start: #on_start,
                            on_end: #on_end,
                            on_chat: #on_chat,
                            on_error: #on_error,
                            options: None,
                        }
                    }
                }
            )
        };
        tokens.extend(build_impl);
    }
    tokens
}

fn type_var_to_where_constraint(type_var: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match type_var.to_string().as_str() {
        "SF" => quote!(#type_var: Fn(String)),
        "ENF" => quote!(#type_var: Fn()),
        "CF" => quote!(#type_var: Fn()),
        "ERF" => quote!(#type_var: Fn()),
        t => unreachable!("unexpected type var: {}", t),
    }
}

#[proc_macro]
pub fn gen_invokes(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_invokes_impl(tokens.into()).into()
}

fn gen_invokes_impl(_tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    for (handling_idx, (type_var, fn_name)) in TYPE_VARS.into_iter().zip(FN_FIELD_NAMES).enumerate()
    {
        let type_declarations_for_without_fn: proc_macro2::TokenStream = TYPE_VARS
            .into_iter()
            .enumerate()
            .filter_map(|(idx, val)| (idx != handling_idx).then(|| val))
            .join(",")
            .parse()
            .unwrap();
        let live_chat_client_types_for_without_fn: proc_macro2::TokenStream = TYPE_VARS
            .into_iter()
            .enumerate()
            .map(
                |(idx, val)| {
                    if idx == handling_idx {
                        EMPTY_TYPE
                    } else {
                        val
                    }
                },
            )
            .join(",")
            .parse()
            .unwrap();
        let fn_name = format_ident!("invoke_{}", fn_name);
        tokens.extend(quote!(
            impl <#type_declarations_for_without_fn> LiveChatClient<#live_chat_client_types_for_without_fn> {
               fn #fn_name(&self, ) 
            }
        ));
    }
    tokens
}
/*
impl<ENF, CF, ERF> LiveChatClient<(), ENF, CF, ERF> {
    pub fn invoke_start(&self, live_id: String) {}
}

impl<SF, ENF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF>
where
    SF: Fn(String),
{
    pub fn invoke_start(&self, live_id: String) {
        (self.on_start)(live_id)
    }
}
*/

#[test]
fn snapshot_impl() {
    let expanded = gen_builder_impl(quote!());
    insta::assert_display_snapshot!(expanded.to_string());
}
