pub trait PrintTokenStream {
    fn print_token_stream(&self, label: &str);
}

impl PrintTokenStream for syn::Field {
    fn print_token_stream(&self, label: &str) {
        let field_ts = quote::ToTokens::to_token_stream(self);
        let debug_label = format!("field({}) token_stream", label);
        field_ts.print_token_stream(&debug_label);
    }
}

// Only available in Debug mode
impl PrintTokenStream for proc_macro::TokenStream {
    fn print_token_stream(&self, label: &str) {
        if cfg!(debug_assertions) {
            let str = self.to_string();
            println!("{} {:#?}", label, str.as_str());
        }
    }
}

impl PrintTokenStream for proc_macro2::TokenStream {
    fn print_token_stream(&self, label: &str) {
        if cfg!(debug_assertions) {
            let str = self.to_string();
            println!("{} {:#?}", label, str.as_str());
        }
    }
}
