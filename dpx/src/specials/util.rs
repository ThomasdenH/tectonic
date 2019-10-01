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
    non_camel_case_types,
    non_snake_case,
    unused_mut
)]

use super::{spc_arg, spc_env, spc_warn};
use crate::dpx_dpxutil::{parse_c_ident, parse_float_decimal};
use crate::dpx_pdfcolor::pdf_color;
use crate::dpx_pdfdev::{pdf_tmatrix, transform_info};
use crate::dpx_pdfparse::skip_white;
use crate::mfree;
use crate::streq_ptr;
use libc::{atof, free, memcmp, strcmp, strlen};
use crate::shims::strcasecmp;

/* Color names */
#[derive(Clone)]
#[repr(C)]
pub struct colordef_ {
    pub key: *const i8,
    pub color: pdf_color,
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp; /* 360 / 60 */
    while p < endptr && (*p as i32 & !0x7fi32 == 0i32 && crate::isblank(*p as _) != 0) {
        p = p.offset(1)
    }
    *pp = p;
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_numbers(
    mut values: *mut f64,
    mut num_values: i32,
    mut args: *mut spc_arg,
) -> i32 {
    skip_blank(&mut (*args).curptr, (*args).endptr);
    let mut count = 0;
    while count < num_values && (*args).curptr < (*args).endptr {
        let q = parse_float_decimal(&mut (*args).curptr, (*args).endptr);
        if q.is_null() {
            break;
        }
        *values.offset(count as isize) = atof(q);
        free(q as *mut libc::c_void);
        skip_blank(&mut (*args).curptr, (*args).endptr);
        count += 1
    }
    count
}
unsafe extern "C" fn rgb_color_from_hsv(color: &mut pdf_color, mut h: f64, mut s: f64, mut v: f64) {
    let mut b = v;
    let mut g = b;
    let mut r = g;
    if s != 0.0f64 {
        let h6 = h * 6i32 as f64;
        let i = h6 as i32;
        let f = h6 - i as f64;
        let v1 = v * (1i32 as f64 - s);
        let v2 = v * (1i32 as f64 - s * f);
        let v3 = v * (1i32 as f64 - s * (1i32 as f64 - f));
        match i {
            0 => {
                r = v;
                g = v3;
                b = v1
            }
            1 => {
                r = v2;
                g = v;
                b = v1
            }
            2 => {
                r = v1;
                g = v;
                b = v3
            }
            3 => {
                r = v1;
                g = v2;
                b = v
            }
            4 => {
                r = v3;
                g = v1;
                b = v
            }
            5 => {
                r = v;
                g = v1;
                b = v2
            }
            6 => {
                r = v;
                g = v1;
                b = v2
            }
            _ => {}
        }
    }
    *color = pdf_color::rgb(r, g, b).unwrap();
}
unsafe extern "C" fn spc_read_color_color(
    mut spe: *mut spc_env,
    colorspec: &mut pdf_color,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut cv: [f64; 4] = [0.; 4];
    let mut error: i32 = 0i32;
    let q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        spc_warn(
            spe,
            b"No valid color specified?\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(q, b"rgb\x00" as *const u8 as *const i8) {
        /* Handle rgb color */
        let nc = spc_util_read_numbers(cv.as_mut_ptr(), 3i32, ap);
        if nc != 3i32 {
            spc_warn(
                spe,
                b"Invalid value for RGB color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            *colorspec = pdf_color::rgb(cv[0], cv[1], cv[2]).unwrap();
        }
    } else if streq_ptr(q, b"cmyk\x00" as *const u8 as *const i8) {
        /* Handle cmyk color */
        let nc = spc_util_read_numbers(cv.as_mut_ptr(), 4i32, ap);
        if nc != 4i32 {
            spc_warn(
                spe,
                b"Invalid value for CMYK color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            *colorspec = pdf_color::cmyk(cv[0], cv[1], cv[2], cv[3]).unwrap();
        }
    } else if streq_ptr(q, b"gray\x00" as *const u8 as *const i8) {
        /* Handle gray */
        let nc = spc_util_read_numbers(cv.as_mut_ptr(), 1i32, ap);
        if nc != 1i32 {
            spc_warn(
                spe,
                b"Invalid value for gray color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            *colorspec = pdf_color::gray(cv[0]).unwrap();
        }
    } else if streq_ptr(q, b"spot\x00" as *const u8 as *const i8) {
        /* Handle spot colors */
        let mut color_name: *mut i8 = parse_c_ident(&mut (*ap).curptr, (*ap).endptr); /* Must be a "named" color */
        if color_name.is_null() {
            spc_warn(
                spe,
                b"No valid spot color name specified?\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        let nc = spc_util_read_numbers(cv.as_mut_ptr(), 1i32, ap);
        if nc != 1i32 {
            spc_warn(
                spe,
                b"Invalid value for spot color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32;
            free(color_name as *mut libc::c_void);
        } else {
            *colorspec = pdf_color::spot(color_name, cv[0]).unwrap();
        }
    } else if streq_ptr(q, b"hsb\x00" as *const u8 as *const i8) {
        let nc = spc_util_read_numbers(cv.as_mut_ptr(), 3i32, ap);
        if nc != 3i32 {
            spc_warn(
                spe,
                b"Invalid value for HSB color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            rgb_color_from_hsv(colorspec, cv[0], cv[1], cv[2]);
            spc_warn(
                spe,
                b"HSB color converted to RGB: hsb: <%g, %g, %g> ==> rgb: <%g, %g, %g>\x00"
                    as *const u8 as *const i8,
                cv[0],
                cv[1],
                cv[2],
                (*colorspec).values[0],
                (*colorspec).values[1],
                (*colorspec).values[2],
            );
        }
    } else {
        error = pdf_color_namedcolor(colorspec, q);
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized color name: %s\x00" as *const u8 as *const i8,
                q,
            );
        }
    }
    free(q as *mut libc::c_void);
    error
}
/* Argument for this is PDF_Number or PDF_Array.
 * But we ignore that since we don't want to add
 * dependency to pdfxxx and @foo can not be
 * allowed for color specification. "pdf" here
 * means pdf: special syntax.
 */
unsafe extern "C" fn spc_read_color_pdf(
    mut spe: *mut spc_env,
    colorspec: &mut pdf_color,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut cv: [f64; 4] = [0.; 4]; /* at most four */
    let mut isarry: i32 = 0i32;
    let mut error: i32 = 0i32;
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if *(*ap).curptr.offset(0) as i32 == '[' as i32 {
        (*ap).curptr = (*ap).curptr.offset(1);
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        isarry = 1i32
    }
    let nc = spc_util_read_numbers(cv.as_mut_ptr(), 4i32, ap);
    match nc {
        1 => {
            *colorspec = pdf_color::gray(cv[0]).unwrap();
        }
        3 => {
            *colorspec = pdf_color::rgb(cv[0], cv[1], cv[2]).unwrap();
        }
        4 => {
            *colorspec = pdf_color::cmyk(cv[0], cv[1], cv[2], cv[3]).unwrap();
        }
        _ => {
            /* Try to read the color names defined in dvipsname.def */
            let q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
            if q.is_null() {
                spc_warn(
                    spe,
                    b"No valid color specified?\x00" as *const u8 as *const i8,
                );
                return -1i32;
            }
            error = pdf_color_namedcolor(colorspec, q);
            if error != 0 {
                spc_warn(
                    spe,
                    b"Unrecognized color name: %s, keep the current color\x00" as *const u8
                        as *const i8,
                    q,
                );
            }
            free(q as *mut libc::c_void);
        }
    }
    if isarry != 0 {
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        if (*ap).curptr >= (*ap).endptr || *(*ap).curptr.offset(0) as i32 != ']' as i32 {
            spc_warn(
                spe,
                b"Unbalanced \'[\' and \']\' in color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            (*ap).curptr = (*ap).curptr.offset(1)
        }
    }
    error
}
/* This is for reading *single* color specification. */
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_colorspec(
    mut spe: *mut spc_env,
    colorspec: &mut pdf_color,
    mut ap: *mut spc_arg,
    mut syntax: i32,
) -> i32 {
    assert!(!spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -1i32;
    }
    if syntax != 0 {
        return spc_read_color_color(spe, colorspec, ap);
    } else {
        return spc_read_color_pdf(spe, colorspec, ap);
    };
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_pdfcolor(
    mut spe: *mut spc_env,
    colorspec: &mut pdf_color,
    mut ap: *mut spc_arg,
    defaultcolor: Option<&pdf_color>,
) -> i32 {
    assert!(!spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -1i32;
    }
    let mut error = spc_read_color_pdf(spe, colorspec, ap);
    if error < 0i32 {
        if let Some(dc) = defaultcolor {
            *colorspec = dc.clone();
            error = 0i32
        }
    }
    error
}
/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn spc_util_read_length(
    mut spe: *mut spc_env,
    mut vp: *mut f64,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut u: f64 = 1.0f64;
    let mut ukeys: [*const i8; 10] = [
        b"pt\x00" as *const u8 as *const i8,
        b"in\x00" as *const u8 as *const i8,
        b"cm\x00" as *const u8 as *const i8,
        b"mm\x00" as *const u8 as *const i8,
        b"bp\x00" as *const u8 as *const i8,
        b"pc\x00" as *const u8 as *const i8,
        b"dd\x00" as *const u8 as *const i8,
        b"cc\x00" as *const u8 as *const i8,
        b"sp\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut error: i32 = 0i32;
    let q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr); /* inverse magnify */
    if q.is_null() {
        return -1i32;
    }
    let v = atof(q);
    free(q as *mut libc::c_void);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    let mut q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        let mut qq: *mut i8 = q;
        if strlen(q) >= strlen(b"true\x00" as *const u8 as *const i8)
            && memcmp(
                q as *const libc::c_void,
                b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"true\x00" as *const u8 as *const i8),
            ) == 0
        {
            u /= if (*spe).mag != 0.0f64 {
                (*spe).mag
            } else {
                1.0f64
            };
            q = q.offset(strlen(b"true\x00" as *const u8 as *const i8) as isize);
            if *q == 0 {
                free(qq as *mut libc::c_void);
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
                qq = q
            }
        }
        if !q.is_null() {
            let mut k = 0;
            while !ukeys[k].is_null() && strcmp(ukeys[k], q) != 0 {
                k += 1
            }
            match k {
                0 => u *= 72.0f64 / 72.27f64,
                1 => u *= 72.0f64,
                2 => u *= 72.0f64 / 2.54f64,
                3 => u *= 72.0f64 / 25.4f64,
                4 => u *= 1.0f64,
                5 => u *= 12.0f64 * 72.0f64 / 72.27f64,
                6 => u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                7 => u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                8 => u *= 72.0f64 / (72.27f64 * 65536i32 as f64),
                _ => {
                    spc_warn(
                        spe,
                        b"Unknown unit of measure: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    error = -1i32
                }
            }
            free(qq as *mut libc::c_void);
        } else {
            spc_warn(
                spe,
                b"Missing unit of measure after \"true\"\x00" as *const u8 as *const i8,
            );
            error = -1i32
        }
    }
    *vp = v * u;
    error
}
/*
 * Compute a transformation matrix
 * transformations are applied in the following
 * order: scaling, rotate, displacement.
 */
extern "C" fn make_transmatrix(
    M: &mut pdf_tmatrix,
    mut xoffset: f64,
    mut yoffset: f64,
    mut xscale: f64,
    mut yscale: f64,
    mut rotate: f64,
) {
    let (s, c) = rotate.sin_cos();
    M.a = xscale * c;
    M.b = xscale * s;
    M.c = -yscale * s;
    M.d = yscale * c;
    M.e = xoffset;
    M.f = yoffset;
}
unsafe extern "C" fn spc_read_dimtrns_dvips(
    mut spe: *mut spc_env,
    t: &mut transform_info,
    mut ap: *mut spc_arg,
) -> i32 {
    const _DTKEYS: [*const i8; 15] = [
        b"hoffset\x00" as *const u8 as *const i8,
        b"voffset\x00" as *const u8 as *const i8,
        b"hsize\x00" as *const u8 as *const i8,
        b"vsize\x00" as *const u8 as *const i8,
        b"hscale\x00" as *const u8 as *const i8,
        b"vscale\x00" as *const u8 as *const i8,
        b"angle\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"llx\x00" as *const u8 as *const i8,
        b"lly\x00" as *const u8 as *const i8,
        b"urx\x00" as *const u8 as *const i8,
        b"ury\x00" as *const u8 as *const i8,
        b"rwi\x00" as *const u8 as *const i8,
        b"rhi\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut error: i32 = 0i32;
    let mut rotate = 0.0f64;
    let mut yoffset = rotate;
    let mut xoffset = yoffset;
    let mut yscale = 1.0f64;
    let mut xscale = yscale;
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        let mut k = 0;
        while !_DTKEYS[k].is_null() && strcmp(kp, _DTKEYS[k]) != 0 {
            k += 1
        }
        if _DTKEYS[k as usize].is_null() {
            spc_warn(
                spe,
                b"Unrecognized dimension/transformation key: %s\x00" as *const u8 as *const i8,
                kp,
            );
            error = -1i32;
            free(kp as *mut libc::c_void);
            break;
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
            if k == 7 {
                t.flags |= 1i32 << 3i32;
                free(kp as *mut libc::c_void);
            /* not key-value */
            } else {
                if (*ap).curptr < (*ap).endptr && *(*ap).curptr.offset(0) as i32 == '=' as i32 {
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                }
                let mut vp;
                if *(*ap).curptr.offset(0) as i32 == '\'' as i32
                    || *(*ap).curptr.offset(0) as i32 == '\"' as i32
                {
                    let mut qchr: i8 = *(*ap).curptr.offset(0);
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                    vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                    if !vp.is_null() && qchr as i32 != *(*ap).curptr.offset(0) as i32 {
                        spc_warn(
                            spe,
                            b"Syntax error in dimension/transformation specification.\x00"
                                as *const u8 as *const i8,
                        );
                        error = -1i32;
                        vp = mfree(vp as *mut libc::c_void) as *mut i8
                    }
                    (*ap).curptr = (*ap).curptr.offset(1)
                } else {
                    vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr)
                }
                if error == 0 && vp.is_null() {
                    spc_warn(
                        spe,
                        b"Missing value for dimension/transformation: %s\x00" as *const u8
                            as *const i8,
                        kp,
                    );
                    error = -1i32
                }
                free(kp as *mut libc::c_void);
                if vp.is_null() || error != 0 {
                    break;
                }
                match k {
                    0 => xoffset = atof(vp),
                    1 => yoffset = atof(vp),
                    2 => {
                        t.width = atof(vp);
                        t.flags |= 1i32 << 1i32
                    }
                    3 => {
                        t.height = atof(vp);
                        t.flags |= 1i32 << 2i32
                    }
                    4 => xscale = atof(vp) / 100.0f64,
                    5 => yscale = atof(vp) / 100.0f64,
                    6 => rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64,
                    8 => {
                        t.bbox.llx = atof(vp);
                        t.flags |= 1i32 << 0i32
                    }
                    9 => {
                        t.bbox.lly = atof(vp);
                        t.flags |= 1i32 << 0i32
                    }
                    10 => {
                        t.bbox.urx = atof(vp);
                        t.flags |= 1i32 << 0i32
                    }
                    11 => {
                        t.bbox.ury = atof(vp);
                        t.flags |= 1i32 << 0i32
                    }
                    12 => {
                        t.width = atof(vp) / 10.0f64;
                        t.flags |= 1i32 << 1i32
                    }
                    13 => {
                        t.height = atof(vp) / 10.0f64;
                        t.flags |= 1i32 << 2i32
                    }
                    _ => {}
                }
                skip_blank(&mut (*ap).curptr, (*ap).endptr);
                free(vp as *mut libc::c_void);
            }
        }
    }
    make_transmatrix(&mut t.matrix, xoffset, yoffset, xscale, yscale, rotate);
    error
}
/* "page" and "pagebox" are not dimension nor transformation nor
 * something acceptable to put into here.
 * PLEASE DONT ADD HERE!
 */
unsafe extern "C" fn spc_read_dimtrns_pdfm(
    mut spe: *mut spc_env,
    p: &mut transform_info,
    mut ap: *mut spc_arg,
) -> i32 {
    const _DTKEYS: [*const i8; 12] = [
        b"width\x00" as *const u8 as *const i8,
        b"height\x00" as *const u8 as *const i8,
        b"depth\x00" as *const u8 as *const i8,
        b"scale\x00" as *const u8 as *const i8,
        b"xscale\x00" as *const u8 as *const i8,
        b"yscale\x00" as *const u8 as *const i8,
        b"rotate\x00" as *const u8 as *const i8,
        b"bbox\x00" as *const u8 as *const i8,
        b"matrix\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"hide\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut error: i32 = 0i32;
    let mut has_matrix = 0i32;
    let mut has_rotate = has_matrix;
    let mut has_scale = has_rotate; /* default: do clipping */
    let mut has_yscale = has_scale;
    let mut has_xscale = has_yscale;
    let mut yscale = 1.0f64;
    let mut xscale = yscale;
    let mut rotate = 0.0f64;
    p.flags |= 1i32 << 3i32;
    p.flags &= !(1i32 << 4i32);
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        let mut k = 0;
        while !_DTKEYS[k].is_null() && strcmp(_DTKEYS[k], kp) != 0 {
            k += 1
        }
        match k {
            0 => {
                error = spc_util_read_length(spe, &mut p.width, ap);
                p.flags |= 1i32 << 1i32
            }
            1 => {
                error = spc_util_read_length(spe, &mut p.height, ap);
                p.flags |= 1i32 << 2i32
            }
            2 => {
                error = spc_util_read_length(spe, &mut p.depth, ap);
                p.flags |= 1i32 << 2i32
            }
            3 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            4 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    xscale = atof(vp);
                    has_xscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            5 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    has_yscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            6 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64;
                    has_rotate = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            7 => {
                let mut v: [f64; 4] = [0.; 4];
                if spc_util_read_numbers(v.as_mut_ptr(), 4i32, ap) != 4i32 {
                    error = -1i32
                } else {
                    p.bbox.llx = v[0];
                    p.bbox.lly = v[1];
                    p.bbox.urx = v[2];
                    p.bbox.ury = v[3];
                    p.flags |= 1i32 << 0i32
                }
            }
            8 => {
                let mut v_0: [f64; 6] = [0.; 6];
                if spc_util_read_numbers(v_0.as_mut_ptr(), 6i32, ap) != 6i32 {
                    error = -1i32
                } else {
                    p.matrix.a = v_0[0];
                    p.matrix.b = v_0[1];
                    p.matrix.c = v_0[2];
                    p.matrix.d = v_0[3];
                    p.matrix.e = v_0[4];
                    p.matrix.f = v_0[5];
                    has_matrix = 1i32
                }
            }
            9 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    if atof(vp) != 0. {
                        p.flags |= 1i32 << 3i32
                    } else {
                        p.flags &= !(1i32 << 3i32)
                    }
                    free(vp as *mut libc::c_void);
                }
            }
            10 => p.flags |= 1i32 << 4i32,
            _ => error = -1i32,
        }
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized key or invalid value for dimension/transformation: %s\x00"
                    as *const u8 as *const i8,
                kp,
            );
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut libc::c_void);
    }
    if error == 0 {
        /* Check consistency */
        if has_xscale != 0 && p.flags & 1i32 << 1i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both width and xscale. Ignore xscale.\x00" as *const u8
                    as *const i8,
            );
            xscale = 1.0f64
        } else if has_yscale != 0 && p.flags & 1i32 << 2i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both height/depth and yscale. Ignore yscale.\x00" as *const u8
                    as *const i8,
            );
            yscale = 1.0f64
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            spc_warn(
                spe,
                b"Can\'t supply overall scale along with axis scales.\x00" as *const u8
                    as *const i8,
            );
            error = -1i32
        } else if has_matrix != 0
            && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0)
        {
            spc_warn(spe,
                     b"Can\'t supply transform matrix along with scales or rotate. Ignore scales and rotate.\x00"
                         as *const u8 as *const i8);
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&mut p.matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if p.flags & 1i32 << 0i32 == 0 {
        p.flags &= !(1i32 << 3i32)
        /* no clipping needed */
    }
    error
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_dimtrns(
    mut spe: *mut spc_env,
    ti: &mut transform_info,
    mut args: *mut spc_arg,
    mut syntax: i32,
) -> i32 {
    if spe.is_null() || args.is_null() {
        return -1i32;
    }
    if syntax != 0 {
        return spc_read_dimtrns_dvips(spe, ti, args);
    } else {
        return spc_read_dimtrns_pdfm(spe, ti, args);
    };
}
/* syntax 1: ((rgb|cmyk|hsb|gray) colorvalues)|colorname
 * syntax 0: pdf_number|pdf_array
 *
 * This is for reading *single* color specification.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_blahblah(
    mut spe: *mut spc_env,
    p: &mut transform_info,
    mut page_no: *mut i32,
    mut bbox_type: *mut i32,
    mut ap: *mut spc_arg,
) -> i32 {
    const _DTKEYS: [*const i8; 14] = [
        b"width\x00" as *const u8 as *const i8,
        b"height\x00" as *const u8 as *const i8,
        b"depth\x00" as *const u8 as *const i8,
        b"scale\x00" as *const u8 as *const i8,
        b"xscale\x00" as *const u8 as *const i8,
        b"yscale\x00" as *const u8 as *const i8,
        b"rotate\x00" as *const u8 as *const i8,
        b"bbox\x00" as *const u8 as *const i8,
        b"matrix\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"hide\x00" as *const u8 as *const i8,
        b"page\x00" as *const u8 as *const i8,
        b"pagebox\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut error: i32 = 0i32;
    let mut has_matrix = 0i32; /* default: do clipping */
    let mut has_rotate = has_matrix;
    let mut has_scale = has_rotate;
    let mut has_yscale = has_scale;
    let mut has_xscale = has_yscale;
    let mut yscale = 1.0f64;
    let mut xscale = yscale;
    let mut rotate = 0.0f64;
    p.flags |= 1i32 << 3i32;
    p.flags &= !(1i32 << 4i32);
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        let mut k = 0;
        while !_DTKEYS[k].is_null() && strcmp(_DTKEYS[k], kp) != 0 {
            k += 1
        }
        match k {
            0 => {
                error = spc_util_read_length(spe, &mut p.width, ap);
                p.flags |= 1i32 << 1i32
            }
            1 => {
                error = spc_util_read_length(spe, &mut p.height, ap);
                p.flags |= 1i32 << 2i32
            }
            2 => {
                error = spc_util_read_length(spe, &mut p.depth, ap);
                p.flags |= 1i32 << 2i32
            }
            3 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            4 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    xscale = atof(vp);
                    has_xscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            5 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    has_yscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            6 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64;
                    has_rotate = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            7 => {
                let mut v: [f64; 4] = [0.; 4];
                if spc_util_read_numbers(v.as_mut_ptr(), 4i32, ap) != 4i32 {
                    error = -1i32
                } else {
                    p.bbox.llx = v[0];
                    p.bbox.lly = v[1];
                    p.bbox.urx = v[2];
                    p.bbox.ury = v[3];
                    p.flags |= 1i32 << 0i32
                }
            }
            8 => {
                let mut v_0: [f64; 6] = [0.; 6];
                if spc_util_read_numbers(v_0.as_mut_ptr(), 6i32, ap) != 6i32 {
                    error = -1i32
                } else {
                    p.matrix.a = v_0[0];
                    p.matrix.b = v_0[1];
                    p.matrix.c = v_0[2];
                    p.matrix.d = v_0[3];
                    p.matrix.e = v_0[4];
                    p.matrix.f = v_0[5];
                    has_matrix = 1i32
                }
            }
            9 => {
                let vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    if atof(vp) != 0. {
                        p.flags |= 1i32 << 3i32
                    } else {
                        p.flags &= !(1i32 << 3i32)
                    }
                    free(vp as *mut libc::c_void);
                }
            }
            11 => {
                let mut page: f64 = 0.;
                if !page_no.is_null() && spc_util_read_numbers(&mut page, 1i32, ap) == 1i32 {
                    *page_no = page as i32
                } else {
                    error = -1i32
                }
            }
            10 => p.flags |= 1i32 << 4i32,
            12 => {
                let q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
                if !q.is_null() {
                    if !bbox_type.is_null() {
                        if strcasecmp(q, b"cropbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 1i32
                        } else if strcasecmp(q, b"mediabox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 2i32
                        } else if strcasecmp(q, b"artbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 3i32
                        } else if strcasecmp(q, b"trimbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 4i32
                        } else if strcasecmp(q, b"bleedbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 5i32
                        }
                    }
                    free(q as *mut libc::c_void);
                } else if !bbox_type.is_null() {
                    *bbox_type = 0i32
                }
            }
            _ => error = -1i32,
        }
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized key or invalid value for dimension/transformation: %s\x00"
                    as *const u8 as *const i8,
                kp,
            );
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut libc::c_void);
    }
    if error == 0 {
        /* Check consistency */
        if has_xscale != 0 && p.flags & 1i32 << 1i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both width and xscale. Ignore xscale.\x00" as *const u8
                    as *const i8,
            );
            xscale = 1.0f64
        } else if has_yscale != 0 && p.flags & 1i32 << 2i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both height/depth and yscale. Ignore yscale.\x00" as *const u8
                    as *const i8,
            );
            yscale = 1.0f64
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            spc_warn(
                spe,
                b"Can\'t supply overall scale along with axis scales.\x00" as *const u8
                    as *const i8,
            );
            error = -1i32
        } else if has_matrix != 0
            && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0)
        {
            spc_warn(spe,
                     b"Can\'t supply transform matrix along with scales or rotate. Ignore scales and rotate.\x00"
                         as *const u8 as *const i8);
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&mut p.matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if p.flags & 1i32 << 0i32 == 0 {
        p.flags &= !(1i32 << 3i32)
        /* no clipping needed */
    }
    error
}
static mut COLORDEFS: [colordef_; 69] = [
    {
        let mut init = colordef_ {
            key: b"GreenYellow\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.15f64, 0.00f64, 0.69f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Yellow\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Goldenrod\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.10f64, 0.84f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Dandelion\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.29f64, 0.84f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Apricot\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.32f64, 0.52f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Peach\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.50f64, 0.70f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Melon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.46f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"YellowOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.42f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Orange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.61f64, 0.87f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BurntOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.51f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Bittersweet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.75f64, 1.00f64, 0.24f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RedOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.77f64, 0.87f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Mahogany\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.85f64, 0.87f64, 0.35f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Maroon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.87f64, 0.68f64, 0.32f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BrickRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.89f64, 0.94f64, 0.28f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Red\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 1.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"OrangeRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 1.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RubineRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 1.00f64, 0.13f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"WildStrawberry\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.96f64, 0.39f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Salmon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.53f64, 0.38f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CarnationPink\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.63f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Magenta\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"VioletRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.81f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Rhodamine\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.82f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Mulberry\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.34f64, 0.90f64, 0.00f64, 0.02f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RedViolet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.07f64, 0.90f64, 0.00f64, 0.34f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Fuchsia\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.47f64, 0.91f64, 0.00f64, 0.08f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Lavender\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.48f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Thistle\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.12f64, 0.59f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Orchid\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.32f64, 0.64f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"DarkOrchid\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.40f64, 0.80f64, 0.20f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Purple\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.45f64, 0.86f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Plum\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.50f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Violet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.79f64, 0.88f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RoyalPurple\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.75f64, 0.90f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BlueViolet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.86f64, 0.91f64, 0.00f64, 0.04f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Periwinkle\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.57f64, 0.55f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CadetBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.62f64, 0.57f64, 0.23f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CornflowerBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.65f64, 0.13f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"MidnightBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.98f64, 0.13f64, 0.00f64, 0.43f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"NavyBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.94f64, 0.54f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RoyalBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [1.00f64, 0.50f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Blue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [1.00f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Cerulean\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.94f64, 0.11f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Cyan\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [1.00f64, 0.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"ProcessBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.96f64, 0.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SkyBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.62f64, 0.00f64, 0.12f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Turquoise\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.85f64, 0.00f64, 0.20f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"TealBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.86f64, 0.00f64, 0.34f64, 0.02f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Aquamarine\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.82f64, 0.00f64, 0.30f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BlueGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.85f64, 0.00f64, 0.33f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Emerald\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [1.00f64, 0.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"JungleGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.99f64, 0.00f64, 0.52f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SeaGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.69f64, 0.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Green\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [1.00f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"ForestGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.91f64, 0.00f64, 0.88f64, 0.12f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"PineGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.92f64, 0.00f64, 0.59f64, 0.25f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"LimeGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.50f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"YellowGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.44f64, 0.00f64, 0.74f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SpringGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.26f64, 0.00f64, 0.76f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"OliveGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.64f64, 0.00f64, 0.95f64, 0.40f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RawSienna\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.72f64, 1.00f64, 0.45f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Sepia\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.83f64, 1.00f64, 0.70f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Brown\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.00f64, 0.81f64, 1.00f64, 0.60f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Tan\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: None,
                    values: [0.14f64, 0.42f64, 0.56f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Gray\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: None,
                    values: [0.5f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Black\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: None,
                    values: [0.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"White\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: None,
                    values: [1.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: 0 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 0i32,
                    spot_color_name: None,
                    values: [0.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
];
/* From pdfcolor.c */
unsafe extern "C" fn pdf_color_namedcolor(color: &mut pdf_color, mut name: *const i8) -> i32 {
    let mut i: i32 = 0;
    i = 0i32;
    while !COLORDEFS[i as usize].key.is_null() {
        if streq_ptr(COLORDEFS[i as usize].key, name) {
            *color = (*COLORDEFS.as_mut_ptr().offset(i as isize)).color.clone();
            return 0i32;
        }
        i += 1
    }
    -1i32
}