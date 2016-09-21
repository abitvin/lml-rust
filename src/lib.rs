// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

extern crate grammer;
extern crate rule;
use grammer::Grammer;
use rule::RuleError;

pub struct Lml {
    pub children: Vec<Lml>,
    pub tag: String,
    pub text: String,
}

impl Lml
{
    pub fn parse(text: &str) -> Result<Lml, Vec<RuleError>>
    {
        let branch_fn = |mut b: Vec<Lml>, _: &_, _: &mut _| {
            let mut tag = String::new();

            if b.len() > 0 && b[0].tag.len() > 0 {
                tag = b.remove(0).tag;
            }

            vec![Lml {
                children: b,
                tag: tag,
                text: String::new(),
            }]
        };

        let root_fn = |b: Vec<Lml>, _: &_, _: &mut _| {
            vec![Lml {
                children: b,
                tag: String::new(),
                text: String::new(),
            }]
        };

        let tag_fn = |_: Vec<Lml>, l: &str, _: &mut _| {
            vec![Lml {
                children: Vec::new(),
                tag: String::from(l),
                text: String::new(),
            }]
        };

        let text_fn = |_: Vec<Lml>, l: &str, _: &mut _| {
            vec![Lml {
                children: Vec::new(),
                tag: String::new(),
                text: String::from(l),
            }]
        };

        let mut g: Grammer<Lml, ()> = Grammer::new();
        g.ws("(\\ |\t|\n|\r)");
        g.declare(vec!["branch", "branch-start"]);
        g.add("escape-chars", "(~\\{\\{,\\{|\\}\\},\\})", None);
        g.add("control-chars", "(\\ |\t|\n|\r|\\{|\\})", None);
        g.add("char", "(<escape-chars>|!<control-chars>.)", None);
        g.add("tag", "<char>+", Some(Box::new(tag_fn)));
        g.add("word", "!<branch-start><char>+", None);
        g.add("text", "<word>( <word>)*", Some(Box::new(text_fn)));
        g.add("child", "(<text>|<branch>)", None);
        g.add("children", "<child>( <child>)*", None);
        g.add("branch-start", "<tag> \\{!(\\{)", None);
        g.add("branch", "<branch-start> <children>? }", Some(Box::new(branch_fn)));
        g.add("root", " <children>? ", Some(Box::new(root_fn)));

        match g.scan("root", text, &mut ()) {
            Ok(mut b) => Ok(b.remove(0)),
            Err(e) => Err(e)
        }
    }
} 