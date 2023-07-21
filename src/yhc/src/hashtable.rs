use std::sync::Arc;

use libc;

use crate::platform::platform::Int;

/* simple string hashtable used in module resolving */

/* a link in the hash chain */
struct HashLink<'a> {
    key: String,
    value: *mut libc::c_void,
    next: Arc<&'a HashLink<'a>>,
}

/* a hash table */
pub struct Hashtable<'a> {
    size: Int,
    threshhold: Int,
    entries: Int,
    table: &'a &'a HashLink<'a>,
}
