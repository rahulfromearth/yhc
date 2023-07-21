/* the module system works as follows.
When the module is first loaded it first scans the string table and remembers
for each string id where the string can be loaded in the file.

It then scans each object reference, it reads the name id and loads the string for that
name. It then stores the object name against the object offset in the lookup table and skips
the rest of the object (using the size information for each object).

Now when something wants to resolve an object it looks up the name in the lookup table, that gives
it an object reference id. It then looks at the corresponding object reference, if the object is
already loaded it gives that otherwise it jumps to the appropriate place in the bytecode file and
loads the object, and then updates the object reference to note that it's been loaded */

//C #include "hashtable.h"
//C #include "node.h"
//C #include "heap.h"

/* module version numbers, should correspond with what the compiler produces */
//C #define VERSION_MAJOR 1
//C #define VERSION_MINOR 10

/* Which byte marks something as being a selector */
//C #define SELECTOR_INS            0

use libc;

use crate::{
    hashtable::Hashtable,
    heap::Global,
    node::{Info, Node},
    platform::platform::UInt,
};

/* an object is something loaded fully from the bytecode file, with info being the info
and node being the corresponding caf/zcon */
struct Object<'a> {
    info: &'a Info,
    node: &'a Node,
    global: Global<'a>,
}

/* an object reference is a possibly not yet loaded reference into the bytecode file */
struct ObjectRef<'a> {
    object: &'a Object<'a>,
    offset: UInt,
}

/* a string reference is a possibly not yet loaded string in the string table */
struct StringRef {
    string: String,
    offset: UInt,
}

/* a module stores information necessary in loading the bytecode */
pub struct Module<'a> {
    file: std::fs::File, /* file this module was loaded from */
    name: String,        /* the qualified name of this module */
    path: String,        /* the path this module was loaded from, excluding extension */
    numObjects: UInt,    /* the number of objects in the module */
    numStrings: UInt,    /* the number of strings in the module */

    strings: &'a StringRef,     /* string reference table */
    objects: &'a ObjectRef<'a>, /* object reference table */

    lookup: &'a Hashtable<'a>, /* lookup, maps object names to object references */

    external: *mut libc::c_void, /* external, non-null if this module has a dynamically linked
                                 external counterpart */
                                 //C   #ifdef HAT
                                 //C          hatModule: HModule*;
                                 //C   #endif
}
