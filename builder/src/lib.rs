extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate prettyplease;
extern crate maplit;

use proc_macro::TokenStream;

// use quote::ToTokens;
use syn::punctuated::Pair;
use syn::Meta::NameValue;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};

use std::option::Option;
use std::vec::Vec;

#[path = "./objectbox-model-serde.rs"]
mod json;

#[path = "./id.rs"]
mod id;

/// For lack of the Debug trait in certain tokens
/// Only available in Debug mode
// fn print_token_stream(label: &str, stream: TokenStream) {
//   if cfg!(debug_assertions) {
//     stream.into_iter()
//       .for_each(|x| {
//         println!("{} {:#?}", label, x);
//       });
//     println!("{}", "///");
//   }
// }  

// For lack of the Debug trait in certain tokens
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

// fn print_field_token_stream(field: &syn::Field, field_ident_str: String) {
//   let field_ts = field.to_token_stream();
//   let  debug_label = format!("field({}) token_stream", field_ident_str);
//   // print_token_stream2(&debug_label, field_ts);
// }

#[derive(Debug)]
struct Field {
  name: String,
  field_type: u16,
  ///
  id: id::IdUid,
  unique: bool,
  index: bool,
  flags: Option<u16>,
}

impl Field {

  fn scan_obx_property_type_and_flags (mnv: &syn::MetaNameValue) -> (u16, Option<u16>) {
    let mut obx_property_type: u16 = 0;
    let mut obx_property_flags: Option<u16> = None;

    if let syn::Lit::Int(li) = &mnv.lit {
      let result = li.base10_parse::<u16>();
      if let Ok(value) = result {
        if let Some(ident) = mnv.path.get_ident() {
          let param_name: &str = &ident.to_string();
          match param_name {
            "type" => { obx_property_type = value },
            "flags" => { obx_property_flags = Some(value) }
            _ => {}
          }
        }
      }
    }
    (obx_property_type, obx_property_flags)
  }

