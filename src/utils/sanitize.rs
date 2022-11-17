use rustrict::Censor;

const ALLOWED_CHARS: &str = r#" !\"$#%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜø£Ø×ƒáíóúñÑªº¿®¬½¼¡«»"#;

pub fn sanitize_message(m: &str) -> String {
    let filtered = m.chars().filter(|c| ALLOWED_CHARS.contains(*c));
    Censor::new(filtered).censor()
}

#[test]
fn filter_works() {
    let m = "Hello, world!";
    assert_eq!(sanitize_message(m), m);
    assert_eq!(sanitize_message("fuckfuckfuck"), "f***f***f***");
    assert_eq!(sanitize_message("f u c k"), "f******");
}
