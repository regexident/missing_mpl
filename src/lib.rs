// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin_registrar, plugin)]
#![feature(box_syntax, rustc_private)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_lint;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;

use std::collections::HashSet;
use std::str;

use rustc_ast::{ast::Mod, node_id::NodeId};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::{BytePos, Span};

static MPL_HEADER: &str = r#"
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
"#;

declare_lint! {
    pub MISSING_MPL,
    Warn,
    "detects missing MPL-2.0 license headers"
}

#[derive(Default)]
struct MissingMpl {
    file_pos: HashSet<BytePos>,
}

impl_lint_pass!(MissingMpl => [MISSING_MPL]);

impl EarlyLintPass for MissingMpl {
    fn check_mod(&mut self, context: &EarlyContext, module: &Mod, _: Span, _: NodeId) {
        let span = module.inner;

        let code_map = context.sess.source_map();
        let location = code_map.lookup_char_pos(span.lo());
        let file_map = location.file;

        if !file_map.is_real_file() {
            return;
        }

        let file_pos = file_map.start_pos;
        let mod_pos = span.lo();
        if self.file_pos.contains(&file_pos) {
            return;
        }
        self.file_pos.insert(file_pos);

        if let Some(ref src) = file_map.src {
            let len = (mod_pos - file_pos).0 as usize;

            let header_bytes = &src.as_bytes()[0..len];
            let header = str::from_utf8(header_bytes).unwrap().trim();
            let distance = strsim::levenshtein(header, MPL_HEADER);

            if distance > MPL_HEADER.len() / 10 {
                let lint_span = span.with_hi(span.lo() + BytePos(1));
                let message = "Missing MPL license header in source file.";
                let help = format!("The license should look like this:\n{}", MPL_HEADER);

                context.struct_span_lint(MISSING_MPL, lint_span, |diag| {
                    let mut diag = diag.build(message);
                    diag.help(&help);
                    diag.emit();
                });
            }
        }
    }
}
