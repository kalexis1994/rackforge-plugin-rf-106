//! Compact-cassette artwork for the eight program-library bays.
//!
//! The JUNO-106 backed up a complete memory group to ordinary cassette tape.
//! RF-106 uses that physical object as the visual container for a host-owned
//! `.106` or `.syx` file. Every drawing receives a key so its SVG definitions remain
//! independent when all eight cassettes are on screen.

use core::fmt::Write;

const LABEL_CHARS: usize = 24;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Face {
    pub bay: usize,
    pub programs: usize,
    pub empty: bool,
    pub label: String,
}

fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn cut(value: &str) -> String {
    if value.chars().count() <= LABEL_CHARS {
        return value.to_owned();
    }
    let mut shortened: String = value.chars().take(LABEL_CHARS - 1).collect();
    shortened.push('…');
    shortened
}

fn id_key(key: &str) -> String {
    key.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}

pub fn svg(face: &Face, key: &str) -> String {
    let key = id_key(key);
    let label = escape(&cut(&face.label));
    let status = if face.empty {
        "EMPTY".to_owned()
    } else if face.programs == 1 {
        "1 PROGRAM".to_owned()
    } else {
        format!("{} PROGRAMS", face.programs)
    };
    let aria = escape(&format!("Cassette bay {}, {status}", face.bay + 1));
    let reel = |x: i32| {
        format!(
            "<g transform=\"translate({x} 103)\"><circle r=\"27\" fill=\"#d8d4c8\" stroke=\"#77736c\" stroke-width=\"2\"/><circle r=\"19\" fill=\"#171718\"/><g fill=\"#d8d4c8\"><rect x=\"-3\" y=\"-18\" width=\"6\" height=\"12\" rx=\"2\"/><rect x=\"-3\" y=\"6\" width=\"6\" height=\"12\" rx=\"2\"/><rect x=\"6\" y=\"-3\" width=\"12\" height=\"6\" rx=\"2\"/><rect x=\"-18\" y=\"-3\" width=\"12\" height=\"6\" rx=\"2\"/></g><circle r=\"5\" fill=\"#29292b\" stroke=\"#9e9a90\"/></g>"
        )
    };
    let mut output = String::with_capacity(5_000);
    let _ = write!(
        output,
        "<svg class=\"rf-cassette\" viewBox=\"0 0 320 210\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"{aria}\">\
<defs>\
<linearGradient id=\"shell-{key}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\"><stop offset=\"0\" stop-color=\"#363536\"/><stop offset=\".48\" stop-color=\"#1c1b1c\"/><stop offset=\"1\" stop-color=\"#0d0d0e\"/></linearGradient>\
<linearGradient id=\"label-{key}\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#d9d5c9\"/><stop offset=\"1\" stop-color=\"#aaa69d\"/></linearGradient>\
<linearGradient id=\"window-{key}\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#050506\"/><stop offset=\"1\" stop-color=\"#242327\"/></linearGradient>\
<filter id=\"shadow-{key}\" x=\"-10%\" y=\"-15%\" width=\"120%\" height=\"140%\"><feDropShadow dx=\"0\" dy=\"7\" stdDeviation=\"6\" flood-color=\"#000\" flood-opacity=\".62\"/></filter>\
</defs>\
<g filter=\"url(#shadow-{key})\">\
<rect x=\"8\" y=\"8\" width=\"304\" height=\"190\" rx=\"15\" fill=\"url(#shell-{key})\" stroke=\"#080809\" stroke-width=\"3\"/>\
<path d=\"M25 20H295M25 187H295\" stroke=\"#fff\" stroke-opacity=\".08\"/>\
<rect x=\"31\" y=\"28\" width=\"258\" height=\"112\" rx=\"7\" fill=\"url(#label-{key})\" stroke=\"#5a5752\"/>\
<path d=\"M38 39H282\" stroke=\"#e73020\" stroke-width=\"9\"/><path d=\"M38 50H282\" stroke=\"#2c9ed0\" stroke-width=\"5\"/>\
<text x=\"44\" y=\"70\" fill=\"#272628\" font-family=\"Arial Narrow,Segoe UI,sans-serif\" font-size=\"10\" font-weight=\"800\" letter-spacing=\"1.1\">RF-106 PROGRAM DATA</text>\
<text x=\"276\" y=\"70\" text-anchor=\"end\" fill=\"#272628\" font-family=\"Consolas,monospace\" font-size=\"10\" font-weight=\"800\">BAY {}</text>\
<rect x=\"57\" y=\"77\" width=\"206\" height=\"54\" rx=\"21\" fill=\"url(#window-{key})\" stroke=\"#74716a\" stroke-width=\"2\"/>\
<path d=\"M116 91C142 82 178 82 204 91M116 115C142 124 178 124 204 115\" fill=\"none\" stroke=\"#5b3924\" stroke-width=\"5\" opacity=\".82\"/>\
{}{}\
<rect x=\"139\" y=\"91\" width=\"42\" height=\"24\" rx=\"4\" fill=\"#bab6ab\" stroke=\"#77736c\"/>\
<text x=\"160\" y=\"107\" text-anchor=\"middle\" fill=\"#343235\" font-family=\"Consolas,monospace\" font-size=\"9\" font-weight=\"800\">106</text>\
<text x=\"44\" y=\"135\" fill=\"#302e30\" font-family=\"Consolas,monospace\" font-size=\"9\" font-weight=\"800\">{label}</text>\
<text x=\"276\" y=\"135\" text-anchor=\"end\" fill=\"#302e30\" font-family=\"Consolas,monospace\" font-size=\"9\" font-weight=\"800\">{status}</text>\
<path d=\"M77 143H243L271 194H49Z\" fill=\"#222123\" stroke=\"#080809\" stroke-width=\"2\"/>\
<path d=\"M96 153H224L236 184H84Z\" fill=\"#111113\" stroke=\"#4b494b\"/>\
<circle cx=\"116\" cy=\"171\" r=\"6\" fill=\"#050506\" stroke=\"#77736c\"/><circle cx=\"204\" cy=\"171\" r=\"6\" fill=\"#050506\" stroke=\"#77736c\"/>\
<rect x=\"151\" y=\"157\" width=\"18\" height=\"22\" rx=\"3\" fill=\"#09090a\" stroke=\"#69666a\"/>\
<g fill=\"#080809\" stroke=\"#77736c\"><circle cx=\"24\" cy=\"24\" r=\"4\"/><circle cx=\"296\" cy=\"24\" r=\"4\"/><circle cx=\"30\" cy=\"181\" r=\"4\"/><circle cx=\"290\" cy=\"181\" r=\"4\"/></g>\
</g></svg>",
        face.bay + 1,
        reel(101),
        reel(219),
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cassette_is_keyed_labelled_and_escaped() {
        let drawing = svg(
            &Face {
                bay: 2,
                programs: 17,
                empty: false,
                label: "Brass & Strings <Set>".to_owned(),
            },
            "bay 3/x",
        );
        assert!(drawing.contains("id=\"shell-bay-3-x\""));
        assert!(drawing.contains("BAY 3"));
        assert!(drawing.contains("17 PROGRAMS"));
        assert!(drawing.contains("Brass &amp; Strings &lt;Set&gt;"));
        assert!(!drawing.contains("<Set>"));
        assert_eq!(drawing.matches("translate(").count(), 2);
    }
}
