use parsable::parsable;
use crate::program::{ProgramContext, VISIBILITY_KEYWORDS, VAR_DECLARATION_KEYWORDS, TYPE_DECLARATION_KEYWORDS, FUNCTION_DECLARATION_KEYWORDS};
use super::{Identifier, Word};

thread_local! {
    static NON_VISIBILITY_KEYWORDS : Vec<&'static str> = {
        let mut keywords = TYPE_DECLARATION_KEYWORDS.to_vec();
        keywords.extend_from_slice(VAR_DECLARATION_KEYWORDS);
        keywords.extend_from_slice(FUNCTION_DECLARATION_KEYWORDS);
        keywords
    };

    static BLOCK_START_KEYWORDS : Vec<&'static str> = {
        let mut keywords = TYPE_DECLARATION_KEYWORDS.to_vec();
        keywords.extend_from_slice(VAR_DECLARATION_KEYWORDS);
        keywords.extend_from_slice(FUNCTION_DECLARATION_KEYWORDS);
        keywords.extend_from_slice(VISIBILITY_KEYWORDS);
        keywords
    };
}

#[parsable]
pub struct ParsedTopLevelBlockInProgress {
    pub visibility_keyword: Word,
    pub qualifier_keyword: Option<Word>
}

impl ParsedTopLevelBlockInProgress {
    pub fn process(&self, context: &mut ProgramContext) {
        NON_VISIBILITY_KEYWORDS.with(|non_visibility_keywords| {
            match &self.qualifier_keyword {
                Some(qualifier_keyword) => {
                        context.completion_provider.add_keyword_completion(&self.visibility_keyword, VISIBILITY_KEYWORDS);
                        context.completion_provider.add_keyword_completion(qualifier_keyword, non_visibility_keywords);

                        if !VISIBILITY_KEYWORDS.contains(&self.visibility_keyword.as_str()) {
                            context.errors.keyword_mismatch(&self.visibility_keyword, VISIBILITY_KEYWORDS);
                        } else {
                            context.errors.expected_identifier(qualifier_keyword);
                        }

                        if !non_visibility_keywords.contains(&qualifier_keyword.as_str()) {
                            context.errors.keyword_mismatch(qualifier_keyword, non_visibility_keywords);
                        }
                },
                None => {
                    BLOCK_START_KEYWORDS.with(|all_keywords| {
                        context.completion_provider.add_keyword_completion(self, all_keywords);

                        if VISIBILITY_KEYWORDS.contains(&self.visibility_keyword.as_str()) {
                            context.errors.expected_keyword_among(&self.visibility_keyword, non_visibility_keywords);
                        } else if !all_keywords.contains(&self.visibility_keyword.as_str()) {
                            context.errors.keyword_mismatch(&self.visibility_keyword, all_keywords);
                        } else {
                            context.errors.expected_identifier(&self.visibility_keyword);
                        }
                    });
                },
            }
        });
    }
}