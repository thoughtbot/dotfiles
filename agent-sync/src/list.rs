use anyhow::Result;

use crate::config::Config;
use crate::library::{ItemSource, Library};

pub fn run(config: &Config) -> Result<()> {
    let library = Library::scan(config)?;
    for diagnostic in &library.diagnostics {
        eprintln!("WARN {}", diagnostic.message);
    }
    for tombstone in &library.tombstones {
        println!("tombstone\t{tombstone}");
    }
    for item in &library.items {
        let vendor = item
            .vendor_origin
            .as_ref()
            .map(|origin| format!("vendor/{origin}/"))
            .unwrap_or_default();
        let source = if item.shadows_public {
            "local-shadow"
        } else {
            match item.source {
                ItemSource::Public => "public",
                ItemSource::Local => "local",
                ItemSource::Cache => "cache",
            }
        };
        let fanout = config.fanout_name(&item.name, item.vendor_origin.as_deref());
        println!(
            "{source}\t{}/{vendor}{}\t->\t{fanout}\t{}",
            item.kind,
            item.name,
            item.path.display()
        );
    }
    Ok(())
}
