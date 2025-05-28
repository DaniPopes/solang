#[test]
fn not_actually_keywords() {
    let src = r#"
struct S {
    uint error;
    uint layout;
    uint at;
    // uint transient;
}

function f() {
    uint error = 0;
    uint layout = 0;
    uint at = 0;
    // uint transient = 0;

    error = 0;
    // layout = 0;
    at = 0;
    // transient = 0;

    S memory x = S({
        error: 0,
        layout: 0,
        at: 0
        // transient: 0
    });

    x.error = 0;
    x.layout = 0;
    x.at = 0;
    // x.transient = 0;

    assembly {
        let error := 0
        let layout := 0
        let at := 0
        // let transient := 0

        error := 0
        layout := 0
        at := 0
        // transient := 0
    }
}
    "#;

    let _ = crate::parse(src, 0).unwrap();
}
