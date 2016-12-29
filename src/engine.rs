// src/engine.rs -- interface for the Tectonic engine
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2::{Compression, GzBuilder};
use std::ffi::{CStr, CString, OsString};
use std::fs::File;
use std::io::{stderr, stdout, Cursor, Read, Write};
use std::os::unix::io::{IntoRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::ptr;
use zip::result::{ZipResult};

use ::{assign_global_engine, EngineInternals};
use bundle::Bundle;
use c_api;
use file_format::{format_to_extension, FileFormat};


type OutputItem = Box<Write>;

pub enum InputItem {
    File(File),
    BundleFile(Cursor<Vec<u8>>),
    //BundleGz(GzDecoder<Cursor<Vec<u8>>>),
}

pub struct Engine {
    bundle: Option<Bundle<File>>,
    output_handles: Vec<Box<OutputItem>>,
    input_handles: Vec<Box<InputItem>>,
}


// The public interface.

impl Engine {
    pub fn new () -> Engine {
        Engine {
            bundle: None,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
        }
    }

    pub fn set_output_format (&mut self, outfmt: &str) -> () {
        // TODO: use enums for safety, etc.
        if outfmt == "xdv" {
            unsafe { c_api::tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); }
        }
    }

    pub fn use_bundle (&mut self, path: &Path) -> ZipResult<()> {
        match Bundle::open (path) {
            Ok(b) => { self.bundle = Some(b) ; Ok(()) },
            Err(e) => Err(e)
        }
    }

    pub fn process (&mut self, format_file_name: &str, input_file_name: &str) -> Option<String> {
        let cformat = CString::new(format_file_name).unwrap();
        let cinput = CString::new(input_file_name).unwrap();

        unsafe {
            assign_global_engine (self, || {
                c_api::tt_misc_initialize(cformat.as_ptr());
                let result = c_api::tt_run_engine(cinput.as_ptr());

                if result == 3 {
                    let ptr = c_api::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    return Some(msg)
                }

                None
            })
        }
    }
}


impl EngineInternals for Engine {
    fn get_readable_fd(&mut self, name: &Path, format: FileFormat, must_exist: bool) -> Option<RawFd> {
        /* We currently don't care about must_exist. */

        /* For now: if we can open straight off of the filesystem, do that. No
         * bundle needed. */

        if let Ok(f) = File::open (name) {
            return Some(f.into_raw_fd());
        }

        let mut ext = PathBuf::from (name);
        let mut ename = OsString::from (ext.file_name ().unwrap ());
        ename.push (format_to_extension (format));
        ext.set_file_name (ename);

        if let Ok(f) = File::open (ext.clone ()) {
            return Some(f.into_raw_fd());
        }

        /* If the bundle has been opened, see if it's got the file. */

        match self.bundle {
            Some(ref mut bundle) => bundle.get_readable_fd(name, format, must_exist),
            None => None
        }
    }

    type OutputHandle = OutputItem;
    type InputHandle = InputItem;

    fn output_open(&mut self, name: &Path, is_gz: bool) -> *const Self::OutputHandle {
        // TODO: use the I/O layer and write to a buffer!

        match File::create (name) {
            Ok(f) => {
                let oi: Box<Write> = if is_gz {
                    let gzf = GzBuilder::new().write(f, Compression::Default);
                    Box::new(gzf)
                } else {
                    Box::new(f)
                };
                self.output_handles.push(Box::new(oi));
                &*self.output_handles[self.output_handles.len()-1]
            },
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: open of {} failed: {}",
                         name.display(), e).expect("stderr failed");
                ptr::null()
            }
        }
    }

    fn output_open_stdout(&mut self) -> *const Self::OutputHandle {
        self.output_handles.push(Box::new(Box::new(stdout())));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_write(&mut self, handle: *mut Self::OutputHandle, buf: &[u8]) -> bool {
        let rhandle: &mut OutputItem = unsafe { &mut *handle };
        let result = rhandle.write_all(buf);

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: write failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn output_flush(&mut self, handle: *mut Self::OutputHandle) -> bool {
        let rhandle: &mut OutputItem = unsafe { &mut *handle };
        let result = rhandle.flush();

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: flush failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn output_close(&mut self, handle: *mut Self::OutputHandle) -> bool {
        let len = self.output_handles.len();

        for i in 0..len {
            let p: *const Self::OutputHandle = &*self.output_handles[i];

            if p == handle {
                self.output_handles.swap_remove(i);
                break;
            }
        }

        false
    }

    fn input_open(&mut self, name: &Path, format: FileFormat, is_gz: bool) -> *const Self::InputHandle {
        /* For now: if we can open straight off of the filesystem, do that. No
         * bundle needed. */

        if is_gz {
            panic!("implement is_gz!");
        }

        if let Ok(f) = File::open (name) {
            let ii = InputItem::File(f);
            self.input_handles.push(Box::new(ii));
            return &*self.input_handles[self.input_handles.len()-1];
        }

        let mut ext = PathBuf::from (name);
        let mut ename = OsString::from (ext.file_name ().unwrap ());
        ename.push (format_to_extension (format));
        ext.set_file_name (ename);

        if let Ok(f) = File::open (ext.clone ()) {
            let ii = InputItem::File(f);
            self.input_handles.push(Box::new(ii));
            return &*self.input_handles[self.input_handles.len()-1];
        }

        /* If the bundle has been opened, see if it's got the file. */

        if let Some(ref mut bundle) = self.bundle {
            let ii = match bundle.get_buffer(name, format) {
                Ok(b) => InputItem::BundleFile(b),
                Err(_) => return ptr::null()
            };

            self.input_handles.push(Box::new(ii));
            &*self.input_handles[self.input_handles.len()-1]
        } else {
            ptr::null()
        }
    }

    fn input_get_size(&mut self, handle: *mut Self::InputHandle) -> usize {
        let rhandle: &mut InputItem = unsafe { &mut *handle };

        match *rhandle {
            InputItem::File(ref mut f) => f.metadata().unwrap().len() as usize,
            InputItem::BundleFile(ref mut f) => f.get_ref().len(),
        }
    }

    fn input_read(&mut self, handle: *mut Self::InputHandle, buf: &mut [u8]) -> bool {
        let rhandle: &mut InputItem = unsafe { &mut *handle };

        let result = match *rhandle {
            InputItem::File(ref mut f) => f.read_exact(buf),
            InputItem::BundleFile(ref mut f) => f.read_exact(buf),
        };

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: read failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn input_close(&mut self, handle: *mut Self::InputHandle) -> bool {
        let len = self.input_handles.len();

        for i in 0..len {
            let p: *const Self::InputHandle = &*self.input_handles[i];

            if p == handle {
                self.input_handles.swap_remove(i);
                return false;
            }
        }

        panic!("unexpeted handle {:?}", handle);
    }
}