use bitvec::prelude::*;


pub fn hamming_distance(hash1: HashValue, hash2: HashValue) -> anyhow::Result<usize> {
    match (hash1, hash2) {
        (HashValue::Binary(vec1), HashValue::Binary(vec2)) => {
            if vec1.len() != vec2.len() {
                anyhow::bail!("Unequal vector lengths.")
            }
            Ok(vec1.iter().zip(vec2).filter(|(x, y)| *x != *y).count())
        }
    }
}

pub enum HashValue {
    Binary(BitVec),
}

pub struct AHash {
    pub value: HashValue,
}

pub struct HashingParameters {
    pub reduce_size: (u32, u32),
    pub reduce_color: ReduceColor,
}

pub enum ReduceColor {
    Grayscale,
}

impl HashingParameters {
    pub fn for_ahash() -> Self {
        Self {
            reduce_size: (8, 8),
            reduce_color: ReduceColor::Grayscale,
        }
    }
}

impl std::fmt::Display for HashValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashValue::Binary(value) => write!(
                f,
                "{}",
                value
                    .iter()
                    .map(|b| if *b { '1' } else { '0' })
                    .collect::<String>()
            ),
        }
    }
}

impl AHash {
    pub fn from_image(
        image: image::DynamicImage,
        params: HashingParameters,
    ) -> anyhow::Result<Self> {
        let (nwidth, nheight) = params.reduce_size;
        let image = image.resize_exact(nwidth, nheight, image::imageops::FilterType::Triangle);

        let image = match params.reduce_color {
            ReduceColor::Grayscale => image.grayscale(),
        };

        let bytes = image.as_bytes();
        let mean: f32 = bytes.iter().map(|x| *x as f32).sum::<f32>() / bytes.len() as f32;

        let mut bits: BitVec = BitVec::new();
        for byte in bytes {
            if *byte as f32 > mean {
                bits.push(true);
            } else {
                bits.push(false);
            }
        }

        Ok(Self {
            value: HashValue::Binary(bits),
        })
    }

    pub fn from_image_path(path: &str, params: HashingParameters) -> anyhow::Result<Self> {
        let img = image::ImageReader::open(path)?.decode()?;
        AHash::from_image(img, params)
    }
}
