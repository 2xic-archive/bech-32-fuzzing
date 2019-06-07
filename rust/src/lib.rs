// Copyright (c) 2017 Clark Moody
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.



#![warn(missing_docs)]


pub mod bech32;

/// Error types for Bech32 encoding / decoding
#[derive(PartialEq, Debug)]
pub enum CodingError {
	/// String does not contain the separator character
	MissingSeparator,
	/// The checksum does not match the rest of the data
	InvalidChecksum,
	/// The data or human-readable part is too long or too short
	InvalidLength,
	/// Some part of the string contains an invalid character
	InvalidChar,
	/// Some part of the data has an invalid value
	InvalidData,
	/// The whole string must be of one case
	MixedCase,
}

/// Error types for validating scriptpubkeys
#[derive(PartialEq, Debug)]
pub enum ScriptPubKeyError {
	/// scriptpubkeys does not have enough data
	TooShort,
	/// The provided length byte does not match the data
	InvalidLengthByte,
}

/// Error types for witness programs
///
/// BIP141 specifies Segregated Witness and defines valid program lengths
/// for Version 0 scripts. Script version is also limited to values 0-16.
#[derive(PartialEq, Debug)]
pub enum WitnessProgramError {
	/// Denotes that the WitnessProgram is too long or too short
	///
	/// Programs must be between 2 and 40 bytes
	InvalidLength,
	/// Given the program version, the length is invalid
	///
	/// Version 0 scripts must be either 20 or 32 bytes
	InvalidVersionLength,
	/// Script version must be 0 to 16 inclusive
	InvalidScriptVersion,
}

/// Error types during bit conversion
#[derive(PartialEq, Debug)]
pub enum BitConversionError {
	/// Input value exceeds "from bits" size
	InvalidInputValue(u8),
	/// Invalid padding values in data
	InvalidPadding,
}

/// Error types while encoding and decoding SegWit addresses
#[derive(PartialEq, Debug)]
pub enum AddressError {
	/// Some Bech32 conversion error
	Bech32(CodingError),
	/// Some witness program error
	WitnessProgram(WitnessProgramError),
	/// Some 5-bit <-> 8-bit conversion error
	Conversion(BitConversionError),
	/// The provided human-readable portion does not match
	HumanReadableMismatch,
	/// The human-readable part is invalid (must be "bc" or "tb")
	InvalidHumanReadablePart,
}



extern crate libc;
use libc::c_char;
use libc::strlen;
use std::ffi::CStr;


#[no_mangle]
pub extern fn bech32_decode_rust(s: *const c_char) -> i32 {
	let c_str = unsafe {
		assert!(!s.is_null());
		CStr::from_ptr(s)
	};
//	println!("{:?}", unsafe {strlen(s) } );
	let str_buf: String = String::from_utf8_lossy(c_str.to_bytes()).into_owned() ; 
	let dec_result = bech32::Bech32::from_string(str_buf, unsafe {strlen(s)} as i32);
	return dec_result as i32;
}




