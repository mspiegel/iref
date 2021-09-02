use super::Error;
use crate::parsing;
use pct_str::PctStr;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::{cmp, fmt};

#[derive(Clone, Copy)]
pub struct Segment<'a> {
	/// The path segment slice.
	pub(crate) data: &'a [u8],

	pub(crate) open: bool,
}

impl<'a> Segment<'a> {
	/// The special dot segment `.` indicating the current directory.
	#[inline]
	pub fn current() -> Segment<'static> {
		Segment {
			data: b".",
			open: false,
		}
	}

	/// The special dot segment `..` indicating the parent directory.
	#[inline]
	pub fn parent() -> Segment<'static> {
		Segment {
			data: b"..",
			open: false,
		}
	}

	/// Get the length of the path name.
	#[inline]
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns a reference to the byte representation of the segment.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data
	}

	/// Get the underlying segment slice as a string slice.
	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.data) }
	}

	/// Get the underlying segment slice as a string slice by consuming the segment reference.
	#[inline]
	pub fn into_str(self) -> &'a str {
		unsafe { std::str::from_utf8_unchecked(self.data) }
	}

	/// Get the underlying segment slice as a percent-encoded string slice.
	#[inline]
	pub fn as_pct_str(&self) -> &PctStr {
		unsafe { PctStr::new_unchecked(self.as_str()) }
	}

	#[inline]
	pub fn is_open(&self) -> bool {
		self.open
	}

	/// Checks if the segment is empty.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	/// Open that path.
	#[inline]
	pub fn open(&mut self) {
		self.open = true
	}
}

impl<'a> AsRef<[u8]> for Segment<'a> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<'a> TryFrom<&'a str> for Segment<'a> {
	type Error = Error;

	#[inline]
	fn try_from(str: &'a str) -> Result<Segment<'a>, Error> {
		let segment_len = parsing::parse_path_segment(str.as_ref(), 0)?;
		let data: &[u8] = str.as_ref();
		if segment_len < data.len() {
			if segment_len == data.len() - 1 && data[segment_len] == b'/' {
				Ok(Segment {
					data: &data[0..segment_len],
					open: true,
				})
			} else {
				Err(Error::InvalidSegment)
			}
		} else {
			Ok(Segment { data, open: false })
		}
	}
}

impl<'a> fmt::Display for Segment<'a> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.open {
			write!(f, "{}/", self.as_str())
		} else {
			self.as_str().fmt(f)
		}
	}
}

impl<'a> fmt::Debug for Segment<'a> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.open {
			write!(f, "{}/", self.as_str())
		} else {
			self.as_str().fmt(f)
		}
	}
}

impl<'a> cmp::PartialEq for Segment<'a> {
	#[inline]
	fn eq(&self, other: &Segment) -> bool {
		self.open == other.open && self.as_pct_str() == other.as_pct_str()
	}
}

impl<'a> Eq for Segment<'a> {}

impl<'a> cmp::PartialEq<&'a str> for Segment<'a> {
	#[inline]
	fn eq(&self, other: &&'a str) -> bool {
		self.as_pct_str() == *other
	}
}

impl<'a> PartialOrd for Segment<'a> {
	#[inline]
	fn partial_cmp(&self, other: &Segment<'a>) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<'a> Ord for Segment<'a> {
	#[inline]
	fn cmp(&self, other: &Segment<'a>) -> Ordering {
		self.as_pct_str().cmp(other.as_pct_str())
	}
}

impl<'a> Hash for Segment<'a> {
	#[inline]
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_pct_str().hash(hasher)
	}
}
