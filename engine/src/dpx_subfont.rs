#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, label_break_value)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    /* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    /* Both XeTeX and bibtex use this enum: */
    /* The weird enum values are historical and could be rationalized. But it is
     * good to write them explicitly since they must be kept in sync with
     * `src/engines/mod.rs`.
     */
    /* quasi-hack to get the primary input */
    /* Bridge API. Keep synchronized with src/engines/mod.rs. */
    /* These functions are not meant to be used in the C/C++ code. They define the
     * API that we expose to the Rust side of things. */
    /* The internal, C/C++ interface: */
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(
        buffer: *mut libc::c_char,
        length: libc::c_int,
        file: rust_input_handle_t,
    ) -> *mut libc::c_char;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: uint32_t) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
/* Don't forget fontmap reading now requires information
 * from SFD files. You must initialize at least sfd_file_
 * cache before starting loading of fontmaps.
 */
/* Subfont Definition File:
 *  struct sfd_file_ is for storing subfont identifiers
 *  contained in a SFD file and for mapping string pair
 *  <SFD_file, Subfont_id> to internal code mapping table
 *  ID which is index within an array of struct sfd_rec_.
 *  We store code mapping tables in different place than
 *  struct sfd_file_.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfd_file_ {
    pub ident: *mut libc::c_char,
    pub sub_id: *mut *mut libc::c_char,
    pub rec_id: *mut libc::c_int,
    pub max_subfonts: libc::c_int,
    pub num_subfonts: libc::c_int,
}
/* Mapping table */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfd_rec_ {
    pub vector: [libc::c_ushort; 256],
    /* 0 for undefined */
}
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn subfont_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
unsafe extern "C" fn init_sfd_file_(mut sfd: *mut sfd_file_) {
    (*sfd).ident = 0 as *mut libc::c_char;
    (*sfd).sub_id = 0 as *mut *mut libc::c_char;
    (*sfd).rec_id = 0 as *mut libc::c_int;
    (*sfd).num_subfonts = 0i32;
    (*sfd).max_subfonts = (*sfd).num_subfonts;
}
unsafe extern "C" fn clean_sfd_file_(mut sfd: *mut sfd_file_) {
    let mut i: libc::c_int = 0;
    free((*sfd).ident as *mut libc::c_void);
    if !(*sfd).sub_id.is_null() {
        i = 0i32;
        while i < (*sfd).num_subfonts {
            free(*(*sfd).sub_id.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free((*sfd).sub_id as *mut libc::c_void);
    }
    free((*sfd).rec_id as *mut libc::c_void);
    init_sfd_file_(sfd);
}
static mut sfd_files: *mut sfd_file_ = 0 as *const sfd_file_ as *mut sfd_file_;
static mut num_sfd_files: libc::c_int = 0i32;
static mut max_sfd_files: libc::c_int = 0i32;
static mut sfd_record: *mut sfd_rec_ = 0 as *const sfd_rec_ as *mut sfd_rec_;
static mut num_sfd_records: libc::c_int = 0i32;
static mut max_sfd_records: libc::c_int = 0i32;
static mut line_buf: [libc::c_char; 4096] = [0; 4096];
/* Each lines describes character code mapping for each
 * subfonts. '#' is start of comment.
 * SFD file format uses a '\' before newline sequence
 * for line-continuation.
 */
unsafe extern "C" fn readline(
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
    mut handle: *mut rust_input_handle_t,
) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = buf;
    let mut n: libc::c_int = 0i32;
    let mut c: libc::c_int = 0i32;
    while buf_len - n > 0i32 && {
        q = tt_mfgets(p, buf_len - n, handle as rust_input_handle_t);
        !q.is_null()
    } {
        c += 1;
        r = strchr(q, '#' as i32);
        /* Comment is converted to single wsp (followed by a newline). */
        if !r.is_null() {
            *r = ' ' as i32 as libc::c_char; /* empty line */
            *r.offset(1) = '\u{0}' as i32 as libc::c_char
        }
        if strlen(q) == 0i32 as libc::c_ulong {
            break;
        }
        n = (n as libc::c_ulong).wrapping_add(strlen(q)) as libc::c_int as libc::c_int;
        q = q.offset(strlen(q).wrapping_sub(1i32 as libc::c_ulong) as isize);
        if *q as libc::c_int != '\\' as i32 {
            break;
        }
        /* line continued */
        n -= 1i32;
        p = buf.offset(n as isize)
    }
    if n >= buf_len - 1i32 {
        dpx_warning(
            b"Possible buffer overflow in reading SFD file (buffer full, size=%d bytes)\x00"
                as *const u8 as *const libc::c_char,
            buf_len - 1i32,
        );
    }
    return if c > 0i32 {
        buf
    } else {
        0 as *mut libc::c_char
    };
}
/* subfont_id ( integer ':' | integer '_' integer | integer )*
 *
 *  0x32: ==> Subsequent integers are place into slots starting at 0x32.
 *    0x32: 0xA1A1 0xA1A2 ... ==> 0x32 is mappned to 0xA1A1, 0x33 to 0xA1A2
 *  0xA1A1_0xA1A5 ==> Expanded to 0xA1A1 0xA1A2 ... 0xA1A5
 */
/* subfont_id is already consumed here. */
unsafe extern "C" fn read_sfd_record(
    mut rec: *mut sfd_rec_,
    mut lbuf: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = lbuf;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut repos: libc::c_int = 0i32;
    let mut c: libc::c_int = 0;
    let mut v1: libc::c_int = 0i32;
    let mut v2: libc::c_int = 0i32;
    let mut curpos: libc::c_int = 0i32;
    let mut error: libc::c_int = 0i32;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    while error == 0 && *p as libc::c_int != 0 {
        repos = 0i32;
        q = p;
        v1 = strtol(p, &mut r, 0i32) as libc::c_int;
        q = r;
        if q == p
            || !(*q as libc::c_int == '\u{0}' as i32
                || *(*__ctype_b_loc()).offset(*q as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    != 0)
                && *q as libc::c_int != ':' as i32
                && *q as libc::c_int != '_' as i32
        {
            dpx_warning(
                b"Unknown token in subfont mapping table: %c\x00" as *const u8
                    as *const libc::c_char,
                *q as libc::c_int,
            );
            return -1i32;
        }
        match *q as libc::c_int {
            58 => {
                if v1 < 0i32 || v1 > 0xffi32 {
                    dpx_warning(
                        b"Invalud value for subfont table offset: %d\x00" as *const u8
                            as *const libc::c_char,
                        v1,
                    );
                    return -1i32;
                }
                repos = 1i32;
                q = q.offset(1)
            }
            95 => {
                p = q.offset(1);
                v2 = strtol(p, &mut r, 0i32) as libc::c_int;
                q = r;
                if v1 < 0i32
                    || v1 as libc::c_long > 0xffff
                    || v2 < 0i32
                    || v2 as libc::c_long > 0xffff
                {
                    dpx_warning(
                        b"Invalid value in subfont mapping table: 0x%x_0x%x\x00" as *const u8
                            as *const libc::c_char,
                        v1,
                        v2,
                    );
                    return -1i32;
                } else {
                    if q == p
                        || !(*q as libc::c_int == '\u{0}' as i32
                            || *(*__ctype_b_loc())
                                .offset(*q as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                    {
                        dpx_warning(
                            b"Invalid char in subfont mapping table: %c\x00" as *const u8
                                as *const libc::c_char,
                            *q as libc::c_int,
                        );
                        return -1i32;
                    }
                }
            }
            _ => {
                if v1 < 0i32 || v1 as libc::c_long > 0xffff {
                    dpx_warning(
                        b"Invalid character code in subfont mapping table: 0x%x\x00" as *const u8
                            as *const libc::c_char,
                        v1,
                    );
                    return -1i32;
                }
                v2 = v1
            }
        }
        if repos != 0 {
            curpos = v1
        } else {
            if v2 < v1 || curpos + (v2 - v1) > 0xffi32 {
                dpx_warning(b"Invalid range in subfont mapping: curpos=\"0x%02x\" range=\"0x%04x,0x%04x\"\x00"
                                as *const u8 as *const libc::c_char, curpos,
                            v1, v2);
                return -1i32;
            }
            c = v1;
            while c <= v2 {
                if (*rec).vector[curpos as usize] as libc::c_int != 0i32 {
                    dpx_warning(
                        b"Subfont mapping for slot=\"0x%02x\" already defined...\x00" as *const u8
                            as *const libc::c_char,
                        curpos,
                    );
                    return -1i32;
                }
                if curpos >= 0i32 && curpos <= 255i32 {
                } else {
                    __assert_fail(
                        b"curpos >= 0 && curpos <= 255\x00" as *const u8 as *const libc::c_char,
                        b"dpx-subfont.c\x00" as *const u8 as *const libc::c_char,
                        230i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                            b"int read_sfd_record(struct sfd_rec_ *, const char *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                let fresh0 = curpos;
                curpos = curpos + 1;
                (*rec).vector[fresh0 as usize] = c as libc::c_ushort;
                c += 1
            }
        }
        p = q;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
    }
    return error;
}
/* Scan for subfont IDs */
unsafe extern "C" fn scan_sfd_file(
    mut sfd: *mut sfd_file_,
    mut handle: *mut rust_input_handle_t,
) -> libc::c_int {
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char; /* empty */
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut lpos: libc::c_int = 0i32;
    if !sfd.is_null() && !handle.is_null() {
    } else {
        __assert_fail(
            b"sfd && handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-subfont.c\x00" as *const u8 as *const libc::c_char,
            248i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"int scan_sfd_file(struct sfd_file_ *, rust_input_handle_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    if verbose > 3i32 {
        dpx_message(
            b"\nsubfont>> Scanning SFD file \"%s\"...\n\x00" as *const u8 as *const libc::c_char,
            (*sfd).ident,
        );
    }
    ttstub_input_seek(handle as rust_input_handle_t, 0i32 as ssize_t, 0i32);
    (*sfd).num_subfonts = 0i32;
    (*sfd).max_subfonts = (*sfd).num_subfonts;
    loop {
        p = readline(line_buf.as_mut_ptr(), 4096i32, handle);
        if p.is_null() {
            break;
        }
        lpos += 1;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
        if *p as libc::c_int == 0i32 {
            continue;
        }
        /* Saw non-wsp here */
        n = 0i32;
        q = p;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            p = p.offset(1);
            n += 1
        }
        id = new(((n + 1i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char;
        memcpy(
            id as *mut libc::c_void,
            q as *const libc::c_void,
            n as libc::c_ulong,
        );
        *id.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
        if (*sfd).num_subfonts >= (*sfd).max_subfonts {
            (*sfd).max_subfonts += 16i32;
            (*sfd).sub_id = renew(
                (*sfd).sub_id as *mut libc::c_void,
                ((*sfd).max_subfonts as uint32_t as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as uint32_t,
            ) as *mut *mut libc::c_char
        }
        if verbose > 3i32 {
            dpx_message(
                b"subfont>>   id=\"%s\" at line=\"%d\"\n\x00" as *const u8 as *const libc::c_char,
                id,
                lpos,
            );
        }
        let ref mut fresh1 = *(*sfd).sub_id.offset((*sfd).num_subfonts as isize);
        *fresh1 = id;
        (*sfd).num_subfonts += 1
    }
    (*sfd).rec_id = new(((*sfd).num_subfonts as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_int;
    n = 0i32;
    while n < (*sfd).num_subfonts {
        *(*sfd).rec_id.offset(n as isize) = -1i32;
        n += 1
        /* Not loaded yet. We do lazy loading of map definitions. */
    }
    if verbose > 3i32 {
        dpx_message(
            b"subfont>> %d entries found in SFD file \"%s\".\n\x00" as *const u8
                as *const libc::c_char,
            (*sfd).num_subfonts,
            (*sfd).ident,
        );
    }
    return 0i32;
}
/* Open SFD file and gather subfont IDs. We do not read mapping tables
 * here but only read subfont IDs used in SFD file.
 */
unsafe extern "C" fn find_sfd_file(mut sfd_name: *const libc::c_char) -> libc::c_int {
    let mut id: libc::c_int = -1i32;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = -1i32;
    /* Check if we already opened SFD file */
    i = 0i32;
    while i < num_sfd_files {
        if streq_ptr((*sfd_files.offset(i as isize)).ident, sfd_name) {
            id = i;
            break;
        } else {
            i += 1
        }
    }
    if id < 0i32 {
        let mut sfd: *mut sfd_file_ = 0 as *mut sfd_file_;
        let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
        if num_sfd_files >= max_sfd_files {
            max_sfd_files += 8i32;
            sfd_files = renew(
                sfd_files as *mut libc::c_void,
                (max_sfd_files as uint32_t as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<sfd_file_>() as libc::c_ulong)
                    as uint32_t,
            ) as *mut sfd_file_
        }
        sfd = &mut *sfd_files.offset(num_sfd_files as isize) as *mut sfd_file_;
        init_sfd_file_(sfd);
        (*sfd).ident = new(
            (strlen(sfd_name).wrapping_add(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as uint32_t,
        ) as *mut libc::c_char;
        strcpy((*sfd).ident, sfd_name);
        handle = ttstub_input_open((*sfd).ident, TTIF_SFD, 0i32) as *mut rust_input_handle_t;
        if handle.is_null() {
            clean_sfd_file_(sfd);
            return -1i32;
        }
        error = scan_sfd_file(sfd, handle);
        ttstub_input_close(handle as rust_input_handle_t);
        if error == 0 {
            let fresh2 = num_sfd_files;
            num_sfd_files = num_sfd_files + 1;
            id = fresh2
        } else {
            dpx_warning(
                b"Error occured while reading SFD file \"%s\"\x00" as *const u8
                    as *const libc::c_char,
                sfd_name,
            );
            clean_sfd_file_(sfd);
            id = -1i32
        }
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn sfd_get_subfont_ids(
    mut sfd_name: *const libc::c_char,
    mut num_ids: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut sfd_id: libc::c_int = 0;
    if sfd_name.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    sfd_id = find_sfd_file(sfd_name);
    if sfd_id < 0i32 {
        return 0 as *mut *mut libc::c_char;
    }
    if !num_ids.is_null() {
        *num_ids = (*sfd_files.offset(sfd_id as isize)).num_subfonts
    }
    return (*sfd_files.offset(sfd_id as isize)).sub_id;
}
/* Make sure that sfd_name does not have the extension '.sfd'.
 * Mapping tables are actually read here.
 */
#[no_mangle]
pub unsafe extern "C" fn sfd_load_record(
    mut sfd_name: *const libc::c_char,
    mut subfont_id: *const libc::c_char,
) -> libc::c_int {
    let mut rec_id: libc::c_int = -1i32;
    let mut sfd: *mut sfd_file_ = 0 as *mut sfd_file_;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut sfd_id: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if sfd_name.is_null() || subfont_id.is_null() {
        return -1i32;
    }
    sfd_id = find_sfd_file(sfd_name);
    if sfd_id < 0i32 {
        return -1i32;
    }
    sfd = &mut *sfd_files.offset(sfd_id as isize) as *mut sfd_file_;
    /* Check if we already loaded mapping table. */
    i = 0i32;
    while i < (*sfd).num_subfonts && strcmp(*(*sfd).sub_id.offset(i as isize), subfont_id) != 0 {
        i += 1
    }
    if i == (*sfd).num_subfonts {
        dpx_warning(
            b"Subfont id=\"%s\" not exist in SFD file \"%s\"...\x00" as *const u8
                as *const libc::c_char,
            subfont_id,
            (*sfd).ident,
        );
        return -1i32;
    } else {
        if *(*sfd).rec_id.offset(i as isize) >= 0i32 {
            return *(*sfd).rec_id.offset(i as isize);
        }
    }
    if verbose > 3i32 {
        dpx_message(
            b"\nsubfont>> Loading SFD mapping table for <%s,%s>...\x00" as *const u8
                as *const libc::c_char,
            (*sfd).ident,
            subfont_id,
        );
    }
    /* reopen */
    handle = ttstub_input_open((*sfd).ident, TTIF_SFD, 0i32) as *mut rust_input_handle_t;
    if handle.is_null() {
        return -1i32;
        /* _tt_abort("Could not open SFD file \"%s\"", sfd_name); */
    }
    loop
    /* Seek to record for 'sub_name'. */
    {
        p = readline(line_buf.as_mut_ptr(), 4096i32, handle); /* empty line */
        if p.is_null() {
            break;
        }
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
        if *p as libc::c_int == 0i32 {
            continue;
        }
        /* q = parse_ident(&p, p + strlen(p)); */
        q = p;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            p = p.offset(1)
        }
        *p = '\u{0}' as i32 as libc::c_char;
        p = p.offset(1);
        if streq_ptr(q, subfont_id) {
            if num_sfd_records >= max_sfd_records {
                max_sfd_records += 16i32;
                sfd_record = renew(
                    sfd_record as *mut libc::c_void,
                    (max_sfd_records as uint32_t as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<sfd_rec_>() as libc::c_ulong)
                        as uint32_t,
                ) as *mut sfd_rec_
            }
            if !(*sfd_record.offset(num_sfd_records as isize))
                .vector
                .as_mut_ptr()
                .is_null()
            {
                let mut __i: libc::c_int = 0;
                __i = 0i32;
                while __i < 256i32 {
                    (*sfd_record.offset(num_sfd_records as isize)).vector[__i as usize] =
                        0i32 as libc::c_ushort;
                    __i += 1
                }
            }
            error = read_sfd_record(&mut *sfd_record.offset(num_sfd_records as isize), p);
            if error != 0 {
                dpx_warning(
                    b"Error occured while reading SFD file: file=\"%s\" subfont_id=\"%s\"\x00"
                        as *const u8 as *const libc::c_char,
                    (*sfd).ident,
                    subfont_id,
                );
            } else {
                let fresh3 = num_sfd_records;
                num_sfd_records = num_sfd_records + 1;
                rec_id = fresh3
            }
        }
    }
    if rec_id < 0i32 {
        dpx_warning(
            b"Failed to load subfont mapping table for SFD=\"%s\" subfont_id=\"%s\"\x00"
                as *const u8 as *const libc::c_char,
            (*sfd).ident,
            subfont_id,
        );
    }
    *(*sfd).rec_id.offset(i as isize) = rec_id;
    ttstub_input_close(handle as rust_input_handle_t);
    if verbose > 3i32 {
        let mut __i_0: libc::c_int = 0;
        if rec_id >= 0i32 {
            dpx_message(
                b" at id=\"%d\"\x00" as *const u8 as *const libc::c_char,
                rec_id,
            );
            dpx_message(
                b"\nsubfont>> Content of mapping table:\x00" as *const u8 as *const libc::c_char,
            );
            __i_0 = 0i32;
            while __i_0 < 256i32 {
                if __i_0 % 16i32 == 0i32 {
                    dpx_message(b"\nsubfont>>  \x00" as *const u8 as *const libc::c_char);
                }
                dpx_message(
                    b" %04x\x00" as *const u8 as *const libc::c_char,
                    (*sfd_record.offset(rec_id as isize)).vector[__i_0 as usize] as libc::c_int,
                );
                __i_0 += 1
            }
        }
        dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return rec_id;
}
/* Lookup mapping table */
#[no_mangle]
pub unsafe extern "C" fn lookup_sfd_record(
    mut rec_id: libc::c_int,
    mut c: libc::c_uchar,
) -> libc::c_ushort {
    if sfd_record.is_null() || rec_id < 0i32 || rec_id >= num_sfd_records {
        _tt_abort(
            b"Invalid subfont_id: %d\x00" as *const u8 as *const libc::c_char,
            rec_id,
        );
    }
    return (*sfd_record.offset(rec_id as isize)).vector[c as usize];
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[no_mangle]
pub unsafe extern "C" fn release_sfd_record() {
    let mut i: libc::c_int = 0;
    if !sfd_record.is_null() {
        free(sfd_record as *mut libc::c_void);
    }
    if !sfd_files.is_null() {
        i = 0i32;
        while i < num_sfd_files {
            clean_sfd_file_(&mut *sfd_files.offset(i as isize));
            i += 1
        }
        free(sfd_files as *mut libc::c_void);
    }
    sfd_record = 0 as *mut sfd_rec_;
    sfd_files = 0 as *mut sfd_file_;
    max_sfd_records = 0i32;
    num_sfd_records = max_sfd_records;
    max_sfd_files = 0i32;
    num_sfd_files = max_sfd_files;
}