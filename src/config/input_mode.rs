use vi;

pub trait Transformer {
    fn transform(&self, input: Vec<char>, output: &mut String);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    English,
    Vni,
    Telex,
}

impl InputMode {
    pub fn get_transformer(&self) -> Box<dyn Transformer> {
        match self {
            InputMode::English => Box::new(EnglishTransformer),
            InputMode::Vni => Box::new(VniTransformer),
            InputMode::Telex => Box::new(TelexTransformer),
        }
    }
}

struct EnglishTransformer;
impl Transformer for EnglishTransformer {
    fn transform(&self, input: Vec<char>, output: &mut String) {
        output.extend(input);
    }
}

struct TelexTransformer;
impl Transformer for TelexTransformer {
    fn transform(&self, input: Vec<char>, output: &mut String) {
        vi::transform_buffer(&vi::TELEX, input, output);
    }
}

struct VniTransformer;
impl Transformer for VniTransformer {
    fn transform(&self, input: Vec<char>, output: &mut String) {
        vi::transform_buffer(&vi::VNI, input, output);
    }
}
