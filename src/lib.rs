// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]
#![feature(macro_vis_matcher)]

extern crate syntax;
extern crate syntax_pos;

// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

extern crate strsim;

use std::collections::HashSet;
use std::str;

use rustc::lint::{
    EarlyContext, EarlyLintPass, EarlyLintPassObject, LintArray, LintContext, LintPass,
};
use rustc_plugin::Registry;
use syntax::ast::{Mod, NodeId};
use syntax::source_map::Span;
use syntax_pos::{BytePos, Pos};

static MPL_HEADER: &'static str = r#"
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
"#;

declare_lint!(
    MISSING_MPL,
    Warn,
    "Warn about missing MPL license header in source file."
);

#[derive(Default)]
struct Pass {
    file_pos: HashSet<BytePos>,
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(MISSING_MPL)
    }
}

impl EarlyLintPass for Pass {
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
                let lint_span = span.with_hi(span.lo() + BytePos::from_usize(1));
                let message = format!("Missing MPL license header in source file:\n{}", MPL_HEADER);
                context.span_lint(MISSING_MPL, lint_span, &message);
            }
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let pass = Pass::default();
    reg.register_early_lint_pass(box pass as EarlyLintPassObject);
}
