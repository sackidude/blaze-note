use blaze_note_parser::compile_to_markdown;

#[test]
fn empty() {
    let compiled = compile_to_markdown("").unwrap();
    assert_eq!(compiled, "");
}

#[test]
fn basic() {
    let compiled = compile_to_markdown("a{{b|c}}d").unwrap();
    assert_eq!(
        compiled,
        r#"a<span class="card front-back-card"><span class="front">b</span><span class="back">c</span></span>d"#
    )
}

#[test]
fn list() {
    let compiled = compile_to_markdown("a{{b|>1.b2.c}}d").unwrap();
    assert_eq!(
        compiled,
        r#"a<span class="card list-card"><span class="question">b</span><span class="entries">1.b2.c</span></span>d"#
    )
}

#[test]
fn reveal() {
    let compiled = compile_to_markdown("a{{b||c||d}}e").unwrap();
    assert_eq!(
        compiled,
        r#"a<span class="card reveal-card">b<span class="hidden">c</span>d</span>e"#
    )
}

#[test]
fn multiple() {
    let compiled = compile_to_markdown("a{{b|c}}d{{e||f||g}}h{{i|>1.j2.k}}l").unwrap();
    assert_eq!(
        compiled,
        r#"a<span class="card front-back-card"><span class="front">b</span><span class="back">c</span></span>d<span class="card reveal-card">e<span class="hidden">f</span>g</span>h<span class="card list-card"><span class="question">i</span><span class="entries">1.j2.k</span></span>l"#
    )
}
