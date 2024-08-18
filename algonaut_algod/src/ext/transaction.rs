use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultisigSubsig {
    #[serde(rename = "pk")]
    pub key: Option<String>,
    #[serde(rename = "s")]
    pub signature: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiSig {
    #[serde(rename = "v")]
    pub version: Option<u8>,
    #[serde(rename = "thr")]
    pub threshold: Option<u8>,
    #[serde(rename = "subsig")]
    pub subsignatures: Option<Vec<MultisigSubsig>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogicSig {
    #[serde(rename = "l")]
    pub logic: Option<String>,
    #[serde(rename = "sig")]
    pub signature: Option<String>,
    #[serde(rename = "msig")]
    pub multisig: Option<MultiSig>,
    #[serde(rename = "arg")]
    pub arguments: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionHeader {
    pub hgi: Option<bool>,
    pub sig: Option<String>,
    pub msig: Option<MultiSig>,
    pub lsig: Option<LogicSig>,

    #[serde(flatten)]
    pub apply_data: Option<ApplyData>,
    pub txn: Transaction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "pay")]
    Payment(PaymentTransaction),
    #[serde(rename = "keyreg")]
    KeyRegistration(KeyRegistrationTransaction),
    #[serde(rename = "acfg")]
    AssetConfig(AssetConfigTransaction),
    #[serde(rename = "axfer")]
    AssetTransfer(AssetTransferTransaction),
    #[serde(rename = "afrz")]
    AssetFreeze(AssetFreezeTransaction),
    #[serde(rename = "appl")]
    ApplicationCall(ApplicationCallTransaction),
    #[serde(rename = "stpf")]
    StateProof(StateProofTransaction)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AssetParams {
    #[serde(rename = "am")]
    pub meta_data_hash: Option<String>,
    #[serde(rename = "an")]
    pub asset_name: Option<String>,
    #[serde(rename = "au")]
    pub url: Option<String>,
    #[serde(rename = "c")]
    pub clawback: Option<String>,
    #[serde(rename = "dc")]
    pub decimals: Option<u32>,
    #[serde(rename = "df")]
    pub default_frozen: Option<bool>,
    #[serde(rename = "f")]
    pub freeze: Option<String>,
    #[serde(rename = "m")]
    pub manager: Option<String>,
    #[serde(rename = "r")]
    pub reserve: Option<String>,
    #[serde(rename = "t")]
    pub total: Option<u64>,
    #[serde(rename = "un")]
    pub unit_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "rcv")]
    pub receiver: Option<String>,
    #[serde(rename = "amt")]
    pub amount: Option<u64>,
    #[serde(rename = "close")]
    pub close_remainder_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyRegistrationTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "votekey")]
    pub vote_pk: Option<String>,
    #[serde(rename = "selkey")]
    pub selection_pk: Option<String>,
    #[serde(rename = "sprfkey")]
    pub state_proof_pk: Option<String>,
    #[serde(rename = "votefst")]
    pub vote_first: Option<u64>,
    #[serde(rename = "votelst")]
    pub vote_last: Option<u64>,
    #[serde(rename = "votekd")]
    pub vote_key_dilution: Option<u64>,
    #[serde(rename = "nonpart")]
    pub nonparticipating: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetConfigTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "caid")]
    pub config_asset: Option<u64>,
    #[serde(rename = "apar")]
    pub params: Option<AssetParams>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetTransferTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "xaid")]
    pub asset_xfer: Option<u64>,
    #[serde(rename = "aamt")]
    pub asset_amount: Option<u64>,
    #[serde(rename = "asnd")]
    pub asset_sender: Option<String>,
    #[serde(rename = "arcv")]
    pub asset_receiver: Option<String>,
    #[serde(rename = "close")]
    pub asset_close_remainder_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetFreezeTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "fadd")]
    pub freeze_account: Option<String>,
    #[serde(rename = "faid")]
    pub asset_id: Option<u64>,
    #[serde(rename = "ffrz")]
    pub frozen: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCallTransaction {
    #[serde(rename = "fee")]
    pub fee: Option<u64>,
    #[serde(rename = "fv")]
    pub first_valid: Option<u64>,
    #[serde(rename = "gh")]
    pub genesis_hash: Option<String>,
    #[serde(rename = "lv")]
    pub last_valid: Option<u64>,
    #[serde(rename = "snd")]
    pub sender: Option<String>,
    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,
    #[serde(rename = "grp")]
    pub group: Option<String>,
    #[serde(rename = "lx")]
    pub lease: Option<String>,
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "rekey")]
    pub rekey: Option<String>,
    // type specific fields
    #[serde(rename = "apid")]
    pub app_id: Option<u64>,
    #[serde(rename = "apan")]
    pub on_complete: Option<u64>,
    #[serde(rename = "apat")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "apap")]
    pub approval_program: Option<String>,
    #[serde(rename = "apaa")]
    pub app_arguments: Option<Vec<Option<String>>>,
    #[serde(rename = "apsu")]
    pub clear_state_program: Option<String>,
    #[serde(rename = "apfa")]
    pub foreign_apps: Option<Vec<u64>>,
    #[serde(rename = "apas")]
    pub foreign_assets: Option<Vec<u64>>,
    #[serde(rename = "apgs")]
    pub global_state_schema: Option<StateSchema>,
    #[serde(rename = "apls")]
    pub local_state_schema: Option<StateSchema>,
    #[serde(rename = "apep")]
    pub extra_program_pages: Option<u64>,
    #[serde(rename = "apbx")]
    pub boxes: Option<Vec<BoxReference>>,
}

