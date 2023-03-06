use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::count,
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};

use crate::structure::{AccessFlags, ClassFile, ConstantPoolInfo, Version};

type Input<'a> = &'a [u8];

fn parse_index(bytes: Input) -> IResult<Input, usize> {
    map(be_u16, |u: u16| u.into())(bytes)
}

fn parse_magic(bytes: Input) -> IResult<Input, &[u8]> {
    tag([0xCA, 0xFE, 0xBA, 0xBE])(bytes)
}

fn parse_version(bytes: Input) -> IResult<Input, Version> {
    let (rest, minor) = be_u16(bytes)?;
    let (rest, major) = be_u16(rest)?;

    Ok((rest, Version { major, minor }))
}

fn parse_class_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, name_index) = parse_index(bytes)?;

    Ok((rest, ConstantPoolInfo::Class(name_index)))
}

fn parse_fieldref_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, class_index) = parse_index(bytes)?;
    let (rest, name_and_type_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::FieldRef(class_index, name_and_type_index),
    ))
}

fn parse_methodref_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, class_index) = parse_index(bytes)?;
    let (rest, name_and_type_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::MethodRef(class_index, name_and_type_index),
    ))
}

fn parse_interfacemethodref_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, class_index) = parse_index(bytes)?;
    let (rest, name_and_type_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::InterfaceMethodRef(class_index, name_and_type_index),
    ))
}

fn parse_string_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, string_index) = parse_index(bytes)?;

    Ok((rest, ConstantPoolInfo::String(string_index)))
}

fn parse_integer_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, byte_count) = be_u32(bytes)?;

    Ok((rest, ConstantPoolInfo::Integer(byte_count)))
}

fn parse_float_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, byte_count) = be_u32(bytes)?;

    Ok((rest, ConstantPoolInfo::Float(byte_count)))
}

fn parse_long_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, high_bytes) = be_u32(bytes)?;
    let (rest, low_bytes) = be_u32(rest)?;

    Ok((rest, ConstantPoolInfo::Long(high_bytes, low_bytes)))
}

fn parse_double_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, high_bytes) = be_u32(bytes)?;
    let (rest, low_bytes) = be_u32(rest)?;

    Ok((rest, ConstantPoolInfo::Double(high_bytes, low_bytes)))
}

fn parse_nameandtype_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, name_index) = parse_index(bytes)?;
    let (rest, descriptor_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::NameAndType(name_index, descriptor_index),
    ))
}

fn parse_utf_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, length) = be_u16(bytes)?;
    let (rest, utf_bytes) = count(be_u8, length.into())(rest)?;

    Ok((rest, ConstantPoolInfo::Utf(length, utf_bytes)))
}

fn parse_method_handle_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, reference_kind) = be_u8(bytes)?;
    let (rest, reference_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::MethodHandle(reference_kind, reference_index),
    ))
}

fn parse_method_type_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, descriptor_index) = parse_index(bytes)?;

    Ok((rest, ConstantPoolInfo::MethodType(descriptor_index)))
}

fn parse_dynamic_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, bootstrap_method_attr_index) = parse_index(bytes)?;
    let (rest, name_and_type_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::Dynamic(bootstrap_method_attr_index, name_and_type_index),
    ))
}

fn parse_invoke_dynamic_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, bootstrap_method_attr_index) = parse_index(bytes)?;
    let (rest, name_and_type_index) = parse_index(rest)?;

    Ok((
        rest,
        ConstantPoolInfo::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index),
    ))
}

fn parse_module_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, name_index) = parse_index(bytes)?;

    Ok((rest, ConstantPoolInfo::Module(name_index)))
}

fn parse_package_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, name_index) = parse_index(bytes)?;

    Ok((rest, ConstantPoolInfo::Package(name_index)))
}

fn parse_constant_pool_info(bytes: Input) -> IResult<Input, ConstantPoolInfo> {
    let (rest, tag) = be_u8(bytes)?;

    match tag {
        1 => parse_utf_info(rest),
        3 => parse_integer_info(rest),
        4 => parse_float_info(rest),
        5 => parse_long_info(rest),
        6 => parse_double_info(rest),
        7 => parse_class_info(rest),
        8 => parse_string_info(rest),
        9 => parse_fieldref_info(rest),
        10 => parse_methodref_info(rest),
        11 => parse_interfacemethodref_info(rest),
        12 => parse_nameandtype_info(rest),
        15 => parse_method_handle_info(rest),
        16 => parse_method_type_info(rest),
        17 => parse_dynamic_info(rest),
        18 => parse_invoke_dynamic_info(rest),
        19 => parse_module_info(rest),
        20 => parse_package_info(rest),
        _ => unreachable!(),
    }
}

fn parse_constant_pool(bytes: Input) -> IResult<Input, Vec<ConstantPoolInfo>> {
    let (rest, cp_count) = be_u16(bytes)?;

    count(parse_constant_pool_info, (cp_count - 1).into())(rest)
}

fn parse_flags(bytes: Input) -> IResult<Input, AccessFlags> {
    map(be_u16, |b: u16| match AccessFlags::from_bits(b) {
        Some(flags) => flags,
        None => panic!("Invalid bytes for access flags"),
    })(bytes)
}

fn parse_interfaces(bytes: Input) -> IResult<Input, Vec<usize>> {
    let (rest, interface_count) = be_u16(bytes)?;

    count(parse_index, interface_count.into())(rest)
}

pub fn parse_classfile(bytes: Input) -> IResult<Input, ClassFile> {
    let (rest, _) = parse_magic(bytes)?;
    let (rest, version) = parse_version(rest)?;
    let (rest, constant_pool) = parse_constant_pool(rest)?;
    let (rest, access_flags) = parse_flags(rest)?;
    let (rest, this_class) = parse_index(rest)?;
    let (rest, super_class) = parse_index(rest)?;
    let (rest, interfaces) = parse_interfaces(rest)?;

    Ok((
        rest,
        ClassFile {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
        },
    ))
}
