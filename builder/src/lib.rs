
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

// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see quote::format_ident
// TODO replace with serde
struct Field {
  name: String,
  field_type: u16,
  ///
  id: Option<u64>, // Do we do that id_uid thing?
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
            _ => {}
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
      

      let field_ts = field.to_token_stream();
      let  debug_label = format!("field({}) token_stream", ident.to_string());
      print_token_stream2(&debug_label, field_ts);
      
      // mental note: it's probably easier to parse the token stream,
      // than exhaustively go through these matches
      // This is here to figure out how different types are parsed (then call get_ident or something)
      let field_type = match &field.ty {
        syn::Type::Array(type_array) => {
          // TODO
          // println!("{} {:#?}", ident.to_string(), type_array);
          /*
          struct TypeArray {
            bracket_token: Bracket,
            elem: Box<Type>,
            semi_token: Semi,
            len: Expr,
          }
          */
          // For lack of the Debug trait in certain tokens
          println!("{} {}", debug_label, "Array");
          0
        },
        // BareFn(TypeBareFn),
        syn::Type::Group(type_group) => {
          // TODO
          /*
          struct TypeGroup {
            group_token: Group,
            elem: Box<Type>,
          }
          */
          // For lack of the Debug trait in certain tokens
          println!("{} {}", debug_label, "Group");
          0
        },
        // ImplTrait(TypeImplTrait),
        // Infer(TypeInfer),
        syn::Type::Macro(type_macro) => {
          println!("{} {}", debug_label, "Macro");
          0
        },
        // Never(TypeNever),
        // Paren(TypeParen),
        syn::Type::Path(type_path) => {
          println!("{} {}", debug_label, "Path");
          0
        },
        // Ptr(typePtr)
        // Reference(typeReference)
        syn::Type::Slice(type_slice) => {
          // TODO
          // For lack of the Debug trait in certain tokens
          println!("{} {}", debug_label, "Slice");
          0
        },
        // TraitObject(TypeTraitObject), 
        syn::Type::Tuple(type_tuple) => {
          // TODO
          // For lack of the Debug trait in certain tokens
          println!("{} {}", debug_label, "Tuple");          
          0
        }, // Flex?
        syn::Type::Verbatim(token_stream) => {
          // TODO
          // For lack of the Debug trait in certain tokens
          println!("{} {}", debug_label, "Verbatim");
          0
        }
        _ => {
          0
        }
      };

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
// TODO replace with serde
struct Entity {
  name: String,
  id: Option<u64>,
  uid: Option<u64>,
  fields: Vec<Field>
}

impl Entity {
  /// Unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C.
  fn parse_entity_name_and_fields(derive_input: DeriveInput) -> Entity {
    let mut entity = Entity {
      name: derive_input.ident.to_string(),
      id: Option::None,
      uid: Option::None,
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

  // TODO this can probably be reused to read the field attributes
  // TODO pass a lambda to the extracted function
  fn parse_entity_attribute_parameter(mut self, args: AttributeArgs) {
    fn panic_only_id_uid_param_allowed() {
      panic!("Only the uid=<integer> parameter is allowed");
    }

    if args.len() > 2 {
      panic_only_id_uid_param_allowed()
    }

    // TODO parse id and uid
    if let Some(Meta(NameValue(mnv))) = args.iter().next() {


      if let Int(li) = &mnv.lit {
        let result = li.base10_parse::<u64>();
        if let Ok(value) = result {
          // self.uid = Some(value);
          return;
        }
      }

      panic_only_id_uid_param_allowed();
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
  // print_token_stream("all: ", input.clone());

  let struct_clone = input.clone();
  let struct_info = parse_macro_input!(struct_clone as DeriveInput);
  let entity = Entity::parse_entity_name_and_fields(struct_info);

  let attr_args = parse_macro_input!(args as AttributeArgs);
  
  entity.parse_entity_attribute_parameter(attr_args);
  

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
