use crate::{
    ast::VariableDefinition,
    parser::{SourcePosition, Spanning},
    types::utilities::is_valid_literal_value,
    validation::{ValidatorContext, Visitor},
    value::ScalarValue,
    Selection,
};

pub struct SingleQueryOnly {
    is_root: bool,
}

pub fn factory() -> SingleQueryOnly {
    SingleQueryOnly { is_root: true }
}

impl<'a, S> Visitor<'a, S> for SingleQueryOnly
where
    S: ScalarValue,
{
    fn enter_selection_set(
        &mut self,
        ctx: &mut ValidatorContext<'a, S>,
        selections: &'a [crate::Selection<S>],
    ) {
        if self.is_root {
            if selections.len() > 1 {
                ctx.report_error(
                    "Query too complex",
                    &[],
                );
            }
            self.is_root = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::factory;

    use crate::{
        parser::SourcePosition,
        validation::{expect_fails_rule, expect_passes_rule, RuleError},
        value::DefaultScalarValue,
    };

    #[test]
    fn variables_with_no_default_values() {
        expect_passes_rule::<_, _, DefaultScalarValue>(
            factory,
            r#"
          query Hello {
            dog { 
                name
                lastName
            }
          }
        "#,
        );
    }

    #[test]
    fn too_many_queries() {
        expect_fails_rule::<_, _, DefaultScalarValue>(
            factory,
            r#"
          query UnreachableDefaultValues {
            dog { name }
            cat {
                 name 
                title
            }
          }
        "#,
            &[
                RuleError::new("Query too complex", &[]),
            ],
        );
    }
}
