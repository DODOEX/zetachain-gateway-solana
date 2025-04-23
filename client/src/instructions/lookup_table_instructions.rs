use anchor_client::solana_sdk::{
    address_lookup_table::{
        instruction::{create_lookup_table, extend_lookup_table},
        state::AddressLookupTable,
    },
    instruction::Instruction,
    pubkey::Pubkey,
    slot_hashes::Slot,
};
use anyhow::Result;

/// 创建 Address Lookup Table
pub fn create_lookup_table_instr(
    authority: Pubkey,
    payer: Pubkey,
    recent_slot: Slot,
) -> Result<(Instruction, Pubkey)> {
    let (instruction, lookup_table_address) = create_lookup_table(authority, payer, recent_slot);

    Ok((instruction, lookup_table_address))
}

/// 扩展 Address Lookup Table
pub fn extend_lookup_table_instr(
    lookup_table_address: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    addresses: Vec<Pubkey>,
) -> Result<Instruction> {
    let instruction = extend_lookup_table(lookup_table_address, authority, Some(payer), addresses);

    Ok(instruction)
}

/// 反序列化 Lookup Table 数据
pub fn deserialize_lookup_table(data: &[u8]) -> Result<AddressLookupTable> {
    let lookup_table = AddressLookupTable::deserialize(data)?;
    Ok(lookup_table)
}
