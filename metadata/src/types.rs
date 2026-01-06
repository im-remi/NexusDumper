
#[derive(Debug)]
pub struct TypeInfoEntry {
    pub address: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct MethodRefEntry {
    pub address: usize,
    pub name: String,
    pub type_name: String,
    pub method_address: usize,
}

#[derive(Debug)]
pub struct MethodEntry {
    pub address: usize,
    pub type_name: String,
    pub name: String,
}

#[derive(Debug)]
pub struct StringLiteralEntry {
    pub address: usize,
    pub value: String,
}