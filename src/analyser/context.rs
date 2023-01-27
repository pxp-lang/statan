use pxp_parser::lexer::byte_string::ByteString;

#[derive(Debug, Clone)]
pub struct Context {
    namespace: ByteString,
    imports: Vec<ByteString>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            namespace: ByteString::default(),
            imports: Vec::new(),
        }
    }

    pub fn resolve_name(&self, name: &ByteString) -> ByteString {
        // If the name is already fully qualified, return as is.
        if name.bytes.starts_with(b"\\") {
            return name.clone();
        }

        let parts = name.split(|b| *b == b'\\').collect::<Vec<&[u8]>>();
        let first_part = parts.first().unwrap();

        // Check each imported name to see if it ends with the first part of the
        // given identifier. If it does, we can assume you're referencing either
        // an imported namespace or class that has been imported.
        for imported_name in self.imports.iter() {
            if imported_name.ends_with(first_part) {
                let mut qualified_name = imported_name.clone();
                qualified_name.extend(&name.bytes[first_part.len()..]);

                return qualified_name;
            }
        }

        // If we've reached this point, we have a simple name that
        // is not fully qualified and we have not imported it.
        // We can simply prepend the current namespace to it.
        let mut qualified_name = self.namespace.clone();
        qualified_name.extend(b"\\");
        qualified_name.extend(&name.bytes);

        qualified_name
    }

    pub fn set_namespace(&mut self, namespace: ByteString) {
        self.namespace = namespace;
    }

    pub fn add_import(&mut self, import: ByteString) {
        self.imports.push(import);
    }
}