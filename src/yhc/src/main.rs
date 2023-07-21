#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

// https://www.reddit.com/r/rust/comments/cfybfa/statically_borrowing_command_line_arguments/
// https://stackoverflow.com/questions/26378842/how-do-i-overcome-match-arms-with-incompatible-types-for-structs-implementing-sa

mod basepath;
mod hashtable;
mod heap;
mod module;
mod node;
mod platform;
mod types;

use libc;
use platform::platform::{Int64, UInt};
use std::process;
use types::types::Bool;

use std::ffi::CStr;

// main.h

const DEFAULT_HEAP_SIZE: u64 = 16 * 1024 * 1024;
const DEFAULT_MAX_STACK_SIZE: u64 = 1 * 1024 * 1024;

#[derive(Copy, Clone, Debug)]
struct Options {
    heapSize: UInt,
    maxStackSize: UInt,
    stats: Bool,
    pretty: Bool,
}

/* global variable storing the program name and arguments */
static mut G_argc: usize = 0;
static mut G_argv: Option<Vec<String>> = None;
static mut G_progName: Option<String> = None;
static mut G_options: Option<Options> = None;
static mut G_yhcBasePath: Option<String> = None; // Char*

static mut G_insCount: Option<Int64> = None;

//C #ifndef VERSION
//C # define VERSION "Unversioned"
//C #endif

static VERSION: &str = "UNVERSIONED";

fn version() {
    eprintln!("yhi: The York Haskell Interpreter version {}", VERSION);
}

/* give usage information and exit */
fn usage() {
    version();
    //C #ifdef HAT
    //C   eprintln!("\t\tCompiled with Hat support");
    //C #endif

    // println!("DEBUG: {:?}\n {:?}", DEFAULT_HEAP_SIZE, unsafe {
    //     G_options.assume_init()
    // });

    eprintln!("\nusage: {} [options] classfile [args]", unsafe {
        G_progName.as_ref().unwrap()
    });

    eprintln!("");
    eprintln!("  classfile            - name of haskell class file to execute");
    eprintln!("  args                 - arguments to pass to haskell program");
    eprintln!("");
    eprintln!("options:");
    eprintln!("  -h --heap size       - set the heap size in bytes, e.g -h 10M");
    eprintln!("  -s --stack size      - set the maximum stack size in bytes, e.g. -s 1M");
    eprintln!("  -stats               - print statistics on the execution");
    eprintln!("  -v --version         - print the Yhi version then exit");
    eprintln!("  -dump                - don't execute bytecode, just print it and exit");
    eprintln!("");

    process::exit(1);
}

/* parse a size argument */
// Int
fn parseSize(arg: String, p: String) -> u64 {
    let mut endptr: *mut libc::c_char = std::ptr::null_mut();

    // Int ret = (Int)strtoul(p, &end, 10);
    let ret = unsafe { libc::strtoul(p.as_ptr() as *const libc::c_char, &mut endptr, 10) };
    let end = unsafe {
        CStr::from_ptr(endptr)
            .to_str()
            .expect("Invalid UTF-8 sequence")
    };

    if end == p {
        eprintln!("ERROR: expected size argument after '{}'", arg);
        usage();
    }

    // Int mult = 0;

    let mult = match end {
        "b" => 1,
        "K" => 1024,
        "M" => 1024 * 1024,
        "G" => 1024 * 1024 * 1024,
        _ => {
            eprintln!("ERROR: unknown size argument '{}' after '{}'", p, arg);
            eprintln!("       expected:  digit*(b|K|M|G)");
            eprintln!("           e.g. 3000b, 10K, 12M, 1G");
            usage();
            unreachable!()
        }
    };

    if end.len() != 1 {
        eprintln!(
            "ERROR: unexpected '{}' after size argument '{}'",
            &end[1..],
            p
        );
        usage();
    }

    ret * mult
}

/* parse all the arguments */
fn parseArgs() -> String {
    let mut mainMod = String::new();

    /* initialize options */
    unsafe { G_progName = Some(std::env::args().nth(0).unwrap()) };

    let mut parsedOptions = Options {
        heapSize: DEFAULT_HEAP_SIZE,
        maxStackSize: DEFAULT_MAX_STACK_SIZE,
        stats: false,
        pretty: false,
    };

    /* parse arguments */

    let mut i: usize = 1;

    while i < std::env::args().len() {
        let arg = std::env::args().nth(i).unwrap();
        let next = std::env::args().nth(i + 1);

        if !arg.starts_with("-") {
            mainMod = arg;
            i += 1;
            break;
        }

        if arg == "-h" || arg == "--heap" {
            if next.is_none() {
                eprintln!("ERROR: expected heap size after option {}", arg);
                usage();
            }
            parsedOptions.heapSize = parseSize(arg, next.unwrap());
            i += 1; // skip size
        } else if arg == "-s" || arg == "--stack" {
            if next.is_none() {
                eprintln!("ERROR: expected stack size after option {}", arg);
                usage();
            }
            parsedOptions.maxStackSize = parseSize(arg, next.unwrap());
            i += 1; // skip size
        } else if arg == "-stats" {
            parsedOptions.stats = true;
        } else if arg == "-dump" {
            parsedOptions.pretty = true;
        } else if arg == "-v" || arg == "-version" || arg == "--version" {
            version();
            process::exit(0);
        } else {
            eprintln!("WARNING: ignored unknown flag '{}'", arg);
        }

        i += 1;
    }

    unsafe { G_options = Some(parsedOptions) }

    /* check we have a main module */
    if mainMod.is_empty() {
        usage();
    }

    /* store argument information */
    unsafe { G_argc = std::env::args().len() - i }

    unsafe {
        G_argv = Some((&std::env::args().collect::<Vec<String>>()[i..]).to_vec());
    }

    println!("{:?}", unsafe {
        G_argv
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    });

    /* get yhc base path */
    unsafe { G_yhcBasePath = basepath::basepath_get() }

    return mainMod;
}

/* initialize program */
fn init(
    mainMod: String,
    mainFunc: &mut Option<node::Node>,
    _toplevel: &mut Option<node::Node>,
    _driver: &mut Option<node::FInfo>,
) {
    /* inits */
    // sanity_init();

    heap::heap_init(unsafe { G_options.heapSize });

    //C   #ifdef HAT
    //C     hgm_init(mainMod);
    //C   #endif

    // mod_init();

    /* load all globals */
    // initGlobals(mainMod, mainFunc, _toplevel, _driver);

    /* initialize the threads system */
    // yhi_thread_init();
    // hsffi_init();

    /* finished with the module system now */
    /* mod_exit(); ... not any more, now we still need it! */
}

// main.c
fn main() {
    /* parse program arguments */
    let mainMod = parseArgs();

    let mut _toplevel: Option<node::Node> = None;
    let mut mainFunc: Option<node::Node> = None;
    let mut _driver: Option<node::FInfo> = None;

    /* initialize */
    init(mainMod, &mut mainFunc, &mut _toplevel, &mut _driver);

    // println!("{}", parseSize(String::from("--heap"), String::from("2K")));
    // "ABCD", "1A", "2b", "3.6G"
}
