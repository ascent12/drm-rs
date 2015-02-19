// Copyright (c) 2015 Scott Anderson <ascent12@hotmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![feature(core)]
#![feature(libc)]
#![feature(std_misc)]

use std::os::unix::prelude::*;
use std::ptr;
use std::mem;

mod ffi;

pub struct Resources {
    fd: Fd,
    ptr: *const ffi::drmModeRes,
}

impl Resources {
    pub fn from_fd(fd: Fd) -> Option<Resources> {
        unsafe {
            let res = ffi::drmModeGetResources(fd);

            if res.is_null() {
                return None;
            }

            return Some(Resources { fd: fd, ptr: res });
        }
    }

    pub fn crtcs(&self) -> Option<Vec<Crtc>> {
        unsafe {
            let mut vec = Vec::with_capacity((*self.ptr).count_crtcs as usize);

            for i in 0..(*self.ptr).count_crtcs {
                match Crtc::from_id(self.fd, *(*self.ptr).crtcs.offset(i as isize)) {
                    Some(crtc) => vec.push(crtc),
                    None => return None,
                }
            }

            return Some(vec);
        }
    }

    pub fn connectors(&self) -> Option<Vec<Connector>> {
        unsafe {
            let mut vec = Vec::with_capacity((*self.ptr).count_connectors as usize);

            for i in 0..(*self.ptr).count_connectors {
                match Connector::from_id(self.fd, *(*self.ptr).connectors.offset(i as isize)) {
                    Some(conn) => vec.push(conn),
                    None => return None,
                }
            }

            return Some(vec);
        }
    }

    pub fn encoders(&self) -> Option<Vec<Encoder>> {
        unsafe {
            let mut vec = Vec::with_capacity((*self.ptr).count_encoders as usize);

            for i in 0..(*self.ptr).count_encoders {
                match Encoder::from_id(self.fd, *(*self.ptr).encoders.offset(i as isize)) {
                    Some(enc) => vec.push(enc),
                    None => return None,
                }
            }

            return Some(vec);
        }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        unsafe { ffi::drmModeFreeResources(self.ptr) }
    }
}

pub struct Connector {
    fd: Fd,
    ptr: *const ffi::drmModeConnector,
}

impl Connector {
    pub fn from_id(fd: Fd, id: u32) -> Option<Connector> {
        unsafe {
            let conn = ffi::drmModeGetConnector(fd, id);

            if conn.is_null() {
                return None;
            }

            return Some(Connector { fd: fd, ptr: conn });
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { (*self.ptr).connector_id }
    }

    pub fn is_connected(&self) -> bool {
        unsafe {
            match (*self.ptr).connection {
                ffi::drmModeConnection::DRM_MODE_CONNECTED => true,
                _ => false,
            }
        }
    }

    pub fn modes(&self) -> Vec<ModeInfo> {
        unsafe {
            let mut vec = Vec::with_capacity((*self.ptr).count_modes as usize);

            for i in 0..(*self.ptr).count_modes {
                let mut mode = ModeInfo { mode: mem::uninitialized() };

                ptr::copy_nonoverlapping_memory(&mut mode.mode,
                                                (*self.ptr).modes.offset(i as isize),
                                                1);

                vec.push(mode)
            }

            return vec;
        }
    }

    pub fn encoder(&self) -> Option<Encoder> {
        unsafe { Encoder::from_id(self.fd, (*self.ptr).encoder_id) }
    }

    pub fn valid_encoders(&self) -> Option<Vec<Encoder>> {
        unsafe {
            let mut vec = Vec::with_capacity((*self.ptr).count_encoders as usize);

            for i in 0..(*self.ptr).count_encoders {
                match Encoder::from_id(self.fd, *(*self.ptr).encoders.offset(i as isize)) {
                    Some(enc) => vec.push(enc),
                    None => return None,
                }
            }

            return Some(vec);
        }
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        unsafe { ffi::drmModeFreeConnector(self.ptr) }
    }
}

pub struct Crtc {
    ptr: *const ffi::drmModeCrtc
}

impl Crtc {
    pub fn from_id(fd: Fd, id: u32) -> Option<Crtc> {
        unsafe {
            let crtc = ffi::drmModeGetCrtc(fd, id);

            if crtc.is_null() {
                return None;
            }

            return Some(Crtc { ptr: crtc });
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { (*self.ptr).crtc_id }
    }
}

impl PartialEq for Crtc {
    fn eq(&self, other: &Crtc) -> bool {
        self.id() == other.id()
    }
}

impl Drop for Crtc {
    fn drop(&mut self) {
        unsafe { ffi::drmModeFreeCrtc(self.ptr) }
    }
}

pub struct Encoder {
    ptr: *const ffi::drmModeEncoder
}

impl Encoder {
    pub fn from_id(fd: Fd, id: u32) -> Option<Encoder> {
        unsafe {
            let enc = ffi::drmModeGetEncoder(fd, id);

            if enc.is_null() {
                return None;
            }

            return Some(Encoder { ptr: enc });
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { (*self.ptr).encoder_id }
    }

    pub fn crtc_valid(&self, crtc_list: Vec<Crtc>, test: Crtc) -> bool {
        unsafe {
            crtc_list.iter()
                     .enumerate()
                     .filter(|&(i, c)| *c == test && (*self.ptr).possible_crtcs & (1 << i) != 0 )
                     .count() != 0
        }
    }
            
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { ffi::drmModeFreeEncoder(self.ptr) }
    }
}

pub struct ModeInfo {
    mode: ffi::drmModeModeInfo
}

pub fn add_fb(fd: Fd, width: u32, height: u32,
              depth: u8, bpp: u8,
              pitch: u32, bo_handle: u32) -> Option<u32> {
    unsafe {
        let mut buf_id: u32 = 0;

        if ffi::drmModeAddFB(fd, width, height, depth, bpp, pitch, bo_handle, &mut buf_id) != 0 {
            return None;
        }

        return Some(buf_id);
    }
}

pub fn set_crtc(fd: Fd, crtc: Crtc, buffer_id: u32, x: u32, y: u32,
                connectors: Vec<Connector>, mode: Option<ModeInfo>) {
    let mut conn_ids = Vec::with_capacity(connectors.len());

    for conn in connectors.iter() {
        conn_ids.push(conn.id());
    }

    unsafe {
        ffi::drmModeSetCrtc(fd, crtc.id(), buffer_id, x, y,
                            conn_ids.as_ptr(), conn_ids.len() as i32,
                            match mode { Some(m) => &m.mode, None => ptr::null() });
    }
}
