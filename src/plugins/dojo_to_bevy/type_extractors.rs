use dojo_types::schema::Member;

pub fn member_to_u8(member: &Member) -> u8 {
    member.ty.as_primitive().unwrap().as_u8().unwrap().clone()
}

pub fn member_to_u32(member: &Member) -> u32 {
    member.ty.as_primitive().unwrap().as_u32().unwrap().clone()
}

pub fn member_to_enum_to_u8(member: &Member) -> u8 {
    member.ty.as_enum().unwrap().option.unwrap().clone()
}
