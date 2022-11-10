use rustrict::Censor;

const ALLOWED_CHARS: &str = r#" !\"$#%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜø£Ø×ƒáíóúñÑªº¿®¬½¼¡«»"#;

pub fn sanitize_message(m: &str) -> String {
    let filtered = m.chars().filter(|c| ALLOWED_CHARS.contains(*c));
    Censor::new(filtered).censor()
}
