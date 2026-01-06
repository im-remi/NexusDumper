use std::{
    fs::File,
    io::{self, Read, Write},
};

use il2cpp::vm::*;

use crate::util::write_escaped_str;
use types::*;

mod util;
mod types;

pub static GLOBAL_METADATA: usize = 0x545E998;
pub static TYPE_GET_INFO: usize = 0x15E3D00;
pub static METHOD_REF_GET_INFO: usize = 0x15E1F80;
pub static STRING_LITERAL_SIZE: u32 = 27987;
pub static TYPE_INFO_SIZE: u32 = 61205;
pub static METHOD_REF_SIZE: u32 = 171787;

pub unsafe fn create_script_json<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "{{\n")?;

    let (type_info, method_refs, string_literals, domain_methods) = parse_gameassembly()?;
    
    writeln!(out, "    \"ScriptString\": ")?;
    let mut is_first = true;
    writeln!(out, "        [")?;
    for literal in string_literals.iter() {
        if !is_first {
            writeln!(out, ",")?;
        }
        write!(
            out,
            "           {{\n                \"Address\":{},\"Value\":\"",
            literal.address
        )?;
        write_escaped_str(out, &literal.value)?;
        write!(out, "\"\n           }}")?;

        is_first = false;
    }
    writeln!(out, "\n        ],")?;
    writeln!(out, "    \"ScriptMetadata\": [")?;
    let mut is_first = true;
    for r#type in type_info.iter() {
        if !is_first {
            writeln!(out, ",")?;
        }
        write!(
            out,
            "           {{\n                \"Address\":{}, \"Name\":\"{}_TypeInfo\", \"Signature\": \"{}_c\"\n           }}",
            r#type.address,
            r#type.name,
            r#type.name
        )?;

        is_first = false;
    }
    writeln!(out, "\n        ],")?;
    writeln!(out, "    \"ScriptMetadataMethod\": [")?;
    let mut is_first = true;
    for method in method_refs.iter() {
        if !is_first {
            writeln!(out, ",")?;
        }
        write!(
            out,
            "           {{\n                \"Address\":{}, \"Name\":\"{}_{}\", \"MethodAddress\": {}\n           }}",
            method.address,
            method.type_name,
            method.name,
            method.method_address
        )?;

        is_first = false;
    }
    writeln!(out, "\n        ],")?;
    writeln!(out, "    \"ScriptMethod\": [")?;
    let mut is_first = true;
    for method in domain_methods.iter() {
        if !is_first {
            writeln!(out, ",")?;
        }
        write!(
            out,
            "           {{\n                \"Address\":{}, \"Name\":\"{}_{}\", \"Signature\": \"\", \"TypeSignature\": \"\"\n           }}",
            method.address,
            method.type_name,
            method.name
        )?;

        is_first = false;
    }
    
    writeln!(out, "\n        ]")?;
    write!(out, "}}")
}

pub fn parse_gameassembly() -> io::Result<(
    Vec<TypeInfoEntry>,
    Vec<MethodRefEntry>,
    Vec<StringLiteralEntry>,
    Vec<MethodEntry>,
)> {
    let mut game_assembly = Vec::new();
    File::open("GameAssembly.dll")?.read_to_end(&mut game_assembly)?;

    let mut offset = 0;
    let mut type_info = Vec::new();
    let mut method_refs = Vec::new();
    let mut string_literals = Vec::new();

    while offset + 8 < game_assembly.len() {
        let chunk: [u8; 8] = game_assembly[offset..offset + 8].try_into().unwrap();

        if (chunk[4] | chunk[5] | chunk[6] | chunk[7]) == 0 {
            let value64 = u64::from_le_bytes(chunk);
            if value64 != 0 {
                let value32 = value64 as u32;
                let extracted = value32 >> 29;
                let index = (value32 >> 1) & 0xFFFFFFF;

                match extracted {
                    1 if index < TYPE_INFO_SIZE => {
                        type_info.push(TypeInfoEntry {
                            address: offset + 0x2200,
                            name: dump_type_info(index),
                        });
                    }
                    3 | 6 if index < METHOD_REF_SIZE && index > 0 => {
                        let method_ref = dump_method_ref(value32);
                        if !method_ref.0.is_null() {
                            method_refs.push(MethodRefEntry {
                                address: offset + 0x2200,
                                name: method_ref.name().to_string(),
                                type_name: method_ref.class().il2cpp_type().name().to_string(),
                                method_address: method_ref.address() - il2cpp::assembly_base(),
                            });
                        }
                    }
                    5 if index < STRING_LITERAL_SIZE => {
                        string_literals.push(StringLiteralEntry {
                            address: offset + 0x2200,
                            value: dump_string_literal(index),
                        });
                    }
                    _ => {}
                }
            }
        }
        offset += 8;
    }

    let mut domain_methods = Vec::new();
    for assembly in Il2cppDomain::get().assemblies() {
        let img = assembly.image();
        for i in 0..img.class_count() {
            let class = img.get_class(i);
            for method in class.methods() {
                domain_methods.push(MethodEntry {
                    address: method.address() - il2cpp::assembly_base(),
                    name: method.name().to_string(),
                    type_name: method.class().il2cpp_type().name().to_string(),
                });
            }
        }
    }

    Ok((type_info, method_refs, string_literals, domain_methods))
}

pub fn dump_type_info(index: u32) -> String {
    unsafe {
        let func: extern "system" fn(a: u32) -> *const u8 =
            std::mem::transmute(il2cpp::assembly_base() + TYPE_GET_INFO);
        let class = Il2cppClass(func(index));
        let ty_name = class.il2cpp_type().name();
        ty_name.to_string()
    }
}

pub fn dump_method_ref(value32: u32) -> Il2cppMethod {
    unsafe {
        let func: extern "system" fn(a: u32, b: i64) -> *const u8 =
            std::mem::transmute(il2cpp::assembly_base() + METHOD_REF_GET_INFO);
        Il2cppMethod(func(value32, 0))
    }
}

pub fn dump_string_literal(index: u32) -> String {
    unsafe {
        let index = index as usize;
        let global_metadata = *((il2cpp::assembly_base() + GLOBAL_METADATA) as *const usize);

        let length = *((global_metadata + (8 * index) + 677104) as *const u32) ^ 0xCED2E1A;
        let ptr = *((global_metadata + (8 * index) + 677108) as *const u32) ^ 0x3119B160;
        let data_offset = (global_metadata + 17168724 + ptr as usize) as *const u32;

        String::from_utf8_lossy(&decrypt_string_literal(
            index as u32,
            data_offset as *const u64,
            length as usize,
        ))
        .to_string()
    }
}

fn decrypt_string_literal(index: u32, encrypted: *const u64, len: usize) -> Vec<u8> {
    let mut key = 0x29244D5F2EBC49F0u64
        .wrapping_mul(
            (0x914AF368BAA589CCu64
                .wrapping_mul(index as u64)
                .wrapping_add(0x5E4FD2EE25C248B4u64))
                ^ 0x2E51E27A668AF5E6,
        )
        .wrapping_add(0x428CBAF26DC07F7D);

    let mut out = Vec::with_capacity(len);
    let blocks = (len + 7) / 8;

    unsafe {
        for i in 0..blocks {
            let enc_qword = *encrypted.add(i);
            let dec_qword = enc_qword ^ key;
            key = key.wrapping_add(0x1E04C27937B19B96);

            let bytes = dec_qword.to_le_bytes();
            for b in bytes {
                if out.len() < len {
                    out.push(b);
                }
            }
        }
    }

    out
}