  fn from_syn_field(field: &syn::Field) -> Option<Field> {
    // TODO check if objectbox-model.json was generated

    let mut name: String = String::new();
    let mut uid : Option<u64> = None;
    let mut id  : Option<u64> = None;
    let mut obx_property_type: u16 = 0;
    let mut obx_property_flags: Option<u16> = None;
    let mut unique: bool = false;
    let mut index: bool = false;
    
    if let Some(ident) = &field.ident {
      let new_name = ident.to_string();
      name.push_str(&new_name);

      // print_field_token_stream(field, new_name);

      // Attribute parsing
      for a in field.attrs.iter() {
        // get attribute name from `#[name]`
        if let Some(attr_path_ident) = a.path.get_ident() {
          let attr_name : &str = &attr_path_ident.to_string();
          match attr_name {
            "index" => { index = true }, // id, uid, type
            "unique" => { unique = true }, // id, uid, type
            // TODO the backlink symbols are lossy, in terms of type-safety, there will be problems
            // TODO with Structs referenced in another file / mod
            "backlink" => {},
            "transient" => { return None }, // no params
            "property" => {}, // id, uid, type, flags
            _ => {
              // skip if not ours
              continue;
            }
          }  
        }

        // TODO move out as generalized function with lambda
        // that parses depending on given attrib parameter names
        // given by 'index', 'backlink', 'transient', 'property'
        if let syn::parse::Result::Ok(m) = a.parse_meta() {
          match m {
            // single parameter
            syn::Meta::NameValue(mnv) => {
              (id, uid) = id::IdUid::scan_id_uid(&mnv);
              (obx_property_type, obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
            },
            // multiple parameters
            syn::Meta::List(meta_list) => {
              meta_list.nested.into_iter().for_each(|nm| {
                if let syn::NestedMeta::Meta(meta) = nm {
                  if let syn::Meta::NameValue(mnv) = meta {
                    (id, uid) = id::IdUid::scan_id_uid(&mnv);
                    (obx_property_type, obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
                  }
                }
              });
            },
            /* syn::Meta::Path(path) */ _ => {}
          }
        }
      }
      
      // TODO Skip type determination if provided in attribute
      // TODO anything can be in a 'Box'
      // Auto-map values based on likely OBXPropertyType correspondence
      
      if let (syn::Type::Path(p), true) = (&field.ty, obx_property_type == 0) {
        if let Some(ident) = p.path.get_ident() {
          let rust_type: &str = &ident.to_string();
          obx_property_type = match rust_type {
            "bool" => 1,
            "u8" => 2,
            "i8" => 2,
            "i16" => 3,
            "u16" => 3,
            "char" => 4,
            "u32" => 5,
            "i32" => 5,
            "u64" => 6,
            "i64" => 6,
            "f32" => 7,
            "f64" => 8,
            "String" => 9,
            _ => {
              // ignore scoping
              let map = maplit::hashmap! {
                "Vec<u8>" => 23,
                "bytes" => 23,
                "ByteArray" => 23,
                "Vec<String>" => 30,
              };
              let mut ends_with = 0;
              for ele in map {
                  if rust_type.ends_with(ele.0) {
                    ends_with = ele.1;
                    break;
                  }
              }
              if rust_type.contains("DateTime") {
                ends_with = 12
              }
              if ends_with == 0 {
                eprintln!("Warning: Unknown translation of rust type {}", rust_type);
              }
              ends_with
            }
          };
        }

      }

      let id_uid = id::IdUid {
        id: id,
        uid: uid,
      };
      let field = Field {
          name: name,
          field_type: obx_property_type,
          id: id_uid,
          unique: unique,
          index: index,
          flags: obx_property_flags,
      };
      return Some(field);
    }

    Option::None
  }
}

// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see how generics work with this e.g. "struct Gen<T> { field: T }"
// TODO see how fields with Option<T> type, that default to None, and how store deals with this
// TODO check if another attribute macro can mess with our attribute, otherwise panic if another attribute is present
#[derive(Debug)]
struct Entity {
  name: String,
  id: id::IdUid,
  fields: Vec<Field>,
}

impl Entity {
  /// Unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C.
  fn from_entity_name_and_fields(id : id::IdUid, derive_input: DeriveInput) -> Entity {
    let mut fields = Vec::<Field>::new();
    if let syn::Data::Struct(ds) = derive_input.data {
        match ds.fields {
          syn::Fields::Named(fields_named) => {
            fields_named.named.pairs().for_each(|p| {
              match p {
                Pair::Punctuated(t, _) => {
                  // TODO check for attribute: #[transient]
                  if let Some(f) = Field::from_syn_field(t) {
                    fields.push(f);
                  }
                },
                Pair::End(t) => {
                  // TODO check for attribute: #[transient]
                  if let Some(f) = Field::from_syn_field(t) {
                    fields.push(f);
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
    if fields.is_empty() {
      panic!("Structs must have at least one attribute / property!");
    }
    Entity {
      name: derive_input.ident.to_string(),
      id: id,
      fields: fields
    }
  }

  fn get_last_property_id(&self) -> id::IdUid {
    if let Some(field) = self.fields.last() {
      return field.id.clone()
    }
    // TODO throw an error down the road, this should never happen
    // TODO write test with an Entity without properties
    id::IdUid { id: None, uid: None }
  }

  fn get_properties(&self) -> Vec<json::Property> {
    let mut v: Vec<json::Property> = Vec::new();
    for f in self.fields.iter() {
      let p = json::Property {
        id: f.id.to_string(),
        name: f.name.clone(),
        type_field: f.field_type,
        flags: f.flags,
      };
      v.push(p);
    }
    v
  }

  fn serialize(&self) -> json::Entity {
    json::Entity {
      id: self.id.to_string(),
        last_property_id: self.get_last_property_id().to_string(),
        name: self.name.clone(),
        properties: self.get_properties(),
        // TODO Optional relations, see flags
    }
  }
}

impl id::IdUid {

  fn scan_id_uid (mnv: &syn::MetaNameValue) -> (Option<u64>, Option<u64>) {
    let mut id: Option<u64> = None;
    let mut uid: Option<u64> = None;

    if let syn::Lit::Int(li) = &mnv.lit {
      let result = li.base10_parse::<u64>();
      if let Ok(value) = result {
        if let Some(ident) = mnv.path.get_ident() {
          let param_name: &str = &ident.to_string();
          match param_name {
            "uid" => { uid = Some(value) },
            "id"  => { id = Some(value) },
            _ => {}
          }
        }
      }
    }
    (id, uid)
  }

  fn from_nested_metas(iter: core::slice::Iter::<syn::NestedMeta>) -> id::IdUid {
    let mut uid : Option<u64> = None;
    let mut id  : Option<u64> = None;

    iter.for_each(|nm| {
      match nm {
        syn::NestedMeta::Meta(NameValue(mnv)) => {
          (id, uid) = Self::scan_id_uid(mnv);
        },
        _ => {}
      }
    });

    id::IdUid { id: id, uid: uid }
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
  let id = id::IdUid::from_nested_metas(attr_args.iter());

  let entity = Entity::from_entity_name_and_fields(id, struct_info);
  entity.serialize().write();
  // println!("{:#?}", entity);

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

// TODO test combinations of these obx property types & flags
/*
      // TODO These consts should be SCREAMING UPPERCASE
      const OBXPropertyType_Bool: u16 = 1;   // bool
      const OBXPropertyType_Byte: u16 = 2;   // i8 / u8
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

      typedef enum {
    /// 64 bit long property (internally unsigned) representing the ID of the entity.
    /// May be combined with: NON_PRIMITIVE_TYPE, ID_MONOTONIC_SEQUENCE, ID_SELF_ASSIGNABLE.
    OBXPropertyFlags_ID = 1,

    /// On languages like Java, a non-primitive type is used (aka wrapper types, allowing null)
    OBXPropertyFlags_NON_PRIMITIVE_TYPE = 2,

    /// Unused yet
    OBXPropertyFlags_NOT_NULL = 4,

    OBXPropertyFlags_INDEXED = 8,

    /// Unused yet
    OBXPropertyFlags_RESERVED = 16,

    /// Unique index
    OBXPropertyFlags_UNIQUE = 32,

    /// Unused yet: Use a persisted sequence to enforce ID to rise monotonic (no ID reuse)
    OBXPropertyFlags_ID_MONOTONIC_SEQUENCE = 64,

    /// Allow IDs to be assigned by the developer
    OBXPropertyFlags_ID_SELF_ASSIGNABLE = 128,

    /// Unused yet
    OBXPropertyFlags_INDEX_PARTIAL_SKIP_NULL = 256,

   /// Used by References for 1) back-references and 2) to clear references to deleted objects (required for ID reuse)
    OBXPropertyFlags_INDEX_PARTIAL_SKIP_ZERO = 512,

    /// Virtual properties may not have a dedicated field in their entity class, e.g. target IDs of to-one relations
    OBXPropertyFlags_VIRTUAL = 1024,

    /// Index uses a 32 bit hash instead of the value
    /// 32 bits is shorter on disk, runs well on 32 bit systems, and should be OK even with a few collisions
    OBXPropertyFlags_INDEX_HASH = 2048,

    /// Index uses a 64 bit hash instead of the value
    /// recommended mostly for 64 bit machines with values longer >200 bytes; small values are faster with a 32 bit hash
    OBXPropertyFlags_INDEX_HASH64 = 4096,

    /// The actual type of the variable is unsigned (used in combination with numeric OBXPropertyType_*).
    /// While our default are signed ints, queries & indexes need do know signing info.
    /// Note: Don't combine with ID (IDs are always unsigned internally).
    OBXPropertyFlags_UNSIGNED = 8192,

    /// By defining an ID companion property, a special ID encoding scheme is activated involving this property.
    ///
    /// For Time Series IDs, a companion property of type Date or DateNano represents the exact timestamp.
    OBXPropertyFlags_ID_COMPANION = 16384,

    /// Unique on-conflict strategy: the object being put replaces any existing conflicting object (deletes it).
    OBXPropertyFlags_UNIQUE_ON_CONFLICT_REPLACE = 32768,

    /// If a date property has this flag (max. one per entity type), the date value specifies the time by which
    /// the object expires, at which point it MAY be removed (deleted), which can be triggered by an API call.
    OBXPropertyFlags_EXPIRATION_TIME = 65536,
      */