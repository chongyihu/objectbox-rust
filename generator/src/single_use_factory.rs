
fn generate_trait_impls() -> Tokens<Rust> {

  let types = vec![
      "i64",
      "f64",
      "String",
    //   "bool",
    //   "char",
      "Vec<u8>",
      "Vec<String>",
  ];

  let mut out = Tokens::<Rust>::new();

  for tt in types.as_slice() {
      let t = *tt;
      let t_nb = t.to_string().replace('<', "").replace('>', "").to_lowercase();
      let safe_t = t_nb.as_str();
      let eq_impl: Tokens<Rust> = quote! {
        fn Eq_$(safe_t)(_:$t) {}
        fn Ne_$(safe_t)(_:$t) {}
          impl<Entity: OBBlanket> Eq<Entity, $(t)> for ConditionBuilder<Entity> {
              fn eq(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new_group(Eq_$(safe_t)(other))
              }

              fn ne(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new_group(Ne_$(safe_t)(other))
              }
          }
      };

      let ord_impl: Tokens<Rust> = quote! {
        fn Lt_$(safe_t)(_:$t) {}
        fn Gt_$(safe_t)(_:$t) {}
        fn Le_$(safe_t)(_:$t) {}
        fn Ge_$(safe_t)(_:$t) {}
          impl<Entity: OBBlanket> Ord<Entity, $(t)> for ConditionBuilder<Entity> {
              fn lt(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), Lt_$(safe_t)(other))
              }

              fn gt(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), Gt_$(safe_t)(other))
              }

              fn le(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), Le_$(safe_t)(other))
              }

              fn ge(&self, other: $(t)) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), Ge_$(safe_t)(other))
              }
          }

      };
      out.extend(eq_impl);
      out.extend(ord_impl);
}

for tt in vec!["i64", "f64"].as_slice() {
    let t = *tt;
      let between_impl: Tokens<Rust> = quote! {
        fn Between_$t(_:$t, _:$t) {}
          impl<Entity: OBBlanket> BetweenExt<Entity, $(t)> for ConditionBuilder<Entity> {
              fn between(&self, this: $(t), that: $(t)) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), Between_$(t)(this, that))
              }
          }

      };

        out.append(between_impl);
}

for tt in vec!["i32", "i64", "String"].as_slice() {
    let t = *tt;
      let inout_impl: Tokens<Rust> = quote! {
        fn In_$t(_:$t) {}
        fn NotIn_$t(_:$t) {}
          impl<Entity: OBBlanket> InOutExt<Entity, $(t)> for ConditionBuilder<Entity> {
              fn member_of(&self, vec: Vec<$(t)>) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), In_$(t)(vec))
              }

              fn not_member_of(&self, vec: Vec<$(t)>) -> Condition<Entity> {
                  Condition::new(self.get_parameters(), NotIn_$(t)(vec))
              }
          }
      };

            out.append(inout_impl);
    }

    


  out
}
