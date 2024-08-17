# Transmiter contract

When compiling (inside the wasm folder or in the root path), two files will be created, "app.idl" which specifies the types, services, etc; and "app_client.rs (contains all the necessary code) that helps to send messages to this contract

To upload the contract, you have to go to [IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network) and upload the .opt.wasm (in target/wasm32--https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network/release) and idl files that were generated.

You can find the explanation of clients in "receiver contract"!

## Use of custom state while using clients

As explained in the receiver contract, clients are stored in the state in order to save a little with tokens, and the required data is also stored here! You need to create another module (For better maintenance, you can manage it as you like) as it is in this contract.

The directory tree would look like this:

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

In the transmiter_state.rs fie inside states directory you will find the struct of the state of the transmiter service. It is used to manage the receiver id and the owner of the contract.




## Service structure:

In the service, you need to add the state and the clients, as in the example:

```rust
pub struct TransmitterService<'a, ReceiverClient, ReceiverQueryClient> {
    pub state: RefMut<'a, TransmitterState>,
    pub receiver_contract_service: RefMut<'a, ReceiverClient>,
    pub receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
}

#[service]
impl<'a, ReceiverClient, ReceiverQueryClient> TransmitterService<'a, ReceiverClient, ReceiverQueryClient>
where 
    ReceiverClient: Receiver,
    ReceiverQueryClient: ReceiverQuery
{
    pub fn new(
        state: RefMut<'a, TransmitterState>, 
        receiver_contract_service: RefMut<'a, ReceiverClient>,
        receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
    ) -> Self {
        Self {
            state,
            receiver_contract_service,
            receiver_conotract_query
        }
    }

    // Command method using the state
    pub fn set_receiver_contract_id(&mut self, receiver_id: ActorId) -> TransmitterEvents {
        // self.state.insert(receiver_id);
        let _ = self
            .state
            .receiver_id
            .insert(receiver_id);

        TransmitterEvents::ReceiverContractIdChanged
    }
}

// ... The missing part is within the contract files
```

It is exactly the same as customers, and the lifetime ('a) is shared for all attributes




## Program structure

In the program (that is in "lib.rs"), yo need to add another field to the program struct and set the "initial" value, and finally pass it to the service:

```rust
// import of the state struct
use states::transmiter_state::TransmitterState;

// program struct
pub struct TransmitterProgram {
    pub receiver_contract_id: RefCell<TransmitterState>,
    pub receiver_client: RefCell<ReceiverClient<GStdRemoting>>,
    pub receiver_query_client: RefCell<ReceiverQueryClient<GStdRemoting>>
}

#[program]
impl TransmitterProgram {
    pub fn new() -> Self {
        let transmiter_state = RefCell::new(
            TransmitterState {
                receiver_id: None,
                owner: msg::source()
            }
        );
        let receiver_client = RefCell::new(
            ReceiverClient::new(GStdRemoting)
        );
        let receiver_query_client = RefCell::new(
            ReceiverQueryClient::new(GStdRemoting)
        );

        Self {
            transmiter_state,
            receiver_client,
            receiver_query_client
        }
    }

    #[route("Transmitter")]
    pub fn transmitter_svc(&self) -> TransmitterService<'_, ReceiverClient<GStdRemoting>, ReceiverQueryClient<GStdRemoting>> {
        TransmitterService::new(
            self.transmiter_state.borrow_mut(), 
            self.receiver_client.borrow_mut(), 
            self.receiver_query_client.borrow_mut()
        )
    }
}
```

