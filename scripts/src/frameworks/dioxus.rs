use std::{error::Error, path::PathBuf};

use proc_macro2::TokenStream;

use crate::framework::Framework;

#[allow(dead_code)]
pub struct Dioxus;

impl Framework for Dioxus {
    fn name(&self) -> &'static str {
        "dioxus"
    }

    fn lib_header(&self) -> Option<String> {
        Some(
            "\
            //! Dioxus port of [Lucide](https://lucide.dev/).\n\
            //!\n\
            //! Lucide is a beautiful & consistent icon toolkit made by the community.\n\
            //!\n\
            //! See [the Rust Lucide book](https://lucide.rustforweb.org/dioxus.html) for more documenation.\n\
            "
            .to_owned()
        )
    }

    fn generate(
        &self,
        _component_name: String,
        _svg: String,
    ) -> Result<TokenStream, Box<dyn Error>> {
        // TODO
        todo!()
    }

    fn generate_example(&self, _component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
        // TODO
        todo!()
    }

    fn format(&self, _package: String, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
