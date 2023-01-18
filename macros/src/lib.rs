extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate prettyplease;
extern crate maplit;

use proc_macro::TokenStream;

use quote::__private::ext::RepToTokensExt;
// use quote::ToTokens;

use syn::Meta::NameValue;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};

use std::option::Option;
use syn::punctuated::Pair;

use objectbox_generator::model_json;
use objectbox_generator::id;
use objectbox_generator::ob_consts as consts;

// For lack of the Debug trait in certain tokens (that I know of, in this version)
// Only available in Debug mode
fn print_token_stream(label: &str, stream: TokenStream) {
  if cfg!(debug_assertions) {
    stream.into_iter()
      .for_each(|x| {
        println!("{} {:#?}", label, x);
      });
    println!("{}", "///");
  }
}  

// For lack of the Debug trait in certain tokens (that I know of, in this version)
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
  let field_ts = quote::ToTokens::to_token_stream(&field);
  let  debug_label = format!("field({}) token_stream", field_ident_str);
  print_token_stream2(&debug_label, field_ts);
}

// TODO implement flags, reference: https://github.com/objectbox/objectbox-dart/blob/main/generator/lib/src/entity_resolver.dart#L23-L30
#[derive(Debug)]
struct Property {
  name: String,
  field_type: consts::OBXPropertyType,
  id: id::IdUid,
  flags: consts::OBXPropertyFlags,
}

impl Property {

  fn new() -> Self {
    Property {
        name: String::new(),
        field_type: 0,
        id: id::IdUid::zero(),
        flags: 0,
    }
  }

  fn scan_obx_property_type_and_flags (mnv: &syn::MetaNameValue) -> (consts::OBXPropertyType, consts::OBXPropertyFlags) {
    let mut obx_property_type: consts::OBXPropertyType = 0;
    let mut obx_property_flags: consts::OBXPropertyFlags = 0;

    if let syn::Lit::Int(li) = &mnv.lit {
      let result = li.base10_parse::<consts::OBXPropertyFlags>();
      if let Ok(value) = result {
        if let Some(ident) = mnv.path.get_ident() {
          let param_name: &str = &ident.to_string();
          match param_name {
            "type" => { obx_property_type = value },
            "flags" => { obx_property_flags = value }
            _ => {}
          }
        }
      }
    }
    (obx_property_type, obx_property_flags)
  }


