use sails_client_gen::ClientGenerator;
use extended_vft_app::Program;
use std::{env, path::PathBuf};

fn main() {
    gwasm_builder::build();

    let idl_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("extended_vft.idl");

    // Generate IDL file for the app
    sails_idl_gen::generate_idl_to_file::<Program>(&idl_file_path).unwrap();

    ClientGenerator::from_idl_path(&idl_file_path)
        .with_mocks("with_mocks")
        .generate_to(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("demo_client.rs"))
        .unwrap();

    // Generate client code from IDL file
    // sails_client_gen::generate_client_from_idl(
    //     &idl_file_path,
    //     PathBuf::from(env::var("OUT_DIR").unwrap()).join("extended_vft_client.rs"),
    //     None,
    // )
    // .unwrap();
}
