extern crate alloc;
extern crate proc_macro;

use proc_macro::TokenStream;
use crate::parse::{Category, Enum, Field, Type};

#[macro_use]
mod shared;

mod parse;

fn ser_proxy_guard(fieldname: &str, field: &Field) -> String {
    if let Some(proxy) = crate::shared::attrs_proxy(&field.attributes) {
        if field.ty.base() == "Option" {
            format!(
                "{{{fieldname}.as_ref().map(|f| {{let proxy: {proxy} = Into::into(f);proxy}})}}"
            )
        } else {
            format!("{{let proxy: {proxy} = Into::into(&{fieldname});proxy}}",)
        }
    } else {
        format!("{fieldname}")
    }
}

fn derive_ser_json_enum(enum_: &Enum) -> TokenStream {
    let mut r = String::new();

    for variant in enum_.variants.iter() {
        let field_name = variant.field_name.clone().unwrap();
        let json_variant_name =
            shared::attrs_rename(&variant.attributes).unwrap_or(field_name.clone());

        match &variant.ty {
            Type {
                wraps: None,
                ident: Category::None,
                ..
            } => {
                // unit variant
                l!(
                    r,
                    "Self::{} => s.label(\"{}\"),",
                    &field_name,
                    json_variant_name
                );
            }

            Type {
                ident: Category::AnonymousStruct { contents },
                ..
            } => {
                let mut items = String::new();
                let mut field_names = vec![];
                let last = contents.fields.len().saturating_sub(1);
                for (index, field) in contents.fields.iter().enumerate() {
                    if let Some(name) = &&field.field_name {
                        let proxied_field = ser_proxy_guard(name, field);
                        if index == last {
                            if field.ty.base() == "Option" {
                                l!(
                                    items,
                                    "if {}.is_some(){{s.field(d+1, \"{}\");{}.ser_json(d+1, s);}}",
                                    name,
                                    name,
                                    proxied_field
                                )
                            } else {
                                l!(
                                    items,
                                    "s.field(d+1, \"{}\");{}.ser_json(d+1, s);",
                                    name,
                                    proxied_field
                                )
                            }
                        } else {
                            if field.ty.base() == "Option" {
                                l!(
                                        items,
                                        "if {}.is_some(){{s.field(d+1, \"{}\");{}.ser_json(d+1, s);s.conl();}}",
                                        name,
                                        name,
                                        proxied_field
                                    );
                            } else {
                                l!(
                                    items,
                                    "s.field(d+1, \"{}\");{}.ser_json(d+1, s);s.conl();",
                                    name,
                                    proxied_field
                                );
                            }
                        }
                        field_names.push(name.clone());
                    }
                }
                l!(
                    r,
                    "Self::{} {{ {} }} => {{
                                s.out.push('{{');
                                s.label(\"type\");
                                s.out.push(':');
                                s.label(\"{}\");
                                s.out.push(',');
                                s.label(\"value\");
                                s.out.push(':');
                                s.st_pre();
                                {}
                                s.st_post(d);
                                s.out.push('}}');
                            }}",
                    &field_name,
                    field_names.join(","),
                    json_variant_name,
                    items
                );
            }
            Type {
                ident: Category::Tuple { contents },
                ..
            } => {
                let mut names = Vec::new();
                let mut inner = String::new();
                let last = contents.len() - 1;
                for (index, _) in contents.iter().enumerate() {
                    let field_name = format!("f{}", index);
                    names.push(field_name.clone());
                    if index != last {
                        l!(inner, "{}.ser_json(d, s); s.out.push(',');", field_name);
                    } else {
                        l!(inner, "{}.ser_json(d, s);", field_name);
                    }
                }
                l!(
                    r,
                    "Self::{}  ({}) => {{
                                s.out.push('{{');
                                s.label(\"type\");
                                s.out.push(':');
                                s.label(\"{}\");
                                s.out.push(',');
                                s.label(\"value\");
                                s.out.push(':');
                                s.out.push('[');
                                {}
                                s.out.push(']');
                                s.out.push('}}');
                            }}",
                    &field_name,
                    names.join(","),
                    json_variant_name,
                    inner
                );
            }
            v => {
                unimplemented!("Unexpected type in enum: {:?}", v)
            }
        };
    }

    format!(
        "
        impl SerJson for {} {{
            fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {{
                match self {{
                    {}
                }}
            }}
        }}",
        enum_.name, r
    )
        .parse()
        .unwrap()
}

#[proc_macro_derive(SerJsonEnumPretty, attributes(nserde))]
pub fn derive_ser_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::parse_data(input);

    // ok we have an ident, its either a struct or a enum
    let ts = match &input {
        parse::Data::Enum(enum_) => derive_ser_json_enum(enum_),
        _ => unimplemented!(""),
    };

    ts
}