# Percepto

*Percepto* is a minimal, no‑nonsense **image‑hashing** crate written in Rust.  Its goal is to provide a set of fast, well‑tested perceptual hashing algorithms that can be used for near‑duplicate image detection, meme‑repost hunting, cache‑key generation, or any task where "do these two pictures look the same?" is the question.

> **Status:** early prototype – API is still in flux.  Only **average hash (aHash)** is implemented as of today; **dHash, pHash, and wHash** are on the short‑ term roadmap.

---

## ✨ Features

| Feature                 | Implemented | Notes                          |
| ----------------------- | ----------- | ------------------------------ |
| Average Hash (aHash)    | ✅           | 8 × 8 grayscale grid           |
| Perceptual Hash (pHash) | 🚧          | DCT‑based 64‑bit signature     |
| Difference Hash (dHash) | 🚧          | Horizontal & vertical variants |
| Wavelet Hash (wHash)    | 🚧          | Haar wavelet transform         |

---

## 🚀 Quick start

```bash
# Clone & build
$ git clone https://github.com/your‑name/percepto.git
$ cd percepto
$ cargo test
```

### Example: hashing two files & computing distance

```rust
use percepto::{AHash, HashingParameters, hamming_distance};

fn main() -> anyhow::Result<()> {
    // 1)  standard 8×8 grayscale params
    let params = HashingParameters::for_ahash();

    // 2)  generate hashes
    let test_png  = AHash::from_image_path("tests/test_image.png",  params)?;
    let cropped_jpg = AHash::from_image_path("tests/test_image_cropped.jpg", params)?;

    // 3)  compare
    let d = hamming_distance(test_png.value, cropped_jpg.value)?;
    println!("Hamming distance: {d}");
    Ok(())
}
```

---

## 📄 License

Licensed under the MIT License.

---

