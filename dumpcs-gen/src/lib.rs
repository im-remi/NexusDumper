use std::{io::{self, Write}, ptr};
use il2cpp::vm::*;


pub fn write_assemblies_list<W: Write>(out: &mut W, domain: &Il2cppDomain) -> io::Result<()> {
    let mut i = 1;
    for assembly in domain.assemblies() {
        let image = assembly.image();

        writeln!(
            out,
            "// Assembly {}: {}, class count: {}",
            i,
            image.name(),
            image.class_count()
        )?;

        i+=1;
    }
    writeln!(out)
}

pub unsafe fn dump<W: Write>(out: &mut W) -> io::Result<()> {
    let domain = Il2cppDomain::get();
    domain.attach_thread();

    write_assemblies_list(out, &domain)?;

    for assembly in domain.assemblies() {
        let image = assembly.image();
        let image_name = image.name().to_string();

        let mut classes: Vec<Il2cppClass> = Vec::new();
        for i in 0..image.class_count() {
            let class = image.get_class(i);
            classes.push(class);
        }

        classes.sort_by_key(|class| class.token());

        for class in classes {
            let namespace = class.namespace();
            writeln!(out, "// Assembly: {image_name}, Namespace: {namespace}")?;

            let flags = class.flags();
            prepend_class_modifiers(out, flags)?;
            if flags & TYPE_ATTRIBUTE_ABSTRACT != 0 && flags & TYPE_ATTRIBUTE_SEALED != 0 {
                write!(out, "static ")?;
            } else if !(flags & TYPE_ATTRIBUTE_INTERFACE != 0) && flags & TYPE_ATTRIBUTE_ABSTRACT != 0 {
                write!(out, "abstract ")?;
            } else if !class.is_struct() && !class.is_enum() && flags & TYPE_ATTRIBUTE_SEALED != 0 {
                write!(out, "sealed ")?;
            }

            if flags & TYPE_ATTRIBUTE_INTERFACE != 0 {
                write!(out, "interface {}", class.name())?;
            } else if class.is_struct() {
                write!(out, "struct {}", class.name())?;
            } else if class.is_enum() {
                write!(out, "enum {}", class.name())?;
            } else {
                write!(out, "class {}", class.name())?;
            }

            if let Some(parent) = class.parent_class() {
                write!(out, " : {}", parent.name())?;
            }

            for (i_iface, interface) in class.interfaces().iter().enumerate() {
                if i_iface == 0 && class.parent_class().is_none() {
                    write!(out, " : {}", interface.name())?;
                } else {
                    write!(out, ", {}", interface.name())?;
                }
            }

            writeln!(out, " {{ // Address: {:?}, Token: 0x{:X}", class.0, class.token())?;

            for field in class.fields().iter() {
                write_class_field(out, field)?;
            }
            if class.fields().iter().next().is_some() {
                writeln!(out)?;
            }
            for method in class.methods().iter() {
                write_class_method(out, method)?;
            }

            writeln!(out, "}}\n")?;
        }
    }

    Ok(())
}

fn write_class_field<W: Write>(out: &mut W, field: &Il2cppField) -> io::Result<()> {
    let field_type = field.il2cpp_type();
    let attrs = field_type.attrs();

    let custom_attributes = field.custom_attributes();
    for attr in custom_attributes.iter() {
        dump_attribute(out, attr)?;
    }
    write!(out, "    ")?;
    prepend_field_modifiers(out, attrs)?;
    write!(out, "{} {}", field_type.name(), field.name())?;

    if attrs & FIELD_ATTRIBUTE_LITERAL != 0 {
        let value = field.static_value();
        match field_type.type_enum() {
            0x3 => write!(out, " = '{}'", char::from(value as u8))?,
            0xC => write!(out, " = {}", f32::from_bits(value as u32))?,
            0xD => write!(out, " = {}", f64::from_bits(value as u64))?,
            0xE => write!(
                out,
                " = \"{}\"",
                Il2cppString(value as *const u8).to_string()
            )?,
            _ => write!(out, " = {value}")?,
        }
    }
    writeln!(out, "; // Offset: 0x{:X}, Token: 0x{:X}", field.offset(), field.token())
}

fn write_class_method<W: Write>(out: &mut W, method: &Il2cppMethod) -> io::Result<()> {
    let base = il2cpp::util::assembly_base() as isize;
    let addr = method.address() as isize;
    
    let rva = if addr != 0 {
        (addr - base) as usize
    } else {
        0
    };
    
    let va = if addr != 0 {
        (addr - base + 0x1800_0000_0) as usize
    } else {
        0
    };
    write!(
        out,
        "    // RVA: 0x{:X}, VA: 0x{:X}\n    ",
        (method.address() != 0)
            .then_some(rva)
            .unwrap_or(0),
        (method.address() != 0)
            .then_some(va)
            .unwrap_or(0)
    )?;
    prepend_method_modifiers(out, method.attrs())?;
    write!(out, "{} {}(", method.return_type().name(), method.name())?;
    
    for i in 0..method.param_count() {
        if i != 0 {
            write!(out, ", ")?;
        }
        write!(out, "{} {}", method.param(i).name(), method.param_name(i))?;
    }
    writeln!(out, ") {{}}\n")
}

