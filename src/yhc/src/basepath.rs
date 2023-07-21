/* this is here because guessing the YHC_BASE_PATH is somewhat involved:

     - if argv[0] was qualified then we should try relative to that

     - otherwise we should search the PATH environment variable
       this is separated differently on different operating systems
*/

//C #include "platform.h"
//C #include "iofuncs.h" file_exists, file_basename
//C #include "main.h"

//C #ifdef WIN32
//C # define SEPARATOR ';'
//C #else
//C # define SEPARATOR ':'
//C #endif

const SEPARATOR: char = ':';

use std::path::Path;

/* return None if you don't find it */
fn basepath_check(directory: &str, suffix: &str) -> Option<String> {
    let search = format!(
        "{}/{}{}",
        directory,
        unsafe { crate::G_progName.as_ref().unwrap() },
        suffix
    );
    println!("Trying to check {}", search); // TODO delete
    let res = Path::new(&search).exists();
    if !res {
        None
    } else {
        Some(format!("{}/..", directory))
    }
}

/* guess the basepath */
fn basepath_guess() -> Option<String> {
    /* check whether argv[0] included some kind of path */
    let progBase = Path::new(unsafe { crate::G_progName.as_ref().unwrap() })
        .parent()
        .unwrap();

    if progBase != Path::new(".") || progBase != Path::new("") {
        return Some(format!("{}/..", progBase.display()));
    }

    /* otherwise search the PATH environment variable for G_progName ... */
    let paths = std::env::var("PATH").unwrap();
    let mut res: Option<String> = None;

    for path in paths.split(SEPARATOR) {
        /* search for progname in that directory */
        res = basepath_check(path, "");
        if res.is_some() {
            return res;
        }

        //C #ifdef WIN32
        //C     res = basepath_check(last, ".exe");
        //C     if res.is_some() {
        //C         return res;
        //C     }
        //C #endif
    }

    /* not found! */
    return res;
}

/* get the yhc base path */
pub fn basepath_get() -> Option<String> {
    /* try the environment var first ... */
    let env_var = std::env::var("YHC_BASE_PATH");

    if env_var.is_ok() {
        return Some(env_var.unwrap());
    }

    /* otherwise guess ... */
    return basepath_guess();
}
