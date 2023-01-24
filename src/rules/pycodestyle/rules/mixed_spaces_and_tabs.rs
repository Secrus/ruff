use rustpython_ast::Location;

use crate::ast::types::Range;
use crate::ast::whitespace::leading_space;
use crate::registry::Diagnostic;
use crate::violations;

/// E101
pub fn mixed_spaces_and_tabs(lineno: usize, line: &str) -> Option<Diagnostic> {
    let indent = leading_space(line);

    if indent.contains(' ') && indent.contains('\t') {
        Some(Diagnostic::new(
            violations::MixedSpacesAndTabs,
            Range::new(
                Location::new(lineno + 1, 0),
                Location::new(lineno + 1, indent.chars().count()),
            ),
        ))
    } else {
        None
    }
}