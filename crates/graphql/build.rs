fn main() {
    cynic_codegen::register_schema("flymodel")
        .from_sdl_file("schema/flymodel.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
