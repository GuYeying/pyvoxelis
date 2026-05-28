use pyo3::prelude::*;

// -----------------------------------------------------------------------------
// Varint static tool class
// -----------------------------------------------------------------------------
#[pyclass(module = "io", name = "Varint", skip_from_py_object)]
#[derive(Clone, Copy)]
pub struct PyVarint;

#[pymethods]
impl PyVarint {
    // -------------------------------------------------------------------------
    // encode：u32 → bytes
    // -------------------------------------------------------------------------
    #[staticmethod]
    pub fn encode_u32(value: u32) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(5);
        let mut v = value;

        while v >= 0x80 {
            bytes.push((v as u8 & 0x7F) | 0x80);
            v >>= 7;
        }
        bytes.push(v as u8);

        bytes
    }

    // -------------------------------------------------------------------------
    // encode：usize → bytes
    // -------------------------------------------------------------------------
    #[staticmethod]
    pub fn encode(value: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        let mut v = value;

        while v >= 0x80 {
            bytes.push((v as u8 & 0x7F) | 0x80);
            v >>= 7;
        }
        bytes.push(v as u8);

        bytes
    }

    // -------------------------------------------------------------------------
    // Decode: bytes → u32 (Python passes bytes)
    // -------------------------------------------------------------------------
    #[staticmethod]
    pub fn decode_u32(data: &[u8]) -> Option<u32> {
        let mut iter = data.iter();
        let mut result = 0u32;
        let mut shift = 0;

        loop {
            let byte = *iter.next()?;
            result |= ((byte & 0x7F) as u32) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift > 28 {
                return None;
            }
        }

        Some(result)
    }

    // -------------------------------------------------------------------------
    // encode：bytes → usize
    // -------------------------------------------------------------------------
    #[staticmethod]
    pub fn decode(data: &[u8]) -> Option<usize> {
        let mut iter = data.iter();
        let mut result = 0usize;
        let mut shift = 0;

        loop {
            let byte = *iter.next()?;
            result |= ((byte & 0x7F) as usize) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift > 63 {
                return None;
            }
        }

        Some(result)
    }
}