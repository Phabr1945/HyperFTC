extern crate qrcode_core;
use qrcode_core::*;

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct QR(QrCode);

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct QRMatrix(Vec<Vec<bool>>);

impl QR {
    pub fn from_str(value: &str) -> AnyResult<Self> {
        Ok(QR(QrCode::encode_text(value, QrCodeEcc::Low)?))
    }

    pub fn from_string(value: String) -> AnyResult<Self> {
        Self::from_str(&value)
    }

    pub fn image(&self, image: QRImageType) -> String {
        let matrix = self.matrix();
        match image {
            QRImageType::ASCII => matrix.ascii_image(),
            QRImageType::SVG => matrix.svg_image(),
        }
    }

    pub fn matrix(&self) -> QRMatrix {
        let size = self.0.size();
        let size_u = size as usize;
        let mut rows = Vec::with_capacity(size_u);
        for y in 0..size {
            let mut row = Vec::with_capacity(size_u);
            for x in 0..size {
                row.push(self.0.get_module(x, y));
            }
            rows.push(row);
        }
        QRMatrix(rows)
    }
}

pub type AnyResult<T=(),E=Box<dyn std::error::Error>> = Result<T,E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QRImageType {
    ASCII,
    SVG,
}

impl QRMatrix {
    pub fn ascii_image(&self) -> String {
        let rows: Vec<String> = self.0.iter().map(|row| {
            row.iter().map(|&b| if b { "██" } else { "  " }).collect()
        }).collect();
        rows.join("\n")
    }
    pub fn svg_image(&self) -> String {
        let size = self.0.len();
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" shape-rendering=\"crispEdges\" width=\"{}\" height=\"{}\" >\n",
            &size, &size, &size, &size
        );
        let rows = self.0.iter().enumerate().map(|(y, row)| {
            let mut row_svg = String::new();
            for (x, &b) in row.iter().enumerate() {
                if b {
                    row_svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"1\" height=\"1\" fill=\"black\" />\n", x, y));
                }
            }
            row_svg
        }).collect::<Vec<String>>().join("");
        svg.push_str(&rows);
        svg.push_str("</svg>");
        svg
    }
}

impl std::fmt::Debug for QR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.matrix())
    }
}

impl std::fmt::Debug for QRMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
