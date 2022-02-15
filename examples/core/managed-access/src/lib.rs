use scrypto::prelude::*;

import! {
r#"
{
    "package": "01ca59a8d6ea4f7efa1765cef702d14e47570c079aedd44992dd09",
    "name": "FlatAdmin",
    "functions": [
        {
            "name": "instantiate_flat_admin",
            "inputs": [
                {
                    "type": "String"
                }
            ],
            "output": {
                "type": "Tuple",
                "elements": [
                    {
                        "type": "Custom",
                        "name": "ComponentRef",
                        "generics": []
                    },
                    {
                        "type": "Custom",
                        "name": "Bucket",
                        "generics": []
                    }
                ]
            }
        }
    ],
    "methods": [
        {
            "name": "create_additional_admin",
            "mutability": "Immutable",
            "inputs": [
                {
                    "type": "Custom",
                    "name": "BucketRef",
                    "generics": []
                }
            ],
            "output": {
                "type": "Custom",
                "name": "Bucket",
                "generics": []
            }
        },
        {
            "name": "destroy_admin_badge",
            "mutability": "Immutable",
            "inputs": [
                {
                    "type": "Custom",
                    "name": "Bucket",
                    "generics": []
                }
            ],
            "output": {
                "type": "Unit"
            }
        },
        {
            "name": "get_admin_badge",
            "mutability": "Immutable",
            "inputs": [],
            "output": {
                "type": "Custom",
                "name": "ResourceDefRef",
                "generics": []
            }
        }
    ]
}
"#
}

blueprint! {
    struct ManagedAccess {
        admin_badge: ResourceDefRef,
        flat_admin_controller: ComponentRef,
        protected_vault: Vault,
    }

    impl ManagedAccess {
        pub fn instantiate_managed_access() -> (ComponentRef, Bucket) {
            let (flat_admin_component, admin_badge) =
                FlatAdmin::instantiate_flat_admin("My Managed Access Badge".into());

            let component = Self {
                admin_badge: admin_badge.resource_def_ref(),
                flat_admin_controller: flat_admin_component,
                protected_vault: Vault::new(RADIX_TOKEN),
            }
            .instantiate();
            (component, admin_badge)
        }

        #[auth(admin_badge)]
        pub fn withdraw_all(&mut self) -> Bucket {
            self.protected_vault.take_all()
        }

        pub fn deposit(&mut self, to_deposit: Bucket) {
            self.protected_vault.put(to_deposit);
        }

        pub fn get_admin_badge(&self) -> ResourceDefRef {
            self.admin_badge
        }

        pub fn get_flat_admin_controller(&self) -> ComponentRef {
            self.flat_admin_controller
        }
    }
}
