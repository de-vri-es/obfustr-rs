use core::marker::PhantomData;

#[doc(hidden)]
pub use obfustr_macros as macros__;

#[macro_export]
macro_rules! obfuscate {
	($($tokens:tt)*) => {
		const {
			let (marker, data) = $crate::macros__::obfuscate_raw!($($tokens)*);
			unsafe { $crate::Obfuscated::new_unchecked(marker, data) }
		}.decrypt().as_inner()
	};
}

/// A slice of obfuscated string data.
#[repr(transparent)]
pub struct Obfuscated<T: ?Sized> {
	marker: PhantomData<T>,
	data: [u16],
}

/// A decrypted string, which owns its data.
///
/// Overwrites the data with zeroes when dropped.
pub struct Decrypted<T: ?Sized> {
	marker: PhantomData<T>,
	data: Box<[u8]>,
}

impl<T: ?Sized> Obfuscated<T> {
	/// Create a new obfuscated string from already encrypted data.
	///
	/// # Safety
	/// The data must decrypt to valid UTF-8.
	#[doc(hidden)]
	pub const unsafe fn new_unchecked(_: PhantomData<T>, data: &[u16]) -> &Self {
		core::mem::transmute(data)
	}

	/// Decrypt the obfuscated string.
	pub fn decrypt(&self) -> Decrypted<T> {
		let mut data = Box::new_uninit_slice(self.data.len());
		for i in 0..self.data.len() {
			// Use read_volatile to avoid the compiler from optimizing away the obfuscation.
			// SAFETY: we read from a pointer directly created from  a reference, so the pointer must be valid.
			let elem = unsafe { std::ptr::read_volatile(&self.data[i]) };
			let [a, b] = elem.to_le_bytes();
			data[i].write(a ^ b);
		}

		// SAFETY: we just wrote to every single byte in `data`.
		let data = unsafe { data.assume_init() };

		Decrypted {
			marker: self.marker,
			data,
		}
	}
}

impl<T: ?Sized + Data> Decrypted<T> {
	/// Get the decrypted data.
	pub fn as_inner(&self) -> &T {
		// SAFETY: We are only ever constructed from an Obfuscated<T>,
		// which guarantees us that the decrypted data is valid for the target type.
		unsafe {
			Data::from_raw(&self.data)
		}
	}
}

impl<T: ?Sized> Drop for Decrypted<T> {
	fn drop(&mut self) {
		for byte in &mut self.data {
			// Use write_volatile to ensure the zero-ing isn't optimized out.
			// SAFETY: We know all the elements of the slice are valid.
			unsafe {
				core::ptr::write_volatile(byte, 0);
			}
		}
	}
}

impl<T: ?Sized + Data> std::ops::Deref for Decrypted<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.as_inner()
	}
}

impl<T: ?Sized + Data> AsRef<T> for Decrypted<T> {
	fn as_ref(&self) -> &T {
		self.as_inner()
	}
}

impl<T: ?Sized + Data> std::borrow::Borrow<T> for Decrypted<T> {
	fn borrow(&self) -> &T {
		self.as_inner()
	}
}

impl<T: ?Sized + Data + std::fmt::Display> std::fmt::Display for Decrypted<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.as_inner(), f)
	}
}

impl<T: ?Sized + Data + std::fmt::Debug> std::fmt::Debug for Decrypted<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self.as_inner(), f)
	}
}

pub trait Data {
	/// Reinterpret the data from a raw byte slice.
	///
	/// # Safety
	/// The data must be valid according to the rules of `Self`.
	unsafe fn from_raw(data: &[u8]) -> &Self;
}

impl Data for str {
	unsafe fn from_raw(data: &[u8]) -> &Self {
		::core::str::from_utf8_unchecked(data)
	}
}

impl Data for [u8] {
	unsafe fn from_raw(data: &[u8]) -> &Self {
		data
	}
}

impl Data for core::ffi::CStr {
	unsafe fn from_raw(data: &[u8]) -> &Self {
		Self::from_bytes_with_nul_unchecked(data)
	}
}
