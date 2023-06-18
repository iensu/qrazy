use qrcode::{Color, QrCode};

#[derive(Debug)]
pub enum QrError {
    /// Failed to generate the QR code.
    GenerationFailed,
}

impl std::fmt::Display for QrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QrError::GenerationFailed => write!(f, "Failed to generate QR code"),
        }
    }
}

impl std::error::Error for QrError {}

/// QR code data representation.
pub struct QrData {
    /// QR code represented as bits where `1` indicates dark and `0` indicates light.
    pub bits: Vec<u8>,
    /// Width of the sides of the QR code.
    pub width: u32,
}

impl QrData {
    /// Convert the `QrData` struct into a vector beginning with the `width`
    /// in as `u32` in little-endian bytes followed by the vector of bits.
    #[must_use]
    pub fn to_vec(self) -> Vec<u8> {
        self.width
            .to_le_bytes()
            .into_iter()
            .chain(self.bits.into_iter())
            .collect()
    }

    /// Scale the `QrData` to the given `pixel_width`.
    #[must_use]
    pub fn scale(&self, pixel_width: usize) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let width = self.width * pixel_width as u32;
        let bits = self
            .bits
            .chunks(self.width as usize)
            .map(|row| {
                row.iter()
                    .flat_map(|x| std::iter::repeat(*x).take(pixel_width))
                    .collect::<Vec<_>>()
            })
            .flat_map(|row| row.repeat(pixel_width))
            .collect::<Vec<u8>>();
        QrData { bits, width }
    }
}

/// Generate a QR code based on the `input` string.
///
/// # Errors
///
/// Returns `QrError::GenerationFailed` if QR code generation fails.
pub fn generate(input: &str) -> Result<QrData, QrError> {
    let code = QrCode::new(input.as_bytes()).map_err(|_| QrError::GenerationFailed)?;
    #[allow(clippy::cast_possible_truncation)]
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
