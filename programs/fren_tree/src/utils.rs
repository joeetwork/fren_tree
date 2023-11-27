use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct InitializeUserParams {
    pub twitter: String, 
    pub role: String
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct AddUniqueNameProps {
    pub username: String
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct CheckRoleProps {
    pub role: String
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct UpgradeUserProps {
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct CheckUpgradeProps {
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct SendRequestProps {
    pub to: Pubkey
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct AcceptRequestProps {
    pub request_id: u8
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct DeclineRequestProps {
    pub request_id: u8
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct RemoveConnectionProps {
    pub connection_id: u8
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct InitializeTopConnectionsProps {

}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct AddTopConnectionsProps {
    pub connection: i32,
    pub position: u8, 
    pub role: String
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Default)]
pub struct RemoveTopConnectionsProps {
    pub position: u8, 
    pub role: String
}