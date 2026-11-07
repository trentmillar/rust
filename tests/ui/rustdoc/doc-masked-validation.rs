//@ compile-flags: --crate-type lib
#![feature(doc_masked)]

// Valid extern crate target
#[doc(masked)]
pub extern crate std as my_std;

// Invalid targets with relevant lint levels
#[deny(invalid_doc_attributes)]
#[doc(masked)]
//~^ ERROR this attribute can only be applied to an `extern crate` item
pub fn fn_with_masked() {}

#[allow(invalid_doc_attributes)]
#[doc(masked)]
pub fn fn_allowed() {}

#[deny(invalid_doc_attributes)]
#[doc(masked)]
//~^ ERROR this attribute can only be applied to an `extern crate` item
pub struct StructWithMasked;

// Disallowed on `extern crate self`
#[deny(invalid_doc_attributes)]
#[doc(masked)]
//~^ ERROR this attribute cannot be applied to an `extern crate self` item
pub extern crate self as reexport;

// Malformed arguments without extra validation diagnostics
#[doc(masked = "foo")]
//~^ WARN didn't expect any arguments here
//~| WARN this was previously accepted
pub fn malformed_fn() {}
