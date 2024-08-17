## app directory

Here you will find the necessary files that will be used to build the contract

Directory tree of the project:

<pre>
    app
    ├── Cargo.toml <- file where dependencies are specified
    └── src <- Here you will find the contract files and directories
        ├── clients <- Directory where all contract clients are stored
        |   ├── mod.rs <- file that specifies the contract clients module
        |   └── app_client.rs <- receiver contract client, helper to send message to that contract
        ├── services <- Directory where all services are stored
        |   ├── mod.rs <- file that specifies the contract services module
        |   ├── query_service.rs <- Query service: expose all queries for contract
        |   └── receiver_service.rs <- Receiver service: main service of the contract
        ├── states <- Directory where all contract states are stored
        |   ├── mod.rs <- file that specifies the contract states module
        |   └── receiver_state.rs <- structs, enums, etc for state
        └── lib.rs <- file where the contract "program" is created
</pre>

