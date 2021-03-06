#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;

pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn UC_is_valid(mut ucv: i32) -> bool {
    !(ucv < 0i32 || ucv as i64 > 0x10ffff || ucv as i64 >= 0xd800 && ucv as i64 <= 0xdfff)
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_is_valid_string(
    mut p: *const u8,
    mut endptr: *const u8,
) -> bool {
    if p.offset(1) >= endptr {
        return false;
    }
    while p < endptr {
        let mut ucv: i32 = UC_UTF16BE_decode_char(&mut p, endptr);
        if !UC_is_valid(ucv) {
            return false;
        }
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF8_is_valid_string(mut p: *const u8, mut endptr: *const u8) -> bool {
    if p.offset(1) >= endptr {
        return false;
    }
    while p < endptr {
        let mut ucv: i32 = UC_UTF8_decode_char(&mut p, endptr);
        if !UC_is_valid(ucv) {
            return false;
        }
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_decode_char(
    mut pp: *mut *const u8,
    mut endptr: *const u8,
) -> i32 {
    let mut p: *const u8 = *pp;
    let mut ucv: i32 = -1i32;
    let mut first: u16 = 0;
    let mut second: u16 = 0;
    if p.offset(1) >= endptr {
        return -1i32;
    }
    first = ((*p.offset(0) as i32) << 8i32 | *p.offset(1) as i32) as u16;
    p = p.offset(2);
    if first as u32 >= 0xd800u32 && (first as u32) < 0xdc00u32 {
        if p.offset(1) >= endptr {
            return -1i32;
        }
        second = ((*p.offset(0) as i32) << 8i32 | *p.offset(1) as i32) as u16;
        p = p.offset(2);
        ucv = (second as u32 & 0x3ffu32) as i32;
        ucv = (ucv as u32 | (first as u32 & 0x3ffu32) << 10i32) as i32;
        ucv += 0x10000i32
    } else if first as u32 >= 0xdc00u32 && (first as u32) < 0xe000u32 {
        return -1i32;
    } else {
        ucv = first as i32
    }
    *pp = p;
    ucv
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_encode_char(
    mut ucv: i32,
    mut pp: *mut *mut u8,
    mut endptr: *mut u8,
) -> size_t {
    let mut count: i32 = 0i32;
    let mut p: *mut u8 = *pp;
    if ucv >= 0i32 && ucv <= 0xffffi32 {
        if p.offset(2) >= endptr {
            return 0i32 as size_t;
        }
        *p.offset(0) = (ucv >> 8i32 & 0xffi32) as u8;
        *p.offset(1) = (ucv & 0xffi32) as u8;
        count = 2i32
    } else if ucv >= 0x10000i32 && ucv <= 0x10ffffi32 {
        let mut high: u16 = 0;
        let mut low: u16 = 0;
        if p.offset(4) >= endptr {
            return 0i32 as size_t;
        }
        ucv -= 0x10000i32;
        high = ((ucv >> 10i32) as u32).wrapping_add(0xd800u32) as u16;
        low = (ucv as u32 & 0x3ffu32).wrapping_add(0xdc00u32) as u16;
        *p.offset(0) = (high as i32 >> 8i32 & 0xffi32) as u8;
        *p.offset(1) = (high as i32 & 0xffi32) as u8;
        *p.offset(2) = (low as i32 >> 8i32 & 0xffi32) as u8;
        *p.offset(3) = (low as i32 & 0xffi32) as u8;
        count = 4i32
    } else {
        if p.offset(2) >= endptr {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xfffdi32 >> 8i32 & 0xffi32) as u8;
        *p.offset(1) = (0xfffdi32 & 0xffi32) as u8;
        count = 2i32
    }
    *pp = (*pp).offset(count as isize);
    count as size_t
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF8_decode_char(mut pp: *mut *const u8, mut endptr: *const u8) -> i32 {
    let mut p: *const u8 = *pp;
    let mut ucv: i32 = 0;
    let fresh0 = p;
    p = p.offset(1);
    let mut c: u8 = *fresh0;
    let mut nbytes: i32 = 0;
    if c as i32 <= 0x7fi32 {
        ucv = c as i32;
        nbytes = 0i32
    } else if c as i32 & 0xe0i32 == 0xc0i32 {
        /* 110x xxxx */
        ucv = c as i32 & 31i32;
        nbytes = 1i32
    } else if c as i32 & 0xf0i32 == 0xe0i32 {
        /* 1110 xxxx */
        ucv = c as i32 & 0xfi32;
        nbytes = 2i32
    } else if c as i32 & 0xf8i32 == 0xf0i32 {
        /* 1111 0xxx */
        ucv = c as i32 & 0x7i32;
        nbytes = 3i32
    } else if c as i32 & 0xfci32 == 0xf8i32 {
        /* 1111 10xx */
        ucv = c as i32 & 0x3i32;
        nbytes = 4i32
    } else if c as i32 & 0xfei32 == 0xfci32 {
        /* 1111 110x */
        ucv = c as i32 & 0x1i32;
        nbytes = 5i32
    } else {
        return -1i32;
    }
    if p.offset(nbytes as isize) > endptr {
        return -1i32;
    }
    loop {
        let fresh1 = nbytes;
        nbytes = nbytes - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        let fresh2 = p;
        p = p.offset(1);
        c = *fresh2;
        if c as i32 & 0xc0i32 != 0x80i32 {
            return -1i32;
        }
        ucv = ucv << 6i32 | c as i32 & 0x3fi32
    }
    *pp = p;
    ucv
}
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
pub unsafe extern "C" fn UC_UTF8_encode_char(
    mut ucv: i32,
    mut pp: *mut *mut u8,
    mut endptr: *mut u8,
) -> size_t {
    let mut count: i32 = 0i32;
    let mut p: *mut u8 = *pp;
    assert!(!pp.is_null() && !(*pp).is_null() && !endptr.is_null());
    if !UC_is_valid(ucv) {
        return 0i32 as size_t;
    }
    if ucv < 0x7fi32 {
        if p >= endptr.offset(-1) {
            return 0i32 as size_t;
        }
        *p.offset(0) = ucv as u8;
        count = 1i32
    } else if ucv <= 0x7ffi32 {
        if p >= endptr.offset(-2) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xc0i32 | ucv >> 6i32) as u8;
        *p.offset(1) = (0x80i32 | ucv & 0x3fi32) as u8;
        count = 2i32
    } else if ucv <= 0xffffi32 {
        if p >= endptr.offset(-3) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xe0i32 | ucv >> 12i32) as u8;
        *p.offset(1) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as u8;
        *p.offset(2) = (0x80i32 | ucv & 0x3fi32) as u8;
        count = 3i32
    } else if ucv <= 0x1fffffi32 {
        if p >= endptr.offset(-4) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xf0i32 | ucv >> 18i32) as u8;
        *p.offset(1) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as u8;
        *p.offset(2) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as u8;
        *p.offset(3) = (0x80i32 | ucv & 0x3fi32) as u8;
        count = 4i32
    } else if ucv <= 0x3ffffffi32 {
        if p >= endptr.offset(-5) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xf8i32 | ucv >> 24i32) as u8;
        *p.offset(1) = (0x80i32 | ucv >> 18i32 & 0x3fi32) as u8;
        *p.offset(2) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as u8;
        *p.offset(3) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as u8;
        *p.offset(4) = (0x80i32 | ucv & 0x3fi32) as u8;
        count = 5i32
    } else if ucv <= 0x7fffffffi32 {
        if p >= endptr.offset(-6) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xfci32 | ucv >> 30i32) as u8;
        *p.offset(1) = (0x80i32 | ucv >> 24i32 & 0x3fi32) as u8;
        *p.offset(2) = (0x80i32 | ucv >> 18i32 & 0x3fi32) as u8;
        *p.offset(3) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as u8;
        *p.offset(4) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as u8;
        *p.offset(5) = (0x80i32 | ucv & 0x3fi32) as u8;
        count = 6i32
    }
    *pp = (*pp).offset(count as isize);
    count as size_t
}
