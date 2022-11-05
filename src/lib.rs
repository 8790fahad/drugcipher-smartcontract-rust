use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use near_sdk::serde::{Serialize, Deserialize};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Consultations {
    consultations: UnorderedMap<String, Consultation>
}

#[near_bindgen]
impl Consultations {
    
    #[init]
    pub fn init() -> Self {
        Self {
            consultations: UnorderedMap::new(b"consultations".to_vec()),
        }
    }

 
    pub fn set_consultation(&mut self, payload: Payload) {
        require!(self.consultations.get(&payload.id).is_none(), format!("a consultation with {} already exists", payload.id));
        let consult = Consultation::from_payload(payload);
        self.consultations.insert(&consult.id, &consult);
    }

    pub fn get_consultation(self, id: &String) -> Option<Consultation> {
        self.consultations.get(id)
    }

  
    pub fn get_consultation_history(self) -> Vec<Consultation> {
        self.consultations.values_as_vector().to_vec()
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    patient_id: String,
    user_id: String,
    consultation_notes: String,
    treatment_plan: String,
    decision: String,
    dressing_request: String,
    nursing_request: String,
    nursing_request_status: String,
    facility_id: String,
    created_at: String,
    treatment_plan_status: String,
    treated_by: String,
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Consultation {
    id: String,
    patient_id: String,
    user_id: String,
    consultation_notes: String,
    treatment_plan: String,
    decision: String,
    dressing_request: String,
    nursing_request: String,
    nursing_request_status: String,
    facility_id: String,
    created_at: String,
    treatment_plan_status: String,
    treated_by: String,
    owner: AccountId,
}

impl Consultation {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            patient_id: payload.patient_id,
            user_id: payload.user_id,
            consultation_notes: payload.consultation_notes,
            treatment_plan: payload.treatment_plan,
            decision: payload.decision,
            dressing_request: payload.dressing_request,
            nursing_request: payload.nursing_request,
            nursing_request_status: payload.nursing_request_status,
            facility_id: payload.facility_id,
            created_at: payload.created_at,
            treatment_plan_status: payload.treatment_plan_status,
            treated_by: payload.treated_by,
            owner: env::signer_account_id()
        }
    }

   
}