  fn from_syn_field(field: &syn::Field) -> Option<Property> {
    let mut property = Property::new();

    let Property {
      name,
      field_type: obx_property_type,
      id,
      flags: obx_property_flags} = &mut property;
    
    if let Some(ident) = &field.ident {
      let new_name = ident.to_string();
      name.push_str(&new_name);

      print_field_token_stream(field, new_name);

      // TODO Document: for the minimal demo, make sure entities are
      // TODO declared on the src/lib.rs or src/main.rs and are pub
      // Attribute parsing
      for a in field.attrs.iter() {
        // get attribute name from `#[name]`
        if let Some(attr_path_ident) = a.path.get_ident() {
          let attr_name : &str = &attr_path_ident.to_string();
          // TODO add safety precaution measures
          // TODO add extra parameters
          match attr_name {
            "id" => { *obx_property_flags |= consts::OBXPropertyFlags_ID }
            "index" => { *obx_property_flags |= consts::OBXPropertyFlags_INDEXED }, // id, uid, type
            "unique" => { *obx_property_flags |= consts::OBXPropertyFlags_UNIQUE }, // id, uid, type
            "backlink" => {},
            "transient" => { a.next(); }, // TODO test if this really skips
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
              id.update_from_scan(&mnv);
              (*obx_property_type, *obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
            },
            // multiple parameters
            syn::Meta::List(meta_list) => {
              meta_list.nested.into_iter().for_each(|nm| {
                if let syn::NestedMeta::Meta(meta) = nm {
                  if let syn::Meta::NameValue(mnv) = meta {
                    id.update_from_scan(&mnv);
                    (*obx_property_type, *obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
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
      // TODO Auto-map values based on likely OBXPropertyType correspondence
      // TODO currently there is an issue with any field that contains an aggregate type, i.e. Vec
      if let (syn::Type::Path(p), true) = (&field.ty, *obx_property_type == 0) {
        if let Some(ident) = p.path.get_ident() {
          let rust_type: &str = &ident.to_string();
          println!("rust type: {}", rust_type);
          if rust_type.starts_with("u") {
            *obx_property_flags |= consts::OBXPropertyFlags_UNSIGNED;
          }
          // C's char is 1 byte, Rust's is 4 bytes (aka a vector, n=4 bytes, OBXPropertyType_ByteVector)
          *obx_property_type = match rust_type {
            "bool" => consts::OBXPropertyType_Bool,
            "u8" => consts::OBXPropertyType_Byte,
            "i8" => consts::OBXPropertyType_Byte,
            "i16" => consts::OBXPropertyType_Short,
            "u16" => consts::OBXPropertyType_Short,
            // TODO ob: char ==> u8
            // TODO rust: char ==> 4*u8 ==> u32
            // TODO what could go wrong?
            "char" => consts::OBXPropertyType_Char, 
            "u32" => consts::OBXPropertyType_Int,
            "i32" => consts::OBXPropertyType_Int,
            "u64" => consts::OBXPropertyType_Long,
            "i64" => consts::OBXPropertyType_Long,
            "f32" => consts::OBXPropertyType_Float,
            "f64" => consts::OBXPropertyType_Double,
            "String" => consts::OBXPropertyType_String,
            "Vec<u8>" => consts::OBXPropertyType_ByteVector, // TODO parsing is broken
            "Vec<String>" => consts::OBXPropertyType_StringVector, // TODO parsing is broken
            _ => 0
        }
      }
      return Some(property);
    }
    }
    None
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
  fields: Vec<Property>,
}

impl Entity {
  /// Unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C.
  fn from_entity_name_and_fields(id : id::IdUid, derive_input: DeriveInput) -> Entity {
    let mut fields = Vec::<Property>::new();
    if let syn::Data::Struct(ds) = derive_input.data {
        match ds.fields {
          syn::Fields::Named(fields_named) => {
            fields_named.named.pairs().for_each(|p| {
              match p {
                Pair::Punctuated(t, _) => {
                  // TODO check for attribute: #[transient]
                  if let Some(f) = Property::from_syn_field(t) {
                    if f.field_type == 0 {
                      println!("Warning: There is a field {} with an unmappable type", f.name);
                    }
                    fields.push(f);
                  }
                },
                Pair::End(t) => {
                  // TODO check for attribute: #[transient]
                  if let Some(f) = Property::from_syn_field(t) {
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
    id::IdUid::zero()
  }

  fn get_properties(&self) -> Vec<model_json::ModelProperty> {
    let mut v: Vec<model_json::ModelProperty> = Vec::new();
    for f in self.fields.iter() {
      let flags = if f.flags == 0 { None } else { Some(f.flags) };
      let p = model_json::ModelProperty {
        id: f.id.to_string(),
        name: f.name.clone(),
        type_field: f.field_type,
        flags: flags,
      };
      v.push(p);
    }
    v
  }

  fn serialize(&self) -> model_json::ModelEntity {
    model_json::ModelEntity {
      id: self.id.to_string(),
        last_property_id: self.get_last_property_id().to_string(),
        name: self.name.clone(),
        properties: self.get_properties(),
        relations: Vec::new(), // TODO
        // path: None,
        // TODO see flags
    }
  }
}

// extension trait for IdUid, boilerplatey, but doable
trait IdUidMacroHelper {
  fn update_from_scan(&mut self, mnv: &syn::MetaNameValue);
  fn update_from_nested_metas(&mut self, iter: core::slice::Iter::<syn::NestedMeta>);
}

impl IdUidMacroHelper for id::IdUid {
    fn update_from_scan(&mut self, mnv: &syn::MetaNameValue) {
      if let syn::Lit::Int(li) = &mnv.lit {
        let result = li.base10_parse::<u64>();
        if let Ok(value) = result {
          if let Some(ident) = mnv.path.get_ident() {
            let param_name: &str = &ident.to_string();
            match param_name {
              "uid" => {
                if self.uid == 0 {
                  self.uid = value
                }
              },
              "id"  => {
                if self.id == 0 {
                  self.id = value
                }
              },
              _ => {}
            }
          }
        }
      }
    }

    fn update_from_nested_metas(&mut self, iter: core::slice::Iter::<syn::NestedMeta>) {
      iter.for_each(|nm| {
        match nm {
          syn::NestedMeta::Meta(NameValue(mnv)) => {
            self.update_from_scan(mnv);
          },
          _ => {}
        }
      });
    }
}

// This will break with nested sub types.
// The last bit will remove the annotations in the generated code
// because the generated code cannot reference the attributes.
// The result of this is unused imported attributes.
// TODO also remove those unused imports, in the generated code
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
  // print_token_stream("all: ", input.clone());

  let struct_clone = input.clone();
  // all parse_macro_input! macro have to happen inside a proc_macro_attribute(d) function
  let struct_info = parse_macro_input!(struct_clone as DeriveInput);

  let attr_args = parse_macro_input!(args as AttributeArgs);
  let mut id = id::IdUid::zero();

  if !attr_args.is_empty() {
    id.update_from_nested_metas(attr_args.iter());
  }

  let entity = Entity::from_entity_name_and_fields(id, struct_info);
  entity.serialize().write();
  
  dbg!(entity);

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
pub fn id(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

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
