# Pallet Contracts

Here we did some modifications on [paritytech/substrate][paritytech/substrate], added the
curves mentioned in last chapters into `runtime_interfaces`, for more detail, check the 
[pallet-contracts][pallet-contracts].

```toml
[dependencies.pallet-contracts]
git = "https://github.com/patractlabs/substrate"
branch = "patract-contracts"
```

We also modified [ink!][ink-lang] to adapt the new interfaces we provided,
all of these modifications will test on [patractlabs/jupiter][patractlabs/jupiter] in the 
future, and will pr to the offical repos if they are accepted by the Polkadot Ecology.


```toml
[dependencies.jupiter-ink-lang]
git = "https://github.com/patractlabs/ink"
branch = "megaclite"
```

## Example

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use jupiter_ink_lang as ink;

#[ink::contract]
mod altbn128 {
    use ink_env::zk_snarks::AltBn128;
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct Altbn128 {
        value: String,
    }

    impl Altbn128 {
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new("hello, world".into())
        }

        #[ink(message)]
        pub fn bn_256_add(&mut self) {
            let mut result = [0; 64];
            ink_env::inflect_add::<AltBn128>(&[], &[], &mut result);
            self.value = ink_prelude::format!("0x{:x?}", result);
        }

        #[ink(message)]
        pub fn get(&self) -> String {
            ink_prelude::format!("{}", &self.value)
        }
    }
}
```

[ink-lang]: https://github.com/patractlabs/ink
[paritytech/substrate]: https://github.com/paritytech/substrate.git
[patractlabs/jupiter]: https://github.com/paritytech/jupiter.git
[pallet-contracts]: https://github.com/patractlabs/substrate/tree/patract-contracts/frame/contracts
