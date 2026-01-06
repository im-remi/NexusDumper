use il2cpp::vm::*;

pub const GET_NAME_FROM_CMD_ID: usize = 0x1F87D50;

pub unsafe fn write_cmd_ids<W: std::io::Write>(out: &mut W) -> std::io::Result<()> {
    let func: extern "C" fn(a: i32) -> *const u8 =
        unsafe { std::mem::transmute(il2cpp::assembly_base() + GET_NAME_FROM_CMD_ID) };
    writeln!(out, "{{")?;

    let mut is_first = true;
    for i in 0..2200 {
        let str = Il2cppString(func(i)).to_string();
        if str.contains("Cmd") {
            if !is_first {
                writeln!(out, ",")?;
            }
            is_first = false;

            write!(out, "    \"{}\": {}", str.strip_prefix("Cmd").unwrap_or(&str), i)?;
        }
    }

    write!(out, "\n}}")
}