/* STATE PROOF TRANSACTION STRUCTS */

pub type Commitment = String;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Verifier {
    #[serde(rename = "cmt")]
    commitment: Option<Commitment>,
    #[serde(rename = "lf")]
    key_lifetime: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Participant {
    #[serde(rename = "p")]
    pk: Option<Verifier>,
    #[serde(rename = "w")]
    weight: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FalconVerifier {
    #[serde(rename = "k")]
    pub falcon_public_key: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Signature {
    #[serde(rename = "s")]
    signature: Option<String>,
    #[serde(rename = "idx")]
    vector_commitment_index: Option<u64>,
    #[serde(rename = "prf")]
    proof: Option<SingleLeafProof>,
    #[serde(rename = "vkey")]
    verifying_key: Option<FalconVerifier>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct SigSlotCommit {
    #[serde(rename = "s")]
    pub signature: Option<Signature>,
    pub l: Option<u64>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Reveal {
    #[serde(rename = "s")]
    sig_slot: Option<SigSlotCommit>,
    #[serde(rename = "p")]
    participant: Option<Participant> 
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct HashFactory {
    #[serde(rename = "t")]
    hash_type: Option<u16>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Proof {
    #[serde(rename = "pth")]
    path: Option<Vec<String>>,
    #[serde(rename = "hsh")]
    hash_factory: Option<HashFactory>,
    #[serde(rename = "td")]
    tree_depth: Option<u8>,
}

type SingleLeafProof = Proof;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct StateProof {
    #[serde(rename = "c")]
    sig_commit: Option<String>,
    #[serde(rename = "w")]
    signed_weight: Option<u64>,
    #[serde(rename = "S")]
    sig_proofs: Option<Proof>,
    #[serde(rename = "P")]
    part_proofs: Option<Proof>,
    #[serde(rename = "v")]
    merkle_signature_salt_version: Option<u8>,
    #[serde(rename = "r")]
    reveals: Option<HashMap<String, Reveal>>,  // Key is expected to be u64 as string.
    #[serde(rename = "pr")]
    position_to_reveal: Option<Vec<u64>>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct StateProofMessage {
    #[serde(rename = "b")]
    pub block_headers_commitment: Option<String>,
    #[serde(rename = "v")]
    pub voters_commitment: Option<String>,
    #[serde(rename = "P")]
    pub ln_proven_weight: Option<u64>,
    #[serde(rename = "f")]
    pub first_attested_round: Option<u64>,
    #[serde(rename = "l")]
    pub last_attested_round: Option<u64>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct StateProofTransaction {
    #[serde(rename = "sptype")]
    pub state_proof_type: Option<u64>,
    #[serde(rename = "sp")]
    pub state_proof: Option<StateProof>,
    #[serde(rename = "spmsg")]
    pub message: Option<StateProofMessage>
}

/* APPLICATION CALL TRANSACTION STRUCTS */

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BoxReference {
    #[serde(rename = "n")]
    name: Option<String>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct StateSchema {
    #[serde(rename = "nui")]
    pub ints: Option<u64>,
    #[serde(rename = "nbs")]
    pub byte_slices: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum DeltaAction {
    SetBytesAction = 1,
    SetUintAction = 2,
    DeleteAction = 3,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ValueDelta {
    #[serde(rename = "at")]
    pub action: Option<DeltaAction>,
    #[serde(rename = "bs")]
    pub bytes: Option<String>,
    #[serde(rename = "ui")]
    pub uint: Option<u64>,
}

type StateDelta = HashMap<String, ValueDelta>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvalDelta {
    #[serde(rename = "gd")]
    pub global_delta: Option<StateDelta>,
    #[serde(rename = "ld")]
    pub local_deltas: Option<HashMap<String, StateDelta>>,
    #[serde(rename = "lg")]
    pub logs: Option<Vec<String>>,
    #[serde(rename = "itx")]
    pub inner_txns: Option<Vec<TransactionHeader>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplyData {
    #[serde(rename = "dt")]
    pub delta: Option<EvalDelta>,
    #[serde(rename = "ca")]
    pub closing_amount: Option<u64>,
    #[serde(rename = "aca")]
    pub asset_closing_amount: Option<u64>
}
