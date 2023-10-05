fn main() {
    #[cfg(target = "wasm32-unknown-unknown")]
    {
    use ic_cdk_bindgen::{Builder, Config};
    use std::path::PathBuf;
   
    const TYPE_ATTRIBUTES: &str = "#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]";
   
    let manifest_dir =
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));
    let mut ledger = Config::new("ledger");
    ledger.binding.type_attributes = TYPE_ATTRIBUTES.to_string();
    let mut builder = Builder::new();
    builder.add(ledger);
    builder.build(Some(manifest_dir.join("declarations")));
    }
   }
