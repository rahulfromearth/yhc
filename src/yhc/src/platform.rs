/*----------------------------------------------------------------------------------------
platform specific definitions should go here
----------------------------------------------------------------------------------------*/

static PLATFORM_H_VERSION: u32 = 0x0100;

//C #include <config.h>

/* #define NO_LIBFFI */
/* #define NO_LIBGMP */
/* #define NO_SHARED */

//C  #if !defined(WIN32) /* windows doesn't need this check */
//C  #  if CONFIG_H_VERSION != PLATFORM_H_VERSION
//C  #    error "config.h is the wrong version. Please re-run configure"
//C  #  endif
//C  #endif

/* VS 2005 started being "security" concious */
//C  #if defined(_MSC_VER) && (_MSC_VER >= 1400)
//C  #  define _CRT_SECURE_NO_DEPRECATE
//C  #endif

/* type synonyms for basic types */
type Char = char; /* could be Int16 for unicode support .. */

/* native byte sizes, should always be a single byte */
type Byte = i8;
type UByte = u8;

/* native int and word types, these *MUST* be the same as the size of a void* */

// https://doc.rust-lang.org/stable/reference/conditional-compilation.html#target_pointer_width

#[cfg(target_pointer_width = "64")]
pub mod platform {

    // config.h
    pub type WORD_TYPE = i64;
    pub type HALF_TYPE = i32;
    pub type QUARTER_TYPE = i16;

    /* fixed size types, always the same size. Alias to different things on different platforms */
    // signed INT*_TYPE
    pub type Int8 = i8;
    pub type Int16 = i16;
    pub type Int32 = i32;
    pub type Int64 = i64;

    // unsigned INT*_TYPE
    pub type UInt8 = u8;
    pub type UInt16 = u16;
    pub type UInt32 = u32;
    pub type UInt64 = u64;

    // FLOAT*_TYPE
    pub type Float32 = f32;
    pub type Float64 = f64;

    /* native byte sizes, should always be a single byte */
    pub type Byte = i8; // Int8
    pub type UByte = u8; // UInt8

    /* native int and word types, these *MUST* be the same as the size of a void* */
    pub type Int = i64;
    pub type UInt = u64;
    pub type Word = u64;

    /* native half int sizes, *MUST* be half the size of native int, whatever that is */
    pub type HInt = i32;
    pub type HUInt = u32;
    pub type HWord = u32;

    /* native quarter int sizes, *MUST* be 1/4 the size of a native int */
    pub type QInt = i16; // QUARTER_TYPE
    pub type QUInt = u16; // unsigned QUARTER_TYPE
    pub type QWord = u16; // unsigned QUARTER_TYPE
}

/* native half int sizes, *MUST* be half the size of native int, whatever that is */
//C type HALF_TYPE           HInt;
//C type unsigned HALF_TYPE  HUInt;
//C type unsigned HALF_TYPE  HWord;

/* native quarter int sizes, *MUST* be 1/4 the size of a native int */
//C type QUARTER_TYPE            QInt;
//C type unsigned QUARTER_TYPE   QUInt;
//C type unsigned QUARTER_TYPE   QWord;

/* native double int size, typically twice the size of a normal int

REMOVED: it's confusing to talk about 'double ints' when they aren't on 64bit platforms
type Int64               DInt;
type UInt64              DUInt;
type UInt64              DWord;
*/

/* native floating point types */
//C type float              Float;
//C type double             Double;

/* endianness */
//C #ifdef WORDS_BIGENDIAN
//C # define IS_BIG_ENDIAN     1
//C #else
//C # define IS_BIG_ENDIAN     0
//C #endif

/* how many bytes there are in a native word */
//C #define WORD_BYTES       (1<<WORD_BYTES_SHIFT)
//C #define WORD_BYTES_MASK  (WORD_BYTES-1)

/* how many bits there are in a native word */
//C #define WORD_BITS        (1<<WORD_BITS_SHIFT)
//C #define WORD_BITS_MASK   (WORD_BITS-1)

/* how many bits there are in a byte */
//C #define BYTE_BITS_SHIFT  3
//C #define BYTE_BITS        (1<<BYTE_BITS_SHIFT)

/* standard headers */
//C #include <stdlib.h>
//C #include <assert.h>
//C #include <string.h>
//C #include <stdio.h>
//C #include <stdarg.h>
//C #include <math.h>
//C #ifndef NO_LIBGMP
//C #   include <gmp.h>
//C #endif
//C #include <float.h>
//C #include <sys/stat.h>
//C #include <sys/types.h>
//C #if defined (WIN32)
//C #  include "win32/dirent.h"
//C #else
//C #  include <dirent.h>
//C #  include <unistd.h>
//C #endif
//C #ifndef NO_LIBFFI
//C #  include <ffi.h>
//C #endif
//C #include <errno.h>
//C #include <time.h>

//C #if defined (WIN32)
//C #  define snprintf		_snprintf
//C #  define vsnprintf		_vsnprintf
//C #  define getcwd		_getcwd
//C #  define strdup        _strdup
//C #endif
