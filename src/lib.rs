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
        self.drug.get(id);
    }

  
    pub fn get_drug_list(self) -> Vec<Drug> {
        self.drug.values_as_vector().to_vec()
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id:String;
    manufcturer_name:String,
    sole_agent : String,
    drug_brand_name:String,
    generic_name:String,
    drug_strength:String,
    formulation_type:String,
    unit_packaging:String,
    nafdac_number:String,
    lot_number:String,
    date_manufacture:String,
    expiry_date:String,
    company_id:String,
    status:bool,
    remarks:String,
    authorize_marketers:String
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Drug {
    
}

impl Drug {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            owner: env::signer_account_id(),
            id:payload.id,
            manufcturer_name:payload.manufcturer_name,
            sole_agent : payload.sole_agent,
            drug_brand_name:payload.drug_brand_name,
            generic_name:payload.generic_name,
            drug_strength:payload.drug_strength,
            formulation_type:payload.formulation_type,
            unit_packaging:payload.unit_packaging,
            nafdac_number:payload.nafdac_number,
            lot_number:payload.lot_number,
            date_manufacture:payload.date_manufacture,
            expiry_date:payload.expiry_date,
            company_id:payload.company_id,
            status:payload.status,
            remarks:payload.remarks,
            authorize_marketers:payload.authorize_marketers,
        }
    }

   
}