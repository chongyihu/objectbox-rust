
extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate prettyplease;

use proc_macro::TokenStream;

use syn::punctuated::Pair;
use syn::NestedMeta::Meta;
use syn::Meta::NameValue;
use syn::Data::Struct;
use syn::Lit::Int;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};

use std::option::Option;
use std::vec::Vec;

// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see quote::format_ident
struct Field {
  name: String,
  ty: syn::Type,
  vis: syn::Visibility,
  ///
  id: Option<u64>,
  uid: Option<u64>,
  unique: bool
}

impl Field {
  fn from_syn_field(field: &syn::Field) -> Option<Field> {
    if let Some(ident) = &field.ident {
      return Some(Field {
        name: ident.to_string(),
        ty: field.ty.clone(),
        vis: field.vis.clone(),
        id: None, // TODO
        uid: None, // TODO
        unique: false // TODO
      });
    }

    Option::None
  }
}

// TODO Make this JSON serializable
// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see how generics work with this e.g. "struct Gen<T> { field: T }"
// TODO see how fields with Option<T> type, that default to None, and how store deals with this
// TODO check if another attribute macro can mess with our attribute, otherwise panic if another attribute is present
struct Entity {
  name: String,
  uid: Option<u64>,
  fields: Vec<Field>
}

/// Only available in Debug mode
fn print_token_stream(label: &str, stream: TokenStream) {
  if cfg!(debug_assertions) {
    stream.into_iter()
      .for_each(|x| {
        println!("{} {:#?}", label, x);
      });
    println!("{}", "///");
  }
}  

// TODO Figure out how to make this generic, since TokenStream2 inherits from TokenStream
// TODO maybe a simple 'x as proc_macro::TokenStream' might suffice
// fn print_token_stream2(label: &str, stream: proc_macro2::TokenStream) {
//   if cfg!(debug_assertions) {
//     stream.into_iter()
//       .for_each(|x| {
//         println!("{} {:#?}", label, x);
//       });
//     println!("{}", "///");
//   }
// }  

impl Entity {
  /// unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C
  fn parse_entity_name_and_fields(derive_input: DeriveInput) -> Entity {
    let mut entity = Entity {
      name: derive_input.ident.to_string(),
      uid: Option::None,
      fields: Vec::new()
    };
    if let Struct(ds) = derive_input.data {
        match ds.fields {
          syn::Fields::Named(fields_named) => {
            fields_named.named.pairs().for_each(|p| {
              match p {
                Pair::Punctuated(t, _) => {
                  if let Some(f) = Field::from_syn_field(t) {
                    entity.fields.push(f);
                  }
                },
                Pair::End(t) => {
                  if let Some(f) = Field::from_syn_field(t) {
                    entity.fields.push(f);
                  }
                }
              }
            });
          }
          _ => {}
        }
    }else {
      panic!("This macro attribute is only applicable on structs");
    }
    entity
  }

  // TODO this can probably be reused to read the field attributes
  // TODO pass a lambda to the extracted function
  fn parse_entity_attribute_parameter(mut self, args: AttributeArgs) {
    fn panic_only_one_param_allowed() {
      panic!("Only the uid=<integer> parameter is allowed");
    }

    if args.len() > 1 {
      panic_only_one_param_allowed()
    }

    if let Some(Meta(NameValue(mnv))) = args.iter().next() {
      if !mnv.path.is_ident("uid") {
        panic_only_one_param_allowed();
      }

      if let Int(li) = &mnv.lit {
        let result = li.base10_parse::<u64>();
        if let Ok(value) = result {
          self.uid = Some(value);
          return;
        }
      }

      panic_only_one_param_allowed();
    }
  }
}

/// This will break with nested sub types.
/// The last bit will remove the annotations in the generated code
/// because the generated code cannot reference the attributes.
/// The result of this is unused imported attributes.
// TODO make sure we only remove _our_ attributes (index, unique etc.)
// TODO also remove those imports
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
  print_token_stream("all: ", input.clone());

  let struct_clone = input.clone();
  let struct_info = parse_macro_input!(struct_clone as DeriveInput);
  let entity = Entity::parse_entity_name_and_fields(struct_info);

  let attr_args = parse_macro_input!(args as AttributeArgs);
  {
    entity.parse_entity_attribute_parameter(attr_args);
  }

  input.into_iter().map(|x| {
    if let proc_macro::TokenTree::Group (group) = x {
      let new_group = group.stream().into_iter().filter(|y| {
        match y {
          proc_macro::TokenTree::Group(_) => false,
          proc_macro::TokenTree::Punct(p) => p.as_char() != '#',
          _ => true
        }
      }).collect::<TokenStream>();
      let pm_group = proc_macro::Group::new(group.delimiter(), new_group);
      proc_macro::TokenTree::from(pm_group)
    }else {
      x
    }
  }).collect::<TokenStream>()
}


#[proc_macro_attribute]
pub fn sync(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Fields

/// Accepts 'type' parameter
/// Note: indexes are currently not supported for ByteVector, Float or Double
/// See ./objectbox/lib/src/annotations.dart implementation
#[proc_macro_attribute]
pub fn index(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Accepts 'uid'
#[proc_macro_attribute]
pub fn unique(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Accepts 'to'
#[proc_macro_attribute]
pub fn backlink(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// No required params
#[proc_macro_attribute]
pub fn transient(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Accepts 'type' and 'uid'
#[proc_macro_attribute]
pub fn property(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}
