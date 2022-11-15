use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ImportFromData<'a> {
    pub module: &'a Option<String>,
    pub level: &'a Option<usize>,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct AliasData<'a> {
    pub name: &'a str,
    pub asname: &'a Option<String>,
}

pub trait Importable {
    fn module_name(&self) -> String;
    fn module_base(&self) -> String;
}

impl Importable for AliasData<'_> {
    fn module_name(&self) -> String {
        self.name.to_string()
    }

    fn module_base(&self) -> String {
        self.module_name().split('.').next().unwrap().to_string()
    }
}

impl Importable for ImportFromData<'_> {
    fn module_name(&self) -> String {
        let mut module_name = String::new();
        if let Some(level) = self.level {
            if level > &0 {
                module_name.push_str(&".".repeat(*level));
            }
        }
        if let Some(module) = self.module {
            module_name.push_str(module);
        }
        module_name
    }

    fn module_base(&self) -> String {
        self.module_name().split('.').next().unwrap().to_string()
    }
}

#[derive(Debug, Default)]
pub struct ImportBlock<'a> {
    // Set of (name, asname), used to track regular imports.
    // Ex) `import module`
    pub import: FnvHashSet<AliasData<'a>>,
    // Map from (module, level) to `AliasData`, used to track 'from' imports.
    // Ex) `from module import member`
    pub import_from: FnvHashMap<ImportFromData<'a>, FnvHashSet<AliasData<'a>>>,
    // Set of (module, level, name, asname), used to track re-exported 'from' imports.
    // Ex) `from module import member as member`
    pub import_from_as: FnvHashSet<(ImportFromData<'a>, AliasData<'a>)>,
}

#[derive(Debug, Default)]
pub struct OrderedImportBlock<'a> {
    pub import: Vec<AliasData<'a>>,
    pub import_from: Vec<(ImportFromData<'a>, Vec<AliasData<'a>>)>,
}