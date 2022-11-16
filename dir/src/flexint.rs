const MAX_SIZE: usize = 10;

/// Provides encoding/decoding U64 in a flex-int format
#[derive(Clone, Debug)]
pub(crate) struct FlexIntEncoding {
    buf: [u8; MAX_SIZE],
    used: usize,
}

impl FlexIntEncoding {
    pub(crate) fn as_slice(&self) -> &[u8] {
        assert!(self.used <= MAX_SIZE);
        &self.buf[..self.used]
    }
}

impl From<u64> for FlexIntEncoding {
    fn from(mut u: u64) -> Self {
        let mut fei = FlexIntEncoding {
            buf: [0; MAX_SIZE],
            used: 0,
        };

        if u == 0 {
            fei.used = 1;
        } else {
            while u > 0 {
                let flagbit: u8 = if (u >> 7) > 0 { 0x80 } else { 0x00 };
                fei.buf[fei.used] = flagbit | ((u & 0x7f) as u8);
                fei.used += 1;
                u >>= 7;
            }
        }

        fei
    }
}

#[derive(Debug)]
pub(crate) struct U64DecodeError {
    #[allow(dead_code)] // Only used for Debug display:
    input: FlexIntEncoding,
    #[allow(dead_code)] // Only used for Debug display:
    reason: U64DecodeErrorReason,
}

#[derive(Debug)]
pub(crate) enum U64DecodeErrorReason {
    Overflow,
    MissingContinuationBit,
    UnexpectedContinuationBit,
}

impl TryFrom<FlexIntEncoding> for u64 {
    type Error = U64DecodeError;

    fn try_from(fei: FlexIntEncoding) -> Result<u64, Self::Error> {
        u64::try_from(&fei)
    }
}

impl<'a> TryFrom<&'a FlexIntEncoding> for u64 {
    type Error = U64DecodeError;

    fn try_from(fei: &'a FlexIntEncoding) -> Result<u64, Self::Error> {
        u64_try_from_fei(fei).map_err(|reason| U64DecodeError {
            reason,
            input: fei.clone(),
        })
    }
}

fn u64_try_from_fei(fei: &FlexIntEncoding) -> Result<u64, U64DecodeErrorReason> {
    let high_bit_set = |b| b & 0x80 == 0x80;

    let mut u: u64 = 0;

    let slice = fei.as_slice();
    for (i, &b) in slice.iter().enumerate() {
        use U64DecodeErrorReason::*;

        dbg!(i, slice.len(), b);
        if i + 1 == MAX_SIZE && b > 0x01 {
            return Err(Overflow);
        } else if i + 1 == slice.len() {
            if high_bit_set(b) {
                return Err(UnexpectedContinuationBit);
            }
        } else if !high_bit_set(b) {
            return Err(MissingContinuationBit);
        }

        u |= ((b & 0x7f) as u64) << (i * 7);
    }

    Ok(u)
}

#[derive(Debug, derive_more::From)]
pub(crate) enum FromSliceError {
    SliceTooLong,
    Overflow(U64DecodeError),
}

impl TryFrom<&[u8]> for FlexIntEncoding {
    type Error = FromSliceError;

    fn try_from(slice: &[u8]) -> Result<FlexIntEncoding, Self::Error> {
        let used = slice.len();
        if used <= MAX_SIZE {
            let mut buf = [0; MAX_SIZE];
            buf[..used].copy_from_slice(slice);
            let fei = FlexIntEncoding { buf, used };
            // Check for overflow:
            let _ = u64::try_from(&fei)?;
            Ok(fei)
        } else {
            Err(FromSliceError::SliceTooLong)
        }
    }
}

#[cfg(test)]
mod tests;
