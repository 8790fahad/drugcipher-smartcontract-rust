use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use near_sdk::serde::{Serialize, Deserialize};
// Promise

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DrugCipher {
    drug_listed: UnorderedMap<String, DrugInfo>
}

#[near_bindgen]
impl DrugCipher {
    
    #[init]
    pub fn init() -> Self {
        Self {
            drug_listed: UnorderedMap::new(b"drug_listed".to_vec()),
        }
    }
    #[payable]
    pub fn set_drug_info(&mut self, payload: Payload,token: &String,beneficiary_id: &String) {
        assert_eq! (env::attached_deposit(), token.trim().parse().unwrap(), "Attached deposit should be equal to the token");
        let drug_info = DrugInfo::from_payload(payload);
        Promise::new(beneficiary_id.parse().unwrap()).transfer(token.trim().parse().unwrap());
        self.drug_listed.insert(&drug_info.id, &drug_info);
    }

    // get the drugs
    pub fn get_drugs_info(&self) -> Vec<DrugInfo> {
        self.drug_listed.values_as_vector().to_vec()
    }

    // recall drug incase of any problem with the drug
    pub fn recall_drug_fun(&mut self, drug_id: &String,remark: &String) {
    match self.drug_listed.get(drug_id) {
        Some(ref mut drug) => {
            drug.recall_drug(remark.to_string());
            self.drug_listed.insert(&drug.id, &drug);
        },
        _ => {
            env::panic_str("Drug info not found");
        }
    }
    }
    pub fn view_drug_info(&self, id: &String) -> Option<DrugInfo> {
        // let amount: u128 = 1000000000000000000000;
        // let account_id: AccountId = beneficiary_id.parse().unwrap();
        // DrugInfo::pay_token(near_sdk::json_types::U128(amount),account_id);
        self.drug_listed.get(id)

    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id:String,
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
    status:u32,
    remark:String,
    authorize_marketers:String
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct DrugInfo {
    owner: AccountId,
    id:String,
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
    status:u32,
    remark:String,
    authorize_marketers:String,
}

#[near_bindgen]
impl DrugInfo {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
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
            remark:payload.remark,
            authorize_marketers:payload.authorize_marketers,
            owner: env::signer_account_id()
        }
    }
    // pub fn pay_token(amount: U128, to: AccountId) -> Promise {
    //     Promise::new(to).transfer(amount.0)
    // }
    pub fn recall_drug(&mut self,remark: String) {
        self.status = 1;
        self.remark = remark;
    }
}