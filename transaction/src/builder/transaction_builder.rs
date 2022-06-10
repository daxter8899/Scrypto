use scrypto::{buffer::scrypto_encode, crypto::*};

use crate::{model::*, signing::Signer};

pub struct TransactionBuilder {
    manifest: Option<TransactionManifest>,
    header: Option<TransactionHeader>,
    intent_signatures: Vec<(EcdsaPublicKey, EcdsaSignature)>,
    notary_signature: Option<EcdsaSignature>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            manifest: None,
            header: None,
            intent_signatures: Vec::new(),
            notary_signature: None,
        }
    }

    pub fn manifest(mut self, manifest: TransactionManifest) -> Self {
        self.manifest = Some(manifest);
        self
    }

    pub fn header(mut self, header: TransactionHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn sign<S: Signer>(mut self, signer: &S) -> Self {
        let intent = self.transaction_intent();
        let intent_payload = scrypto_encode(&intent);
        self.intent_signatures
            .push((signer.public_key(), signer.sign(&intent_payload)));
        self
    }

    pub fn notarize<S: Signer>(mut self, signer: &S) -> Self {
        let signed_intent = self.signed_transaction_intent();
        let signed_intent_payload = scrypto_encode(&signed_intent);
        self.notary_signature = Some(signer.sign(&signed_intent_payload));
        self
    }

    pub fn build(&self) -> NotarizedTransaction {
        NotarizedTransaction {
            signed_intent: self.signed_transaction_intent(),
            notary_signature: self.notary_signature.clone().expect("Not notarized"),
        }
    }

    fn transaction_intent(&self) -> TransactionIntent {
        TransactionIntent {
            manifest: self.manifest.clone().expect("Manifest not specified"),
            header: self.header.clone().expect("Header not specified"),
        }
    }

    fn signed_transaction_intent(&self) -> SignedTransactionIntent {
        let intent = self.transaction_intent();
        SignedTransactionIntent {
            intent,
            intent_signatures: self.intent_signatures.clone(),
        }
    }
}