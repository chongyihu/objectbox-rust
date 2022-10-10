extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate prettyplease;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::punctuated::Pair;
use syn::NestedMeta::{Meta, self};
use syn::Meta::NameValue;
use syn::Data::Struct;
use syn::Lit::Int;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};

use std::option::Option;
use std::vec::Vec;

// TODO These consts should be SCREAMING UPPERCASE
const OBXPropertyType_Bool: u16 = 1;   // bool
const OBXPropertyType_Byte: u16 = 2;   // u8
const OBXPropertyType_Short: u16 = 3;  // i16 / u16
const OBXPropertyType_Char: u16 = 4;   // char
const OBXPropertyType_Int: u16 = 5;    // i32 / u32
const OBXPropertyType_Long: u16 = 6;   // i64 / u64
const OBXPropertyType_Float: u16 = 7;  // f32
const OBXPropertyType_Double: u16 = 8; // f64
const OBXPropertyType_String: u16 = 9; // str, String
const OBXPropertyType_Date: u16 = 10; // chrono::DateTime<Utc>, u64: unix epoch
const OBXPropertyType_Relation: u16 = 11; // TODO
const OBXPropertyType_DateNano: u16 = 12; // chrono::DateTime<Utc>, u64: unix epoch
const OBXPropertyType_Flex: u16 = 13; // tuple?! There is no 'Object' in rustlang
const OBXPropertyType_ByteVector: u16 = 23; // Vec<u8>, bytes, ByteArray, unsized byte slice, compile time statically sized array on stack
const OBXPropertyType_StringVector: u16 = 30; // Vec<str> / Vec<String>

/// For lack of the Debug trait in certain tokens
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

// For lack of the Debug trait in certain tokens
// TODO Figure out how to make this generic, since TokenStream2 inherits from TokenStream
// TODO maybe a simple 'x as proc_macro::TokenStream' might suffice
fn print_token_stream2(label: &str, stream: proc_macro2::TokenStream) {
  if cfg!(debug_assertions) {
    stream.into_iter()
      .for_each(|x| {
        println!("{} {:#?}", label, x);
      });
    println!("{}", "///");
  }
}

fn print_field_token_stream(field: &syn::Field, field_ident_str: String) {
  let field_ts = field.to_token_stream();
  let  debug_label = format!("field({}) token_stream", field_ident_str);
  print_token_stream2(&debug_label, field_ts);
}

// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see quote::format_ident
// TODO replace with serde

#[derive(Debug)]
struct IdUid {
  id: u64,
  uid: u64
}

#[derive(Debug)]
struct Field {
  name: String,
  field_type: u16,
  ///
  id: Option<IdUid>,
  uid: Option<u64>,
  unique: bool
  // TODO all the attributes
}

impl Field {
  fn from_syn_field(field: &syn::Field) -> Option<Field> {
    // TODO check if objectbox-model.json was generated
    // TODO compare values read from macro attributes

    let mut entity = 
      Field {
        name: String::new(),
        field_type: 0,
        id: None,
        uid: None,
        unique: false
    };

    if let Some(ident) = &field.ident {
      let new_name = ident.to_string();
      entity.name.push_str(&new_name);

      print_field_token_stream(field, new_name);

      // Attribute parsing
      for a in field.attrs.iter() {
        // get attribute name from `#[name]`
        if let Some(attr_path_ident) = a.path.get_ident() {
          let attr_name : &str = &attr_path_ident.to_string();
          match attr_name {
            "index" => {},
            "unique" => {},
            "backlink" => {},
            "transient" => {},
            "property" => {},
            _ => {} // skip if not ours
          }  
        }
        // I don't know why we need this yet
        // if let syn::parse::Result::Ok(m) = a.parse_meta() {
          /*
          enum Meta {
            Path(Path),
            List(MetaList),
            NameValue(MetaNameValue),
          }
          */
        // }
        let args_result = a.parse_args::<NestedMeta>();
        if let syn::parse::Result::Ok(args) = args_result {
          // TODO maybe Meta is a better choice, we'll see
          // TODO for multiple params you probably need to parse Punctuated NameValues
          // TODO maybe parsing the TokenStream is easier
        }
      }
      

      if let syn::Type::Path(p) = &field.ty {
          // p.to_token_stream(); or use syn::Type
          // TODO map values based on OBXPropertyType correspondence
      }

      return Some(entity);
    }

    Option::None
  }
}

// TODO Make this JSON serializable, or another like it, with an adapter 'from' in-between
// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see how generics work with this e.g. "struct Gen<T> { field: T }"
// TODO see how fields with Option<T> type, that default to None, and how store deals with this
// TODO check if another attribute macro can mess with our attribute, otherwise panic if another attribute is present
#[derive(Debug)]
struct Entity {
  name: String,
  id: Option<IdUid>,
  fields: Vec<Field>
}

impl Entity {
  /// Unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C.
  fn parse_entity_name_and_fields(id : IdUid, derive_input: DeriveInput) -> Entity {
    let mut entity = Entity {
      name: derive_input.ident.to_string(),
      id: Some(id),
      fields: Vec::new()
    };

    if let Struct(ds) = derive_input.data {
        match ds.fields {
          syn::Fields::Named(fields_named) => {
            fields_named.named.pairs().for_each(|p| {
              match p {
                Pair::Punctuated(t, _) => {
                  // TODO check for attribute: #[transient]
                  if let Some(f) = Field::from_syn_field(t) {
                    entity.fields.push(f);
                  }
                },
                Pair::End(t) => {
                  // TODO check for attribute: #[transient]
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
}

impl IdUid {
  fn from_attribute_args(args: AttributeArgs) -> IdUid {
    fn panic_only_id_uid_param_allowed() {
      panic!("Only the id=<integer>, uid=<integer> parameter are allowed");
    }

    if args.len() > 2 {
      panic_only_id_uid_param_allowed()
    }

    let mut uid : u64 = 0;
    let mut id  : u64 = 0;

    args.iter().for_each(|nm| {
      match nm {
        Meta(NameValue(mnv)) => {
          if let Int(li) = &mnv.lit {
            let result = li.base10_parse::<u64>();
            if let Ok(value) = result {
              if let Some(ident) = mnv.path.get_ident() {
                let param_name: &str = &ident.to_string();
                match param_name {
                  "uid" => { uid = value },
                  "id"  => { id = value },
                  _ => {
                    panic_only_id_uid_param_allowed();
                  }
                }
              }
            }
          }
        },
        _ => {}
      }
    });

    IdUid { id: id, uid: uid }
  }
}

/// This will break with nested sub types.
/// The last bit will remove the annotations in the generated code
/// because the generated code cannot reference the attributes.
/// The result of this is unused imported attributes.
// TODO also remove those unused imports, in the generated code
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
  // print_token_stream("all: ", input.clone());

  let struct_clone = input.clone();
  // all parse_macro_input! macro have to happen inside a proc_macro_attribute(d) function
  let struct_info = parse_macro_input!(struct_clone as DeriveInput);

  let attr_args = parse_macro_input!(args as AttributeArgs);
  let id = IdUid::from_attribute_args(attr_args);

  // TODO transform to objects objectbox-model-serde
  let entity = Entity::parse_entity_name_and_fields(id, struct_info);  

  input.into_iter().map(|x| {
    if let proc_macro::TokenTree::Group (group) = x {
      let new_group = group.stream().into_iter().filter(|y| {
        match y {
          // TODO make sure we only remove _our_ attributes (index, unique etc.)
          // TODO replace false and '#' with something more intelligent
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
