use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use near_sdk::serde::{Serialize, Deserialize};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Drug {
    drug: UnorderedMap<String, Drug>
}

#[near_bindgen]
impl Drug {
    
    #[init]
    pub fn init() -> Self {
        Self {
            drug: UnorderedMap::new(b"drug".to_vec()),
        }
    }

 
    pub fn set_drug(&mut self, payload: Payload) {
        require!(self.drug.get(&payload.id).is_none(), format!("a drug with {} already exists", payload.id));
        let drug = Drug::from_payload(payload);
        self.drug.insert(&drug.id, &drug);
    }

    pub fn get_drug(self, id: &String) -> Option<Drug> {
        self.drug.get(id)
    }

  
    pub fn get_drug_list(self) -> Vec<Drug> {
        self.drug.values_as_vector().to_vec()
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Drug {
    
}

impl Drug {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            owner: env::signer_account_id()
        }
    }

   
}


