#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

mod platform;
mod types;

use platform::platform::{Int64, UInt};
use std::mem::MaybeUninit;
use std::{env::Args, process::exit};
use types::types::Bool;

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
static mut G_argc: i32 = 0;
static mut G_argv: MaybeUninit<Args> = MaybeUninit::uninit();
static mut G_progName: MaybeUninit<&str> = MaybeUninit::uninit();
static mut G_options: MaybeUninit<Options> = MaybeUninit::uninit();
static mut G_yhcBasePath: MaybeUninit<&str> = MaybeUninit::uninit(); // Char*

static mut G_insCount: MaybeUninit<Int64> = MaybeUninit::uninit();

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

    println!("DEBUG: {:?}\n {:?}", DEFAULT_HEAP_SIZE, unsafe {
        G_options.assume_init()
    });

    eprintln!("\nusage: {} [options] classfile [args]", unsafe {
        G_progName.assume_init()
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

    exit(1);
}

/* parse a size argument */
// Int
fn parseSize(arg: String,  p: String) -> UInt {
    char* end;
  
    Int ret = (Int)strtoul(p, &end, 10);
  
    if end == p {
      eprintln!( "ERROR: expected size argument after '{}'", arg);
      usage();
    }

    std::libc::strtoul();


    // Int mult = 0;

    let mult = match  (*end++) {
    'b' => 1,
    'K' => 1024,
    'M' => 1024*1024,
    'G' => 1024*1024*1024,
    _ => {
      eprintln!( "ERROR: unknown size argument '{}' after '{}'", p, arg);
      eprintln!( "       expected:  digit*(b|K|M|G)");
      eprintln!( "           e.g. 3000b, 10K, 12M, 1G");
      usage();
    }
    };

    if (*end){
      eprintln!( "ERROR: unexpected '{}' after size argument '{}'", *end, p);
      usage();
    }

    ret * mult
}

/* parse all the arguments */
fn parseArgs() -> String {
    let mut mainMod: = String::new();;
    //C int i;

    /* initialize options */
    // std::env::args().len()

    unsafe {
        G_progName
            .as_mut_ptr()
            .write(Box::leak(std::env::args().nth(0).unwrap().into_boxed_str()))
    };

    let mut parsedOptions = Options {
        heapSize: DEFAULT_HEAP_SIZE,
        maxStackSize: DEFAULT_MAX_STACK_SIZE,
        stats: false,
        pretty: false,
    };

    //ERRROR parsedOptions.heapSize = ;

    let mut remaining_arg_start: usize = 0;

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

        if arg == "-h" || arg== "--heap"{
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
          }  else if arg == "-stats"{
            parsedOptions.stats = true;
          } else if arg == "-dump"{
            parsedOptions.pretty = true;
          } else if arg == "-v" ||
          arg == "-version" ||
           arg == "--version" {
           version();
           exit(0)
         } else {
           eprintln!("WARNING: ignored unknown flag '{}'", arg);
         }

         i += 1;
    }

    unsafe { G_options.as_mut_ptr().write(parsedOptions) }

    /* check we have a main module */
    // if !mainMod {
    //     usage();
    // }

    if mainMod.len() == 0 {
        usage();
    }

    /* store argument information */
    // G_argc = argc - i;
    // G_argv = &argv[i];

    /* get yhc base path */
    // G_yhcBasePath = basepath_get();

    return mainMod;
}
// {:#?}

// {:?}

// main.c
fn main() {
    /* parse program arguments */
    let mainMod = parseArgs();
    parseArgs();
}
