# Example Contract

For trying the impletation this article shows, we need to import a modified version
[ink-lang][ink-lang] provied by patractlabs.

```toml
[dependencies.jupiter-ink-lang]
git = "https://github.com/patractlabs/ink"
branch = "altbn128/env"
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