fn prepend_field_modifiers<W: Write>(out: &mut W, attrs: u32) -> io::Result<()> {
    match attrs & FIELD_ATTRIBUTE_FIELD_ACCESS_MASK {
        FIELD_ATTRIBUTE_PUBLIC => write!(out, "public "),
        FIELD_ATTRIBUTE_PRIVATE => write!(out, "private "),
        FIELD_ATTRIBUTE_FAMILY => write!(out, "protected "),
        FIELD_ATTRIBUTE_ASSEMBLY | FIELD_ATTRIBUTE_FAM_AND_ASSEM => {
            write!(out, "internal ")
        }
        FIELD_ATTRIBUTE_FAM_OR_ASSEM => write!(out, "protected internal "),
        _ => Ok(()),
    }?;

    if attrs & FIELD_ATTRIBUTE_LITERAL != 0 {
        write!(out, "const ")?;
    } else if attrs & FIELD_ATTRIBUTE_STATIC != 0 {
        write!(out, "static ")?;
    }

    if attrs & FIELD_ATTRIBUTE_INIT_ONLY != 0 {
        write!(out, "readonly ")?;
    }

    Ok(())
}

fn prepend_method_modifiers<W: Write>(out: &mut W, attrs: u32) -> io::Result<()> {
    match attrs & METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK {
        METHOD_ATTRIBUTE_PUBLIC => write!(out, "public "),
        METHOD_ATTRIBUTE_PRIVATE => write!(out, "private "),
        METHOD_ATTRIBUTE_FAMILY => write!(out, "protected "),
        METHOD_ATTRIBUTE_ASSEM | METHOD_ATTRIBUTE_FAM_AND_ASSEM => {
            write!(out, "internal ")
        }
        METHOD_ATTRIBUTE_FAM_OR_ASSEM => write!(out, "protected internal "),
        _ => Ok(()),
    }?;

    if attrs & METHOD_ATTRIBUTE_STATIC != 0 {
        write!(out, "static ")?;
    }
    if attrs & METHOD_ATTRIBUTE_ABSTRACT != 0 {
        write!(out, "abstract ")?;
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
            write!(out, "override ")?;
        }
    } else if attrs & METHOD_ATTRIBUTE_FINAL != 0 {
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
            write!(out, "sealed override ")?;
        }
    } else if attrs & METHOD_ATTRIBUTE_VIRTUAL != 0 {
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_NEW_SLOT {
            write!(out, "virtual ")?;
        } else {
            write!(out, "override ")?;
        }
    }
    if attrs & METHOD_ATTRIBUTE_PINVOKE_IMPL != 0 {
        write!(out, "extern ")?;
    }

    Ok(())
}

fn prepend_class_modifiers<W: Write>(out: &mut W, flags: u32) -> io::Result<()> {
    if flags & TYPE_ATTRIBUTE_SERIALIZABLE != 0 {
        writeln!(out, "[Serializable]")?;
    }
    
    match flags & TYPE_ATTRIBUTE_VISIBILITY_MASK {
        TYPE_ATTRIBUTE_PUBLIC | TYPE_ATTRIBUTE_NESTED_PUBLIC => write!(out, "public "),
        TYPE_ATTRIBUTE_NESTED_PRIVATE => write!(out, "private "),
        TYPE_ATTRIBUTE_NESTED_FAMILY => write!(out, "protected "),
        TYPE_ATTRIBUTE_NOT_PUBLIC | TYPE_ATTRIBUTE_NESTED_FAM_AND_ASSEM | TYPE_ATTRIBUTE_NESTED_ASSEMBLY  => {
            write!(out, "internal ")
        }
        TYPE_ATTRIBUTE_NESTED_FAM_OR_ASSEM => write!(out, "protected internal "),
        _ => Ok(()),
    }?;

    Ok(())
}

fn dump_attribute<W: Write>(out: &mut W, attr: &Il2cppObject) -> io::Result<()> {
    let class = attr.class();
    let attr_name = class.name();

    write!(out, "    [{}(", attr_name)?;

    let mut first = true;

    for field in class.fields().iter() {
        let field_name = extract_property_name(&field.name());
        let field_type = field.il2cpp_type();
        
        let type_name = field_type.name();
        let type_name = type_name.rsplit('.').next().unwrap_or(&type_name);

        let offset = field.offset();
        let field_data_ptr = attr.0.wrapping_add(offset as usize);


        if !first {
            write!(out, ", ")?;
        } else {
            first = false;
        }

        write!(out, "{} = ", field_name)?;

        if let Some(getter) = class.get_method(&format!("get_{}", field_name), 0) && (type_name == "String" || type_name == "System.String"){
            if let Ok(value) = getter.invoke::<usize>(attr, &[]) {
                let s = Il2cppString(value as *const u8).to_string();
                write!(out, "\"{}\"", s)?;
                continue;
            };
        }

        match type_name {
            "String" | "string" | "System.String"=> {
                let str_obj = unsafe { ptr::read_unaligned(field_data_ptr as *const *mut Il2cppString) };
                if !str_obj.is_null() {
                    write!(out, "\"{}\"", unsafe {(*str_obj).to_string() })?;
                } else {
                    write!(out, "null")?;
                }
            },
            "Boolean" | "bool" => {
                let val = unsafe { *(field_data_ptr as *const bool) };
                write!(out, "{}", if val { "true" } else { "false" })?;
            }
            "Int32" | "int" => {
                let val = unsafe { *(field_data_ptr as *const i32) };
                write!(out, "{}", val)?;
            }
            "Single" | "float" => {
                let val = unsafe { *(field_data_ptr as *const f32) };
                write!(out, "{}", val)?;
            }
            "Double" | "double" => {
                let val = unsafe { *(field_data_ptr as *const f64) };
                write!(out, "{}", val)?;
            }
            _ => {
                write!(out, "()")?;
            }
        }
    }

    writeln!(out, ")]")
}

pub fn extract_property_name(backing_field_name: &str) -> String {
    if let (Some(start), Some(end)) = (
        backing_field_name.find('<'),
        backing_field_name.find('>'),
    ) {
        if end > start + 1 {
            return backing_field_name[start + 1..end].to_string();
        }
    }

    backing_field_name.to_string()
}