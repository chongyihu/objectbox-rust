extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate prettyplease;
extern crate maplit;
extern crate objectbox_generator;

use objectbox_generator::id;
use proc_macro::TokenStream;

mod debug;
mod property;
mod entity;
mod path_visitor;

use syn::{DeriveInput, parse_macro_input, AttributeArgs};
use syn::Meta::NameValue;
use entity::Entity;

// extension trait for IdUid, reuse structs
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
  
  // dbg!(entity);

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

// TODO figure out how to silence the macro checker
// that it isn't a derive macro on top of a type
// #[proc_macro_derive(Entity)]


#[proc_macro_attribute]
pub fn sync(_attribute: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Fields

/// Accepts 'type' parameter
/// Note: indexes are currently not supported for ByteVector, Float or Double
/// See ./objectbox/lib/src/annotations.dart implementation
/// All ids are self-assignable, since there are no write-once / const fields.
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
