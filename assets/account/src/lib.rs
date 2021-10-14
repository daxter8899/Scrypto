use scrypto::prelude::*;

blueprint! {
    struct Account {
        vaults: LazyMap<Address, Vault>,
    }

    impl Account {
        pub fn new() -> Component {
            Account {
                vaults: LazyMap::new(),
            }
            .instantiate()
        }

        pub fn with_bucket(bucket: Bucket) -> Component {
            let mut account = Account {
                vaults: LazyMap::new()
            };
            account.deposit(bucket);
            account.instantiate()
        }

        /// Deposit a batch of buckets into this account
        pub fn deposit_batch(&mut self, buckets: Vec<Bucket>) {
            for bucket in buckets {
                self.deposit(bucket);
            }
        }

        /// Deposits resource into this account.
        pub fn deposit(&mut self, bucket: Bucket) {
            let address = bucket.resource_def().address();
            match self.vaults.get(&address) {
                Some(v) => {
                    v.put(bucket);
                }
                None => {
                    let v = Vault::with_bucket(bucket);
                    self.vaults.insert(address, v);
                }
            }
        }

        /// Withdraws resource from this account.
        pub fn withdraw(&mut self, amount: Amount, resource_def: Address) -> Bucket {
            let vault = self
                .vaults
                .get(&resource_def);
            match vault {
                Some(vault) => vault.take(amount),
                None => {
                    error!("Resource not found in account: amount = {}, resource_def = {}", amount, resource_def);
                    panic!();
                }
            }
        }
    }
}