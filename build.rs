extern crate protoc_rust;

use protoc_rust::Customize;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/gen/",
	    input: &["proto/animal.proto"],
	    includes: &["proto/"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("protoc");
    let mut f = File::create("src/gen/mod.rs").unwrap();
    f.write_all(b"// Generated by build.rs\npub mod animal;\n").unwrap();
}
