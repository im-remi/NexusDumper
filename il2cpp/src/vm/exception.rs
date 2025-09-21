use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Il2cppException(Il2cppObject);

impl From<usize> for Il2cppException {
    fn from(value: usize) -> Self {
        Self(Il2cppObject::from(value))
    }
}

impl From<Il2cppObject> for Il2cppException {
    fn from(value: Il2cppObject) -> Self {
        Self(value)
    }
}

impl Il2cppException {
    pub fn object(&self) -> Il2cppObject {
        self.0
    }

    pub fn message(&self) -> Il2cppString {
        self.0
            .class()
            .get_method_recursive("get_Message", 0)
            .unwrap()
            .invoke(&self.0, &[])
            .expect("failed to get exception message")
    }

    pub fn stack_trace(&self) -> Il2cppString {
        self.0
            .class()
            .get_method_recursive("get_StackTrace", 0)
            .unwrap()
            .invoke(&self.0, &[])
            .expect("Failed to get exception stack trace")
    }
}

impl std::fmt::Debug for Il2cppException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Il2cppException")
            .field("type", &self.0.class().il2cpp_type().name())
            .field("message", &self.message().to_string())
            .field("stack_trace", &self.stack_trace().to_string())
            .finish()
    }
}

impl std::fmt::Display for Il2cppException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Exception: type: {}\nMessage: {}\nStackTrace:\n{}",
            self.0.class().il2cpp_type().name(),
            self.message(),
            self.stack_trace()
        )
    }
}

impl std::error::Error for Il2cppException {}
