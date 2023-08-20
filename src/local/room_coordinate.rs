use std::{convert::TryFrom, error::Error, fmt};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::ROOM_SIZE;

pub(crate) const ROOM_AREA: usize = (ROOM_SIZE as usize) * (ROOM_SIZE as usize);

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError(u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds coordinate: {}", self.0)
    }
}

impl Error for OutOfBoundsError {}

#[inline]
#[must_use]
pub const fn xy_to_linear_index(xy: RoomXY) -> usize {
    ((xy.x.0 as usize) * (ROOM_SIZE as usize)) + (xy.y.0 as usize)
}

#[inline]
#[must_use]
pub fn linear_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {idx}");
    RoomXY {
        // SAFETY: assert! bounds checking above ensures coord within range.
        x: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_SIZE as usize)) as u8) },
        // SAFETY: assert! bounds checking above ensures coord within range.
        y: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_SIZE as usize)) as u8) },
    }
}

// The serde try_from u8 impl ensures safety in Deserialize.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(try_from = "u8", into = "u8")]
pub struct RoomCoordinate(u8);

impl RoomCoordinate {
    #[inline]
    pub const fn new(coord: u8) -> Result<Self, OutOfBoundsError> {
        if coord < ROOM_SIZE {
            Ok(RoomCoordinate(coord))
        } else {
            Err(OutOfBoundsError(coord))
        }
    }

    /// # Safety
    /// Calling this method with `coord >= ROOM_SIZE` can result in undefined
    /// behaviour when the resulting `RoomCoordinate` is used.
    #[inline]
    #[must_use]
    pub unsafe fn unchecked_new(coord: u8) -> Self {
        debug_assert!(
            coord < ROOM_SIZE,
            "Out of bounds unchecked coordinate: {coord}"
        );
        RoomCoordinate(coord)
    }

    #[must_use]
    pub const fn u8(self) -> u8 {
        self.0
    }
}

impl fmt::Display for RoomCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct RoomXY {
    pub x: RoomCoordinate,
    pub y: RoomCoordinate,
}

impl RoomXY {
    /// # Safety
    /// Calling this method with `x >= ROOM_SIZE` or `y >= ROOM_SIZE` can result
    /// in undefined behaviour when the resulting `RoomXY` is used.
    #[inline]
    #[must_use]
    pub unsafe fn unchecked_new(x: u8, y: u8) -> Self {
        RoomXY {
            x: RoomCoordinate::unchecked_new(x),
            y: RoomCoordinate::unchecked_new(y),
        }
    }
}

impl fmt::Display for RoomXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<RoomCoordinate> for u8 {
    fn from(coord: RoomCoordinate) -> u8 {
        coord.0
    }
}

impl TryFrom<u8> for RoomCoordinate {
    type Error = OutOfBoundsError;

    fn try_from(coord: u8) -> Result<Self, Self::Error> {
        RoomCoordinate::new(coord)
    }
}

impl From<RoomXY> for (u8, u8) {
    fn from(xy: RoomXY) -> (u8, u8) {
        (xy.x.0, xy.y.0)
    }
}

impl TryFrom<(u8, u8)> for RoomXY {
    type Error = OutOfBoundsError;

    fn try_from(xy: (u8, u8)) -> Result<RoomXY, OutOfBoundsError> {
        Ok(RoomXY {
            x: RoomCoordinate::try_from(xy.0)?,
            y: RoomCoordinate::try_from(xy.1)?,
        })
    }
}

impl From<(RoomCoordinate, RoomCoordinate)> for RoomXY {
    fn from(xy: (RoomCoordinate, RoomCoordinate)) -> RoomXY {
        RoomXY { x: xy.0, y: xy.1 }
    }
}

impl From<RoomXY> for (RoomCoordinate, RoomCoordinate) {
    fn from(xy: RoomXY) -> (RoomCoordinate, RoomCoordinate) {
        (xy.x, xy.y)
    }
}

#[derive(Serialize, Deserialize)]
struct ReadableXY {
    x: RoomCoordinate,
    y: RoomCoordinate,
}

impl From<ReadableXY> for RoomXY {
    fn from(ReadableXY { x, y }: ReadableXY) -> RoomXY {
        RoomXY { x, y }
    }
}

impl From<RoomXY> for ReadableXY {
    fn from(RoomXY { x, y }: RoomXY) -> ReadableXY {
        ReadableXY { x, y }
    }
}

impl Serialize for RoomXY {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            ReadableXY::from(*self).serialize(serializer)
        } else {
            let xy: (u8, u8) = (*self).into();
            let packed: u16 = (u16::from(xy.0) << 8) | u16::from(xy.1);
            packed.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for RoomXY {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            ReadableXY::deserialize(deserializer).map(Into::into)
        } else {
            let packed = u16::deserialize(deserializer)?;
            let xy = (((packed >> 8) & 0xFF) as u8, (packed & 0xFF) as u8);
            RoomXY::try_from(xy).map_err(|err: OutOfBoundsError| {
                de::Error::invalid_value(
                    de::Unexpected::Unsigned(u64::from(err.0)),
                    &format!("a non-negative integer less-than {ROOM_SIZE}").as_str(),
                )
            })
        }
    }
}
