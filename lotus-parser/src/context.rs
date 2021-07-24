use std::{collections::{HashMap}, ops::Deref};

use parsable::Parsable;

use crate::{constants::KEYWORDS, definitions::struct_definition::{FieldKind, StructDefinition}, error::{Error}, items::{file::LotusFile, function_declaration::FunctionDeclaration, identifier::Identifier, statement::{VarDeclaration, VarDeclarationQualifier}, struct_declaration::{StructDeclaration, StructQualifier}, top_level_block::TopLevelBlock}};

#[derive(Default)]
pub struct Context {
    pub errors: Vec<Error>,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub constant_declarations: HashMap<Identifier, VarDeclaration>,

    pub struct_definitions: HashMap<Identifier, StructDefinition>
}

impl Context {
    pub fn new(files: Vec<LotusFile>) -> Self {
        let mut context = Self::default();

        context.build_index(files);
        context.process_structs();

        context
    }

    fn error<T : Parsable, S : Deref<Target=str>>(&mut self, data: &T, error: S) {
        self.errors.push(Error::new(data, error));
    }

    fn build_index(&mut self, files: Vec<LotusFile>) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => {
                        if const_declaration.qualifier != VarDeclarationQualifier::Const {
                            self.error(const_declaration, "global variables must be declared with the \"const\" qualifier");
                        }

                        &const_declaration.name
                    },
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.constant_declarations.contains_key(&identifier) {
                    self.error(&identifier, format!("duplicate declaration: {}", identifier));
                }

                match block {
                    TopLevelBlock::StructDeclaration(def) => { self.struct_declarations.insert(identifier, def); },
                    TopLevelBlock::ConstDeclaration(var_declaration) => { self.constant_declarations.insert(identifier, var_declaration); },
                    TopLevelBlock::FunctionDeclaration(def) => { self.function_declarations.insert(identifier, def); },
                }
            }
        }
    }

    fn process_structs(&mut self) {
        let mut errors = vec![];

        for struct_declaration in self.struct_declarations.values() {
            if self.is_keyword(&struct_declaration.name) {
                errors.push(Error::new(struct_declaration, format!("invalid type name: {}", &struct_declaration.name)));
            } else {
                let mut struct_def = StructDefinition {
                    name: struct_declaration.name.clone(),
                    qualifier: struct_declaration.qualifier,
                    types: vec![],
                    fields: HashMap::new(),
                };

                self.collect_struct_types(&struct_declaration.name, &mut struct_def.types, &mut errors);
                self.collect_struct_fields(&mut struct_def, &mut errors);
            }
        }

        self.errors.append(&mut errors);
    }

    fn collect_struct_types(&self, struct_name: &Identifier, types: &mut Vec<Identifier>, errors: &mut Vec<Error>) {
        if types.contains(struct_name) {
            errors.push(Error::new(struct_name, format!("circular inheritance: {}", struct_name)));
        } else {
            types.push(struct_name.clone());

            let struct_def = self.struct_declarations.get(&struct_name).unwrap();

            if let Some(parent_name) = &struct_def.parent {
                if let Some(parent) = self.struct_declarations.get(parent_name) {
                    if parent.qualifier != struct_def.qualifier {
                        errors.push(Error::new(parent_name, format!("a {} cannot inherit from a {}", struct_def.qualifier, parent.qualifier)));
                    } else {
                        self.collect_struct_types(parent_name, types, errors);
                    }
                } else if self.is_builtin_type_name(parent_name) {
                    errors.push(Error::new(parent_name, format!("cannot inherit from builtin type: {}", parent_name)))
                } else {
                    errors.push(Error::new(parent_name, format!("unknown type: {}", parent_name)))
                }
            }
        }
    }

    fn collect_struct_fields(&self, struct_def: &mut StructDefinition, errors: &mut Vec<Error>) {
        for type_name in struct_def.types.clone().iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_keyword(&field.name) {
                    errors.push(Error::new(&field.name, format!("invalid field name: {}", &field.name)));
                } else {
                    if !self.is_builtin_type_name(&field.type_name) {
                        if let Some(field_struct_declaration) = self.struct_declarations.get(&field.type_name) {
                            if field_struct_declaration.qualifier != StructQualifier::Entity {
                                errors.push(Error::new(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_name)));
                            } else {
                                struct_def.add_field(&field.name, &field.type_name, FieldKind::from_suffix(&field.suffix));
                            }
                        } else {
                            errors.push(Error::new(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_name)));
                        }
                    }
                }
            }
        }
    }

    fn is_keyword(&self, identifier: &Identifier) -> bool {
        KEYWORDS.contains(&identifier.value.as_str())
    }

    fn is_builtin_type_name(&self, name: &Identifier) -> bool {
        name.value == "bool" || name.value == "num"
    }

    fn is_type_name(&self, name: &Identifier) -> bool {
        self.is_builtin_type_name(name) || self.struct_declarations.contains_key(name)
    }
}