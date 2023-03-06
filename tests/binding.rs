use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus_backend_csharp::overloads::{DotNet, Unity};

    let config = Config {
        dll_name: "corrupt_self_demo".to_string(),
        namespace_mappings: NamespaceMappings::new("ConsoleApp"),
        ..Config::default()
    };

    Generator::new(config, corrupt_self_demo::my_inventory())
        .add_overload_writer(DotNet::new())
        //.add_overload_writer(Unity::new())
        .write_file("ConsoleApp/Interop.cs")?;

    Ok(())
}