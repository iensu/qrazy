use qrcode::{Color, QrCode};

#[derive(Debug)]
pub enum QrError {
    GenerationFailed(String),
}

impl std::fmt::Display for QrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QrError::GenerationFailed(msg) => write!(f, "Failed to generate QR code: {msg}"),
        }
    }
}

impl std::error::Error for QrError {}

pub struct QrData {
    pub bits: Vec<u8>,
    pub width: u32,
}

impl QrData {
    pub fn to_vec(self) -> Vec<u8> {
        self.width
            .to_le_bytes()
            .into_iter()
            .chain(self.bits.into_iter())
            .collect()
    }
}

pub fn generate(input: &str) -> Result<QrData, QrError> {
    let code =
        QrCode::new(input.as_bytes()).map_err(|e| QrError::GenerationFailed(format!("{e}")))?;
    let width = code.width() as u32;
    let bits = code
        .into_colors()
        .iter()
        .map(|x| match x {
            Color::Light => 0,
            Color::Dark => 1,
        })
        .collect();

    Ok(QrData { bits, width })
}
