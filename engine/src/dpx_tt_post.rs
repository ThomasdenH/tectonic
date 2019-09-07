#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(label_break_value)]
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const libc::c_char) -> SFNT_ULONG;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> libc::c_uchar;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> libc::c_ushort;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> libc::c_short;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
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
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = libc::c_uchar;
pub type USHORT = libc::c_ushort;
pub type SHORT = libc::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
pub type FWord = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [libc::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut libc::c_char,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const libc::c_char,
    pub names: *mut *mut libc::c_char,
    pub count: USHORT,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
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
/* offset from begenning of the post table */
unsafe extern "C" fn read_v2_post_names(
    mut post: *mut tt_post_table,
    mut sfont: *mut sfnt,
) -> libc::c_int {
    let mut i: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut indices: *mut USHORT = 0 as *mut USHORT;
    let mut maxidx: USHORT = 0;
    let mut len: libc::c_int = 0;
    (*post).numberOfGlyphs = tt_get_unsigned_pair((*sfont).handle);
    indices = new(((*post).numberOfGlyphs as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as libc::c_ulong)
        as uint32_t) as *mut USHORT;
    maxidx = 257i32 as USHORT;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < (*post).numberOfGlyphs as libc::c_int {
        idx = tt_get_unsigned_pair((*sfont).handle);
        if idx as libc::c_int >= 258i32 {
            if idx as libc::c_int > maxidx as libc::c_int {
                maxidx = idx
            }
            if idx as libc::c_int > 32767i32 {
                /* Although this is strictly speaking out of spec, it seems to work
                and there are real-life fonts that use it.
                We show a warning only once, instead of thousands of times */
                static mut warning_issued: libc::c_char = 0i32 as libc::c_char;
                if warning_issued == 0 {
                    dpx_warning(
                        b"TrueType post table name index %u > 32767\x00" as *const u8
                            as *const libc::c_char,
                        idx as libc::c_int,
                    );
                    warning_issued = 1i32 as libc::c_char
                }
                /* In a real-life large font, (x)dvipdfmx crashes if we use
                nonvanishing idx in the case of idx > 32767.
                If we set idx = 0, (x)dvipdfmx works fine for the font and
                created pdf seems fine. The post table may not be important
                in such a case */
                idx = 0i32 as USHORT
            }
        }
        *indices.offset(i as isize) = idx;
        i = i.wrapping_add(1)
    }
    (*post).count = (maxidx as libc::c_int - 257i32) as USHORT;
    if ((*post).count as libc::c_int) < 1i32 {
        (*post).names = 0 as *mut *mut libc::c_char
    } else {
        (*post).names = new(((*post).count as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut *mut libc::c_char;
        i = 0i32 as USHORT;
        while (i as libc::c_int) < (*post).count as libc::c_int {
            /* read Pascal strings */
            len = tt_get_unsigned_byte((*sfont).handle) as libc::c_int;
            if len > 0i32 {
                let ref mut fresh0 = *(*post).names.offset(i as isize);
                *fresh0 = new(((len + 1i32) as uint32_t as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as uint32_t) as *mut libc::c_char;
                ttstub_input_read(
                    (*sfont).handle,
                    *(*post).names.offset(i as isize),
                    len as size_t,
                );
                *(*(*post).names.offset(i as isize)).offset(len as isize) = 0i32 as libc::c_char
            } else {
                let ref mut fresh1 = *(*post).names.offset(i as isize);
                *fresh1 = 0 as *mut libc::c_char
            }
            i = i.wrapping_add(1)
        }
    }
    (*post).glyphNamePtr = new(((*post).numberOfGlyphs as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut *const libc::c_char;
    i = 0i32 as USHORT;
    while (i as libc::c_int) < (*post).numberOfGlyphs as libc::c_int {
        idx = *indices.offset(i as isize);
        if (idx as libc::c_int) < 258i32 {
            let ref mut fresh2 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh2 = macglyphorder[idx as usize]
        } else if idx as libc::c_int - 258i32 < (*post).count as libc::c_int {
            let ref mut fresh3 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh3 = *(*post).names.offset((idx as libc::c_int - 258i32) as isize)
        } else {
            dpx_warning(
                b"Invalid glyph name index number: %u (>= %u)\x00" as *const u8
                    as *const libc::c_char,
                idx as libc::c_int,
                (*post).count as libc::c_int + 258i32,
            );
            free(indices as *mut libc::c_void);
            return -1i32;
        }
        i = i.wrapping_add(1)
    }
    free(indices as *mut libc::c_void);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_post_table(mut sfont: *mut sfnt) -> *mut tt_post_table {
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    /* offset = */
    sfnt_locate_table(sfont, b"post\x00" as *const u8 as *const libc::c_char); /* Fixed */
    post = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<tt_post_table>() as libc::c_ulong)
        as uint32_t) as *mut tt_post_table; /* Fixed */
    (*post).Version = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).italicAngle = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).underlinePosition = tt_get_signed_pair((*sfont).handle); /* wrong */
    (*post).underlineThickness = tt_get_signed_pair((*sfont).handle);
    (*post).isFixedPitch = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).numberOfGlyphs = 0i32 as USHORT;
    (*post).glyphNamePtr = 0 as *mut *const libc::c_char;
    (*post).count = 0i32 as USHORT;
    (*post).names = 0 as *mut *mut libc::c_char;
    if (*post).Version as libc::c_ulong == 0x10000 {
        (*post).numberOfGlyphs = 258i32 as USHORT;
        (*post).glyphNamePtr = macglyphorder.as_mut_ptr()
    } else if (*post).Version as libc::c_ulong == 0x28000 {
        dpx_warning(
            b"TrueType \'post\' version 2.5 found (deprecated)\x00" as *const u8
                as *const libc::c_char,
        );
    } else if (*post).Version as libc::c_ulong == 0x20000 {
        if read_v2_post_names(post, sfont) < 0i32 {
            dpx_warning(
                b"Invalid version 2.0 \'post\' table\x00" as *const u8 as *const libc::c_char,
            );
            tt_release_post_table(post);
            post = 0 as *mut tt_post_table
        }
    } else if !((*post).Version as libc::c_ulong == 0x30000
        || (*post).Version as libc::c_ulong == 0x40000)
    {
        dpx_warning(
            b"Unknown \'post\' version: %08X, assuming version 3.0\x00" as *const u8
                as *const libc::c_char,
            (*post).Version,
        );
    }
    return post;
}
#[no_mangle]
pub unsafe extern "C" fn tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const libc::c_char,
) -> USHORT {
    let mut gid: USHORT = 0;
    if !post.is_null() && !glyphname.is_null() {
    } else {
        __assert_fail(
            b"post && glyphname\x00" as *const u8 as *const libc::c_char,
            b"dpx-tt_post.c\x00" as *const u8 as *const libc::c_char,
            157i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                b"USHORT tt_lookup_post_table(struct tt_post_table *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    gid = 0i32 as USHORT;
    while (gid as libc::c_int) < (*post).count as libc::c_int {
        if !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
            && streq_ptr(glyphname, *(*post).glyphNamePtr.offset(gid as isize)) as libc::c_int != 0
        {
            return gid;
        }
        gid = gid.wrapping_add(1)
    }
    return 0i32 as USHORT;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_glyphname(
    mut post: *mut tt_post_table,
    mut gid: USHORT,
) -> *mut libc::c_char {
    if (gid as libc::c_int) < (*post).count as libc::c_int
        && !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
    {
        return xstrdup(*(*post).glyphNamePtr.offset(gid as isize));
    }
    return 0 as *mut libc::c_char;
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
/* Glyph names (pointer to C string) */
/* Non-standard glyph names */
/* Number of glyph names in names[] */
#[no_mangle]
pub unsafe extern "C" fn tt_release_post_table(mut post: *mut tt_post_table) {
    let mut i: USHORT = 0;
    if !post.is_null() {
    } else {
        __assert_fail(
            b"post\x00" as *const u8 as *const libc::c_char,
            b"dpx-tt_post.c\x00" as *const u8 as *const libc::c_char,
            182i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void tt_release_post_table(struct tt_post_table *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*post).glyphNamePtr.is_null() && (*post).Version as libc::c_ulong != 0x10000 {
        free((*post).glyphNamePtr as *mut libc::c_void);
    }
    if !(*post).names.is_null() {
        i = 0i32 as USHORT;
        while (i as libc::c_int) < (*post).count as libc::c_int {
            free(*(*post).names.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*post).names as *mut libc::c_void);
    }
    (*post).count = 0i32 as USHORT;
    (*post).glyphNamePtr = 0 as *mut *const libc::c_char;
    (*post).names = 0 as *mut *mut libc::c_char;
    free(post as *mut libc::c_void);
}
/* Macintosh glyph order - from apple's TTRefMan */
static mut macglyphorder: [*const libc::c_char; 258] = [
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".null\x00" as *const u8 as *const libc::c_char,
    b"nonmarkingreturn\x00" as *const u8 as *const libc::c_char,
    b"space\x00" as *const u8 as *const libc::c_char,
    b"exclam\x00" as *const u8 as *const libc::c_char,
    b"quotedbl\x00" as *const u8 as *const libc::c_char,
    b"numbersign\x00" as *const u8 as *const libc::c_char,
    b"dollar\x00" as *const u8 as *const libc::c_char,
    b"percent\x00" as *const u8 as *const libc::c_char,
    b"ampersand\x00" as *const u8 as *const libc::c_char,
    b"quotesingle\x00" as *const u8 as *const libc::c_char,
    b"parenleft\x00" as *const u8 as *const libc::c_char,
    b"parenright\x00" as *const u8 as *const libc::c_char,
    b"asterisk\x00" as *const u8 as *const libc::c_char,
    b"plus\x00" as *const u8 as *const libc::c_char,
    b"comma\x00" as *const u8 as *const libc::c_char,
    b"hyphen\x00" as *const u8 as *const libc::c_char,
    b"period\x00" as *const u8 as *const libc::c_char,
    b"slash\x00" as *const u8 as *const libc::c_char,
    b"zero\x00" as *const u8 as *const libc::c_char,
    b"one\x00" as *const u8 as *const libc::c_char,
    b"two\x00" as *const u8 as *const libc::c_char,
    b"three\x00" as *const u8 as *const libc::c_char,
    b"four\x00" as *const u8 as *const libc::c_char,
    b"five\x00" as *const u8 as *const libc::c_char,
    b"six\x00" as *const u8 as *const libc::c_char,
    b"seven\x00" as *const u8 as *const libc::c_char,
    b"eight\x00" as *const u8 as *const libc::c_char,
    b"nine\x00" as *const u8 as *const libc::c_char,
    b"colon\x00" as *const u8 as *const libc::c_char,
    b"semicolon\x00" as *const u8 as *const libc::c_char,
    b"less\x00" as *const u8 as *const libc::c_char,
    b"equal\x00" as *const u8 as *const libc::c_char,
    b"greater\x00" as *const u8 as *const libc::c_char,
    b"question\x00" as *const u8 as *const libc::c_char,
    b"at\x00" as *const u8 as *const libc::c_char,
    b"A\x00" as *const u8 as *const libc::c_char,
    b"B\x00" as *const u8 as *const libc::c_char,
    b"C\x00" as *const u8 as *const libc::c_char,
    b"D\x00" as *const u8 as *const libc::c_char,
    b"E\x00" as *const u8 as *const libc::c_char,
    b"F\x00" as *const u8 as *const libc::c_char,
    b"G\x00" as *const u8 as *const libc::c_char,
    b"H\x00" as *const u8 as *const libc::c_char,
    b"I\x00" as *const u8 as *const libc::c_char,
    b"J\x00" as *const u8 as *const libc::c_char,
    b"K\x00" as *const u8 as *const libc::c_char,
    b"L\x00" as *const u8 as *const libc::c_char,
    b"M\x00" as *const u8 as *const libc::c_char,
    b"N\x00" as *const u8 as *const libc::c_char,
    b"O\x00" as *const u8 as *const libc::c_char,
    b"P\x00" as *const u8 as *const libc::c_char,
    b"Q\x00" as *const u8 as *const libc::c_char,
    b"R\x00" as *const u8 as *const libc::c_char,
    b"S\x00" as *const u8 as *const libc::c_char,
    b"T\x00" as *const u8 as *const libc::c_char,
    b"U\x00" as *const u8 as *const libc::c_char,
    b"V\x00" as *const u8 as *const libc::c_char,
    b"W\x00" as *const u8 as *const libc::c_char,
    b"X\x00" as *const u8 as *const libc::c_char,
    b"Y\x00" as *const u8 as *const libc::c_char,
    b"Z\x00" as *const u8 as *const libc::c_char,
    b"bracketleft\x00" as *const u8 as *const libc::c_char,
    b"backslash\x00" as *const u8 as *const libc::c_char,
    b"bracketright\x00" as *const u8 as *const libc::c_char,
    b"asciicircum\x00" as *const u8 as *const libc::c_char,
    b"underscore\x00" as *const u8 as *const libc::c_char,
    b"grave\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"c\x00" as *const u8 as *const libc::c_char,
    b"d\x00" as *const u8 as *const libc::c_char,
    b"e\x00" as *const u8 as *const libc::c_char,
    b"f\x00" as *const u8 as *const libc::c_char,
    b"g\x00" as *const u8 as *const libc::c_char,
    b"h\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"j\x00" as *const u8 as *const libc::c_char,
    b"k\x00" as *const u8 as *const libc::c_char,
    b"l\x00" as *const u8 as *const libc::c_char,
    b"m\x00" as *const u8 as *const libc::c_char,
    b"n\x00" as *const u8 as *const libc::c_char,
    b"o\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"t\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"v\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
    b"x\x00" as *const u8 as *const libc::c_char,
    b"y\x00" as *const u8 as *const libc::c_char,
    b"z\x00" as *const u8 as *const libc::c_char,
    b"braceleft\x00" as *const u8 as *const libc::c_char,
    b"bar\x00" as *const u8 as *const libc::c_char,
    b"braceright\x00" as *const u8 as *const libc::c_char,
    b"asciitilde\x00" as *const u8 as *const libc::c_char,
    b"Adieresis\x00" as *const u8 as *const libc::c_char,
    b"Aring\x00" as *const u8 as *const libc::c_char,
    b"Ccedilla\x00" as *const u8 as *const libc::c_char,
    b"Eacute\x00" as *const u8 as *const libc::c_char,
    b"Ntilde\x00" as *const u8 as *const libc::c_char,
    b"Odieresis\x00" as *const u8 as *const libc::c_char,
    b"Udieresis\x00" as *const u8 as *const libc::c_char,
    b"aacute\x00" as *const u8 as *const libc::c_char,
    b"agrave\x00" as *const u8 as *const libc::c_char,
    b"acircumflex\x00" as *const u8 as *const libc::c_char,
    b"adieresis\x00" as *const u8 as *const libc::c_char,
    b"atilde\x00" as *const u8 as *const libc::c_char,
    b"aring\x00" as *const u8 as *const libc::c_char,
    b"ccedilla\x00" as *const u8 as *const libc::c_char,
    b"eacute\x00" as *const u8 as *const libc::c_char,
    b"egrave\x00" as *const u8 as *const libc::c_char,
    b"ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"edieresis\x00" as *const u8 as *const libc::c_char,
    b"iacute\x00" as *const u8 as *const libc::c_char,
    b"igrave\x00" as *const u8 as *const libc::c_char,
    b"icircumflex\x00" as *const u8 as *const libc::c_char,
    b"idieresis\x00" as *const u8 as *const libc::c_char,
    b"ntilde\x00" as *const u8 as *const libc::c_char,
    b"oacute\x00" as *const u8 as *const libc::c_char,
    b"ograve\x00" as *const u8 as *const libc::c_char,
    b"ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"odieresis\x00" as *const u8 as *const libc::c_char,
    b"otilde\x00" as *const u8 as *const libc::c_char,
    b"uacute\x00" as *const u8 as *const libc::c_char,
    b"ugrave\x00" as *const u8 as *const libc::c_char,
    b"ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"udieresis\x00" as *const u8 as *const libc::c_char,
    b"dagger\x00" as *const u8 as *const libc::c_char,
    b"degree\x00" as *const u8 as *const libc::c_char,
    b"cent\x00" as *const u8 as *const libc::c_char,
    b"sterling\x00" as *const u8 as *const libc::c_char,
    b"section\x00" as *const u8 as *const libc::c_char,
    b"bullet\x00" as *const u8 as *const libc::c_char,
    b"paragraph\x00" as *const u8 as *const libc::c_char,
    b"germandbls\x00" as *const u8 as *const libc::c_char,
    b"registered\x00" as *const u8 as *const libc::c_char,
    b"copyright\x00" as *const u8 as *const libc::c_char,
    b"trademark\x00" as *const u8 as *const libc::c_char,
    b"acute\x00" as *const u8 as *const libc::c_char,
    b"dieresis\x00" as *const u8 as *const libc::c_char,
    b"notequal\x00" as *const u8 as *const libc::c_char,
    b"AE\x00" as *const u8 as *const libc::c_char,
    b"Oslash\x00" as *const u8 as *const libc::c_char,
    b"infinity\x00" as *const u8 as *const libc::c_char,
    b"plusminus\x00" as *const u8 as *const libc::c_char,
    b"lessequal\x00" as *const u8 as *const libc::c_char,
    b"greaterequal\x00" as *const u8 as *const libc::c_char,
    b"yen\x00" as *const u8 as *const libc::c_char,
    b"mu\x00" as *const u8 as *const libc::c_char,
    b"partialdiff\x00" as *const u8 as *const libc::c_char,
    b"summation\x00" as *const u8 as *const libc::c_char,
    b"product\x00" as *const u8 as *const libc::c_char,
    b"pi\x00" as *const u8 as *const libc::c_char,
    b"integral\x00" as *const u8 as *const libc::c_char,
    b"ordfeminine\x00" as *const u8 as *const libc::c_char,
    b"ordmasculine\x00" as *const u8 as *const libc::c_char,
    b"Omega\x00" as *const u8 as *const libc::c_char,
    b"ae\x00" as *const u8 as *const libc::c_char,
    b"oslash\x00" as *const u8 as *const libc::c_char,
    b"questiondown\x00" as *const u8 as *const libc::c_char,
    b"exclamdown\x00" as *const u8 as *const libc::c_char,
    b"logicalnot\x00" as *const u8 as *const libc::c_char,
    b"radical\x00" as *const u8 as *const libc::c_char,
    b"florin\x00" as *const u8 as *const libc::c_char,
    b"approxequal\x00" as *const u8 as *const libc::c_char,
    b"Delta\x00" as *const u8 as *const libc::c_char,
    b"guillemotleft\x00" as *const u8 as *const libc::c_char,
    b"guillemotright\x00" as *const u8 as *const libc::c_char,
    b"ellipsis\x00" as *const u8 as *const libc::c_char,
    b"nonbreakingspace\x00" as *const u8 as *const libc::c_char,
    b"Agrave\x00" as *const u8 as *const libc::c_char,
    b"Atilde\x00" as *const u8 as *const libc::c_char,
    b"Otilde\x00" as *const u8 as *const libc::c_char,
    b"OE\x00" as *const u8 as *const libc::c_char,
    b"oe\x00" as *const u8 as *const libc::c_char,
    b"endash\x00" as *const u8 as *const libc::c_char,
    b"emdash\x00" as *const u8 as *const libc::c_char,
    b"quotedblleft\x00" as *const u8 as *const libc::c_char,
    b"quotedblright\x00" as *const u8 as *const libc::c_char,
    b"quoteleft\x00" as *const u8 as *const libc::c_char,
    b"quoteright\x00" as *const u8 as *const libc::c_char,
    b"divide\x00" as *const u8 as *const libc::c_char,
    b"lozenge\x00" as *const u8 as *const libc::c_char,
    b"ydieresis\x00" as *const u8 as *const libc::c_char,
    b"Ydieresis\x00" as *const u8 as *const libc::c_char,
    b"fraction\x00" as *const u8 as *const libc::c_char,
    b"currency\x00" as *const u8 as *const libc::c_char,
    b"guilsinglleft\x00" as *const u8 as *const libc::c_char,
    b"guilsinglright\x00" as *const u8 as *const libc::c_char,
    b"fi\x00" as *const u8 as *const libc::c_char,
    b"fl\x00" as *const u8 as *const libc::c_char,
    b"daggerdbl\x00" as *const u8 as *const libc::c_char,
    b"periodcentered\x00" as *const u8 as *const libc::c_char,
    b"quotesinglbase\x00" as *const u8 as *const libc::c_char,
    b"quotedblbase\x00" as *const u8 as *const libc::c_char,
    b"perthousand\x00" as *const u8 as *const libc::c_char,
    b"Acircumflex\x00" as *const u8 as *const libc::c_char,
    b"Ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"Aacute\x00" as *const u8 as *const libc::c_char,
    b"Edieresis\x00" as *const u8 as *const libc::c_char,
    b"Egrave\x00" as *const u8 as *const libc::c_char,
    b"Iacute\x00" as *const u8 as *const libc::c_char,
    b"Icircumflex\x00" as *const u8 as *const libc::c_char,
    b"Idieresis\x00" as *const u8 as *const libc::c_char,
    b"Igrave\x00" as *const u8 as *const libc::c_char,
    b"Oacute\x00" as *const u8 as *const libc::c_char,
    b"Ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"apple\x00" as *const u8 as *const libc::c_char,
    b"Ograve\x00" as *const u8 as *const libc::c_char,
    b"Uacute\x00" as *const u8 as *const libc::c_char,
    b"Ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"Ugrave\x00" as *const u8 as *const libc::c_char,
    b"dotlessi\x00" as *const u8 as *const libc::c_char,
    b"circumflex\x00" as *const u8 as *const libc::c_char,
    b"tilde\x00" as *const u8 as *const libc::c_char,
    b"macron\x00" as *const u8 as *const libc::c_char,
    b"breve\x00" as *const u8 as *const libc::c_char,
    b"dotaccent\x00" as *const u8 as *const libc::c_char,
    b"ring\x00" as *const u8 as *const libc::c_char,
    b"cedilla\x00" as *const u8 as *const libc::c_char,
    b"hungarumlaut\x00" as *const u8 as *const libc::c_char,
    b"ogonek\x00" as *const u8 as *const libc::c_char,
    b"caron\x00" as *const u8 as *const libc::c_char,
    b"Lslash\x00" as *const u8 as *const libc::c_char,
    b"lslash\x00" as *const u8 as *const libc::c_char,
    b"Scaron\x00" as *const u8 as *const libc::c_char,
    b"scaron\x00" as *const u8 as *const libc::c_char,
    b"Zcaron\x00" as *const u8 as *const libc::c_char,
    b"zcaron\x00" as *const u8 as *const libc::c_char,
    b"brokenbar\x00" as *const u8 as *const libc::c_char,
    b"Eth\x00" as *const u8 as *const libc::c_char,
    b"eth\x00" as *const u8 as *const libc::c_char,
    b"Yacute\x00" as *const u8 as *const libc::c_char,
    b"yacute\x00" as *const u8 as *const libc::c_char,
    b"Thorn\x00" as *const u8 as *const libc::c_char,
    b"thorn\x00" as *const u8 as *const libc::c_char,
    b"minus\x00" as *const u8 as *const libc::c_char,
    b"multiply\x00" as *const u8 as *const libc::c_char,
    b"onesuperior\x00" as *const u8 as *const libc::c_char,
    b"twosuperior\x00" as *const u8 as *const libc::c_char,
    b"threesuperior\x00" as *const u8 as *const libc::c_char,
    b"onehalf\x00" as *const u8 as *const libc::c_char,
    b"onequarter\x00" as *const u8 as *const libc::c_char,
    b"threequarters\x00" as *const u8 as *const libc::c_char,
    b"franc\x00" as *const u8 as *const libc::c_char,
    b"Gbreve\x00" as *const u8 as *const libc::c_char,
    b"gbreve\x00" as *const u8 as *const libc::c_char,
    b"Idotaccent\x00" as *const u8 as *const libc::c_char,
    b"Scedilla\x00" as *const u8 as *const libc::c_char,
    b"scedilla\x00" as *const u8 as *const libc::c_char,
    b"Cacute\x00" as *const u8 as *const libc::c_char,
    b"cacute\x00" as *const u8 as *const libc::c_char,
    b"Ccaron\x00" as *const u8 as *const libc::c_char,
    b"ccaron\x00" as *const u8 as *const libc::c_char,
    b"dcroat\x00" as *const u8 as *const libc::c_char,
];