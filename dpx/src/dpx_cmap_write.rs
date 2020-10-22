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
#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use std::ptr;

use crate::warn;

use super::dpx_cid::{CSI_IDENTITY, CSI_UNICODE};
use super::dpx_cmap::{CMap_get_CIDSysInfo, CMap_is_valid};
use super::dpx_mem::new;
use crate::dpx_pdfobj::{pdf_copy_name, pdf_dict, pdf_stream, pdf_string, STREAM_COMPRESS};
use crate::shims::sprintf;
use libc::{free, memcmp, memset};

use crate::bridge::size_t;

use super::dpx_cmap::mapDef;
use super::dpx_cmap::CMap;

/*
 * References:
 *
 *  PostScript Language Reference Manual, 3rd. ed. (Adobe Systems Inc.)
 *    5.11.4 CMap Dictionaries
 *    5.11.5 FMapType 9 Composite Fonts
 *  Building CMap Files for CID-Keyed Fonts, Adobe Technical Note #5099
 *  CID-Keyed Font Technology Overview, Adobe Technical Note #5092
 *  Adobe CMap and CIDFont Files Specification, Adobe Technical Specification #5014
 *
 *  Undefined Character Handling:
 *    PLRM 3rd. ed., sec. 5.11.5., "Handling Undefined Characters"
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct sbuf {
    pub(crate) buf: *mut i8,
    pub(crate) curptr: *mut i8,
    pub(crate) limptr: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct C2RustUnnamed_1 {
    pub(crate) start: i32,
    pub(crate) count: i32,
}
unsafe fn block_count(mtab: *mut mapDef, mut c: i32) -> size_t {
    let mut count: size_t = 0i32 as size_t;
    let n = (*mtab.offset(c as isize)).len.wrapping_sub(1);
    c += 1i32;
    while c < 256i32 {
        if (*mtab.offset(c as isize)).flag & 1i32 << 4i32 != 0
            || (if (*mtab.offset(c as isize)).flag & 0xfi32 != 0i32 {
                1i32
            } else {
                0i32
            }) == 0
            || (*mtab.offset(c as isize)).flag & 0xfi32 != 1i32 << 0i32
                && (*mtab.offset(c as isize)).flag & 0xfi32 != 1i32 << 2i32
            || (*mtab.offset((c - 1i32) as isize)).len != (*mtab.offset(c as isize)).len
        {
            break;
        }
        if !(memcmp(
            (*mtab.offset((c - 1i32) as isize)).code as *const libc::c_void,
            (*mtab.offset(c as isize)).code as *const libc::c_void,
            n as _,
        ) == 0
            && (*(*mtab.offset((c - 1i32) as isize)).code.offset(n as isize) as i32) < 255i32
            && *(*mtab.offset((c - 1i32) as isize)).code.offset(n as isize) as i32 + 1i32
                == *(*mtab.offset(c as isize)).code.offset(n as isize) as i32)
        {
            break;
        }
        count = count.wrapping_add(1);
        c += 1
    }
    count
}
unsafe fn sputx(c: u8, s: *mut *mut i8, end: *mut i8) -> i32 {
    let hi: i8 = (c as i32 >> 4i32) as i8;
    let lo: i8 = (c as i32 & 0xfi32) as i8;
    if (*s).offset(2) > end {
        panic!("Buffer overflow.");
    }
    **s = (if (hi as i32) < 10i32 {
        hi as i32 + '0' as i32
    } else {
        hi as i32 + '7' as i32
    }) as i8;
    *(*s).offset(1) = (if (lo as i32) < 10i32 {
        lo as i32 + '0' as i32
    } else {
        lo as i32 + '7' as i32
    }) as i8;
    *s = (*s).offset(2);
    2i32
}
unsafe fn write_map(
    mtab: *mut mapDef,
    mut count: size_t,
    codestr: *mut u8,
    depth: size_t,
    mut wbuf: *mut sbuf,
    stream: &mut pdf_stream,
) -> i32 {
    /* Must be greater than 1 */
    let mut blocks: [C2RustUnnamed_1; 129] = [C2RustUnnamed_1 { start: 0, count: 0 }; 129];
    let mut num_blocks: size_t = 0i32 as size_t;
    let mut c = 0;
    while c < 256i32 as u64 {
        *codestr.offset(depth as isize) = (c & 0xffi32 as u64) as u8;
        if (*mtab.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
            let mtab1 = (*mtab.offset(c as isize)).next;
            count = write_map(mtab1, count, codestr, depth.wrapping_add(1), wbuf, stream) as size_t
        } else if if (*mtab.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            match (*mtab.offset(c as isize)).flag & 0xfi32 {
                1 | 4 => {
                    let block_length = block_count(mtab, c as i32);
                    if block_length >= 2 {
                        blocks[num_blocks as usize].start = c as i32;
                        blocks[num_blocks as usize].count = block_length as i32;
                        num_blocks = num_blocks.wrapping_add(1);
                        c = (c as u64).wrapping_add(block_length as _) as _
                    } else {
                        *(*wbuf).curptr = '<' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        for i in 0..=depth {
                            sputx(
                                *codestr.offset(i as isize),
                                &mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                        }
                        *(*wbuf).curptr = '>' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *(*wbuf).curptr = ' ' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *(*wbuf).curptr = '<' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        for i in 0..(*mtab.offset(c as isize)).len {
                            sputx(
                                *(*mtab.offset(c as isize)).code.offset(i as isize),
                                &mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                        }
                        *(*wbuf).curptr = '>' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *(*wbuf).curptr = '\n' as i32 as i8;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        count = count.wrapping_add(1)
                    }
                }
                2 => {
                    panic!("{}: Unexpected error...", "CMap",);
                }
                8 => {}
                _ => {
                    panic!(
                        "{}: Unknown mapping type: {}",
                        "CMap",
                        (*mtab.offset(c as isize)).flag & 0xfi32,
                    );
                }
            }
        }
        /* Flush if necessary */
        if count >= 100 || (*wbuf).curptr >= (*wbuf).limptr {
            if count > 100 {
                panic!("Unexpected error....: {}", count,);
            }
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add(
                (*wbuf).buf as *const libc::c_void,
                (*wbuf).curptr.offset_from((*wbuf).buf) as i64 as i32,
            );
            (*wbuf).curptr = (*wbuf).buf;
            stream.add_str("endbfchar\n");
            count = 0i32 as size_t
        }
        c = c.wrapping_add(1)
    }
    if num_blocks > 0 {
        if count > 0 {
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add(
                (*wbuf).buf as *const libc::c_void,
                (*wbuf).curptr.offset_from((*wbuf).buf) as i64 as i32,
            );
            (*wbuf).curptr = (*wbuf).buf;
            stream.add_str("endbfchar\n");
            count = 0i32 as size_t
        }
        stream.add_str(&format!("{} beginbfrange\n", num_blocks));
        for i in 0..num_blocks {
            let c = blocks[i as usize].start as size_t;
            *(*wbuf).curptr = '<' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            for j in 0..depth {
                sputx(
                    *codestr.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
            }
            sputx(c as u8, &mut (*wbuf).curptr, (*wbuf).limptr);
            *(*wbuf).curptr = '>' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *(*wbuf).curptr = ' ' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *(*wbuf).curptr = '<' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            for j in 0..depth {
                sputx(
                    *codestr.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
            }
            sputx(
                c.wrapping_add(blocks[i as usize].count as _) as u8,
                &mut (*wbuf).curptr,
                (*wbuf).limptr,
            );
            *(*wbuf).curptr = '>' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *(*wbuf).curptr = ' ' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *(*wbuf).curptr = '<' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            for j in 0..(*mtab.offset(c as isize)).len {
                sputx(
                    *(*mtab.offset(c as isize)).code.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
            }
            *(*wbuf).curptr = '>' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *(*wbuf).curptr = '\n' as i32 as i8;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
        }
        stream.add(
            (*wbuf).buf as *const libc::c_void,
            (*wbuf).curptr.offset_from((*wbuf).buf) as i64 as i32,
        );
        (*wbuf).curptr = (*wbuf).buf;
        stream.add_str("endbfrange\n");
    }
    count as i32
}

pub(crate) unsafe fn CMap_create_stream(cmap: *mut CMap) -> Option<pdf_stream> {
    let mut wbuf: sbuf = sbuf {
        buf: ptr::null_mut(),
        curptr: ptr::null_mut(),
        limptr: ptr::null_mut(),
    };
    if cmap.is_null() || !CMap_is_valid(cmap) {
        warn!("Invalid CMap");
        return None;
    }
    if (*cmap).type_0 == 0i32 {
        return None;
    }
    let mut stream = pdf_stream::new(STREAM_COMPRESS);
    let stream_dict = stream.get_dict_mut();
    let mut csi = CMap_get_CIDSysInfo(cmap);
    if csi.is_null() {
        csi = if (*cmap).type_0 != 2i32 {
            &mut CSI_IDENTITY
        } else {
            &mut CSI_UNICODE
        }
    }
    if (*cmap).type_0 != 2i32 {
        let mut csi_dict = pdf_dict::new();
        csi_dict.set("Registry", pdf_string::new((*csi).registry.as_bytes()));
        csi_dict.set("Ordering", pdf_string::new((*csi).ordering.as_bytes()));
        csi_dict.set("Supplement", (*csi).supplement as f64);
        stream_dict.set("Type", "CMap");
        stream_dict.set("CMapName", pdf_copy_name((*cmap).name));
        stream_dict.set("CIDSystemInfo", csi_dict);
        if (*cmap).wmode != 0i32 {
            stream_dict.set("WMode", (*cmap).wmode as f64);
        }
    }
    /* TODO:
     * Predefined CMaps need not to be embedded.
     */
    if !(*cmap).useCMap.is_null() {
        panic!("UseCMap found (not supported yet)...");
    }
    wbuf.buf = new((4096_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    let codestr = new(((*cmap).profile.maxBytesIn as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memset(
        codestr as *mut libc::c_void,
        0i32,
        (*cmap).profile.maxBytesIn as _,
    );
    wbuf.curptr = wbuf.buf;
    wbuf.limptr = wbuf
        .buf
        .offset(4096)
        .offset(
            -((2i32 as u64).wrapping_mul(
                (*cmap)
                    .profile
                    .maxBytesIn
                    .wrapping_add((*cmap).profile.maxBytesOut) as _,
            ) as isize),
        )
        .offset(16);
    /* Start CMap */
    stream.add_str("/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n");
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapName /%s def\n\x00" as *const u8 as *const i8,
        (*cmap).name,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapType %d def\n\x00" as *const u8 as *const i8,
        (*cmap).type_0,
    ) as isize);
    if (*cmap).wmode != 0i32 && (*cmap).type_0 != 2i32 {
        wbuf.curptr = wbuf.curptr.offset(sprintf(
            wbuf.curptr,
            b"/WMode %d def\n\x00" as *const u8 as *const i8,
            (*cmap).wmode,
        ) as isize)
    }
    let s = format!(
        "/CIDSystemInfo <<\n  /Registry ({})\n  /Ordering ({})\n  /Supplement {}\n>> def\n",
        (*csi).registry,
        (*csi).ordering,
        (*csi).supplement,
    );
    s.as_bytes()
        .as_ptr()
        .copy_to_nonoverlapping(wbuf.curptr as *mut u8, s.len());
    wbuf.curptr = wbuf.curptr.add(s.len());
    stream.add(
        wbuf.buf as *const libc::c_void,
        wbuf.curptr.offset_from(wbuf.buf) as i64 as i32,
    );
    wbuf.curptr = wbuf.buf;
    /* codespacerange */
    let ranges = (*cmap).codespace.ranges;
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"%d begincodespacerange\n\x00" as *const u8 as *const i8,
        (*cmap).codespace.num,
    ) as isize);
    for i in 0..(*cmap).codespace.num as u64 {
        *wbuf.curptr = '<' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
        for j in 0..(*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeLo.offset(j as isize),
                &mut wbuf.curptr,
                wbuf.limptr,
            );
        }
        *wbuf.curptr = '>' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
        *wbuf.curptr = ' ' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
        *wbuf.curptr = '<' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
        for j in 0..(*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeHi.offset(j as isize),
                &mut wbuf.curptr,
                wbuf.limptr,
            );
        }
        *wbuf.curptr = '>' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
        *wbuf.curptr = '\n' as i32 as i8;
        wbuf.curptr = wbuf.curptr.offset(1);
    }
    stream.add(
        wbuf.buf as *const libc::c_void,
        wbuf.curptr.offset_from(wbuf.buf) as i64 as i32,
    );
    wbuf.curptr = wbuf.buf;
    stream.add_str("endcodespacerange\n");
    /* CMap body */
    if !(*cmap).mapTbl.is_null() {
        let count = write_map((*cmap).mapTbl, 0, codestr, 0, &mut wbuf, &mut stream) as size_t; /* Top node */
        if count > 0 {
            /* Flush */
            if count > 100 {
                panic!("Unexpected error....: {}", count,);
            }
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add(
                wbuf.buf as *const libc::c_void,
                wbuf.curptr.offset_from(wbuf.buf) as i64 as i32,
            );
            stream.add_str("endbfchar\n");
            wbuf.curptr = wbuf.buf
        }
    }
    /* End CMap */
    stream.add_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    free(codestr as *mut libc::c_void);
    free(wbuf.buf as *mut libc::c_void);
    Some(stream)
}
