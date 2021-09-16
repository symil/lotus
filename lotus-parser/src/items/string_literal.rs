use parsable::parsable;
use crate::{program::{BuiltinType, NEW_FUNC_NAME, ProgramContext, SET_CHAR_FUNC_NAME, VI, Vasm}, wat};

#[parsable(name="string")]
pub struct StringLiteral {
    #[parsable(regex = r##""(\\.|[^"\\])*""##)]
    pub value: String
}

impl StringLiteral {
    pub fn to_actual_string(&self) -> String {
        // TODO: remove quotes and unescape the string
        self.value.clone()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut chars : Vec<char> = self.value.chars().collect();

        chars.remove(0);
        chars.remove(chars.len() - 1);

        let mut escaping = false;
        let mut unescaped_chars = vec![];

        for c in chars {
            if escaping {
                let unescaped_char_opt = match c {
                    '0' => Some('\0'),
                    '\\' => Some('\\'),
                    '"' => Some('"'),
                    't' => Some('\t'),
                    'n' => Some('\n'),
                    _ => None
                };

                if let Some(unescaped_char) = unescaped_char_opt {
                    unescaped_chars.push(unescaped_char as u32);
                } else {
                    context.errors.add(self, format!("invalid character '\\{}'", c));
                }

                escaping = false;
            } else if c == '\\' {
                escaping = true;
            } else {
                unescaped_chars.push(c as u32);
            }
        }

        let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
        let mut content = vec![
            VI::call_static_method(&string_type, NEW_FUNC_NAME, vec![VI::int(unescaped_chars.len())])
        ];

        for (i, code) in unescaped_chars.into_iter().enumerate() {
            content.push(
                VI::call_method(&string_type, SET_CHAR_FUNC_NAME, vec![VI::int(i), VI::int(code)]),
            );
        }

        Some(Vasm::new(string_type, vec![], content))
    }
}