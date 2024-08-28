use dojo_types::schema::Member;
use starknet_crypto::Felt;

pub fn member_to_bool(member: &Member) -> bool {
    member.ty.as_primitive().unwrap().as_bool().unwrap().clone()
}

pub fn member_to_u8(member: &Member) -> u8 {
    member.ty.as_primitive().unwrap().as_u8().unwrap().clone()
}

pub fn member_to_u16(member: &Member) -> u16 {
    member.ty.as_primitive().unwrap().as_u16().unwrap().clone()
}

pub fn member_to_u32(member: &Member) -> u32 {
    member.ty.as_primitive().unwrap().as_u32().unwrap().clone()
}

pub fn member_to_enum_to_u8(member: &Member) -> u8 {
    member.ty.as_enum().unwrap().option.unwrap().clone()
}

pub fn member_to_contract_address_to_felt(member: &Member) -> Felt {
    member
        .ty
        .as_primitive()
        .unwrap()
        .as_contract_address()
        .unwrap()
        .clone()
}
