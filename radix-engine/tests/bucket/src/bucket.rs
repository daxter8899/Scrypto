use scrypto::prelude::*;

blueprint! {
    struct BucketTest {
        vault: Vault,
    }

    impl BucketTest {
        fn create_test_token(amount: u32) -> Bucket {
            let bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .auth("take_from_vault", auth!(allow_all))
                .metadata("name", "TestToken")
                .initial_supply(amount);
            let proof1 = bucket.create_proof();
            let proof2 = proof1.clone();
            proof1.drop();
            proof2.drop();
            bucket
        }

        pub fn combine() -> Bucket {
            let mut bucket1 = Self::create_test_token(100);
            let bucket2 = bucket1.take(50);

            bucket1.put(bucket2);
            bucket1
        }

        pub fn split() -> (Bucket, Bucket) {
            let mut bucket1 = Self::create_test_token(100);
            let bucket2 = bucket1.take(Decimal::from(5));
            (bucket1, bucket2)
        }

        pub fn borrow() -> Bucket {
            let bucket = Self::create_test_token(100);
            let proof = bucket.create_proof();
            proof.drop();
            bucket
        }

        pub fn query() -> (Decimal, ResourceDefId, Bucket) {
            let bucket = Self::create_test_token(100);
            (bucket.amount(), bucket.resource_def_id(), bucket)
        }

        pub fn test_restricted_transfer() -> Vec<Bucket> {
            let auth_bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .auth("take_from_vault", auth!(allow_all))
                .initial_supply(1);
            let bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .auth(
                    "take_from_vault",
                    auth!(require(auth_bucket.resource_def_id())),
                )
                .initial_supply(5);
            let mut vault = Vault::with_bucket(bucket);

            let token_bucket = auth_bucket.authorize(|| vault.take(1));

            BucketTest { vault }.instantiate().globalize();
            vec![auth_bucket, token_bucket]
        }

        pub fn test_burn() -> Vec<Bucket> {
            let badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .auth("take_from_vault", auth!(allow_all))
                .initial_supply(1);
            let bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .auth("take_from_vault", auth!(allow_all))
                .auth("burn", auth!(require(badge.resource_def_id())))
                .initial_supply(5);
            badge.authorize(|| bucket.burn());
            vec![badge]
        }

        pub fn test_burn_freely() -> Vec<Bucket> {
            let badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .auth("take_from_vault", auth!(allow_all))
                .initial_supply(1);
            let mut bucket1 = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .auth("take_from_vault", auth!(allow_all))
                .auth("burn", auth!(allow_all))
                .initial_supply(5);
            let bucket2 = bucket1.take(2);
            badge.authorize(|| bucket1.burn());
            bucket2.burn();
            vec![badge]
        }

        pub fn take_from_bucket(mut bucket: Bucket, amount: Decimal) -> (Bucket, Bucket) {
            let x = bucket.take(amount);
            (bucket, x)
        }
    }
}
