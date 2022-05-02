use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use tiny_keccak::keccak256;

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction {
    /// Nonce
    pub nonce: U256,
    /// Recipient (None when contract creation)
    pub to: Option<H160>,
    /// Transfered value
    pub value: U256,
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub data: Vec<u8>,
}

impl RawTransaction {
    pub fn hash(&self, chain_id: u8) -> Vec<u8> {
        let mut hash = RlpStream::new();
        hash.begin_unbounded_list();
        self.encode(&mut hash);
        hash.append(&mut vec![chain_id]);
        hash.append(&mut U256::zero());
        hash.append(&mut U256::zero());
        hash.complete_unbounded_list();
        keccak256_hash(&hash.out())
    }

    pub fn encode(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let Some(ref t) = self.to {
            s.append(t);
        } else {
            s.append(&vec![]);
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

fn keccak256_hash(bytes: &[u8]) -> Vec<u8> {
    keccak256(bytes).into_iter().cloned().collect()
}

pub struct EcdsaSig {
    v: Vec<u8>,
    r: Vec<u8>,
    s: Vec<u8>,
}