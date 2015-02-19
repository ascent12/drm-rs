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

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{
    c_char,
    c_ushort,
    c_int,
    c_void,
    uint8_t,
    int32_t,
    uint16_t,
    uint32_t,
    uint64_t
};

pub const DRM_DISPLAY_INFO_LEN: uint32_t = 32;
pub const DRM_CONNECTOR_NAME_LEN: uint32_t = 32;
pub const DRM_DISPLAY_MODE_LEN: uint32_t = 32;
pub const DRM_PROP_NAME_LEN: uint32_t = 32;

pub const DRM_MODE_TYPE_BUILTIN: uint32_t = (1<<0);
pub const DRM_MODE_TYPE_CLOCK_C: uint32_t = ((1<<1) | DRM_MODE_TYPE_BUILTIN);
pub const DRM_MODE_TYPE_CRTC_C: uint32_t = ((1<<2) | DRM_MODE_TYPE_BUILTIN);
pub const DRM_MODE_TYPE_PREFERRED: uint32_t = (1<<3);
pub const DRM_MODE_TYPE_DEFAULT: uint32_t = (1<<4);
pub const DRM_MODE_TYPE_USERDEF: uint32_t = (1<<5);
pub const DRM_MODE_TYPE_DRIVER: uint32_t = (1<<6);

// Video mode flags 
// bit compatible with the xorg definitions. 
pub const DRM_MODE_FLAG_PHSYNC: uint32_t = (1<<0);
pub const DRM_MODE_FLAG_NHSYNC: uint32_t = (1<<1);
pub const DRM_MODE_FLAG_PVSYNC: uint32_t = (1<<2);
pub const DRM_MODE_FLAG_NVSYNC: uint32_t = (1<<3);
pub const DRM_MODE_FLAG_INTERLACE: uint32_t = (1<<4);
pub const DRM_MODE_FLAG_DBLSCAN: uint32_t = (1<<5);
pub const DRM_MODE_FLAG_CSYNC: uint32_t = (1<<6);
pub const DRM_MODE_FLAG_PCSYNC: uint32_t = (1<<7);
pub const DRM_MODE_FLAG_NCSYNC: uint32_t = (1<<8);
pub const DRM_MODE_FLAG_HSKEW: uint32_t = (1<<9); // hskew provided 
pub const DRM_MODE_FLAG_BCAST: uint32_t = (1<<10);
pub const DRM_MODE_FLAG_PIXMUX: uint32_t = (1<<11);
pub const DRM_MODE_FLAG_DBLCLK: uint32_t = (1<<12);
pub const DRM_MODE_FLAG_CLKDIV2: uint32_t = (1<<13);
pub const DRM_MODE_FLAG_3D_MASK: uint32_t = (0x1f<<14);
pub const DRM_MODE_FLAG_3D_NONE: uint32_t = (0<<14);
pub const DRM_MODE_FLAG_3D_FRAME_PACKING: uint32_t = (1<<14);
pub const DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE: uint32_t = (2<<14);
pub const DRM_MODE_FLAG_3D_LINE_ALTERNATIVE: uint32_t = (3<<14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL: uint32_t = (4<<14);
pub const DRM_MODE_FLAG_3D_L_DEPTH: uint32_t = (5<<14);
pub const DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH: uint32_t = (6<<14);
pub const DRM_MODE_FLAG_3D_TOP_AND_BOTTOM: uint32_t = (7<<14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF: uint32_t = (8<<14);


// DPMS flags 
// bit compatible with the xorg definitions. 
pub const DRM_MODE_DPMS_ON: uint32_t = 0;
pub const DRM_MODE_DPMS_STANDBY: uint32_t = 1;
pub const DRM_MODE_DPMS_SUSPEND: uint32_t = 2;
pub const DRM_MODE_DPMS_OFF: uint32_t = 3;

// Scaling mode options 
pub const DRM_MODE_SCALE_NONE: uint32_t = 0; // Unmodified timing (display or software can still scale) 
pub const DRM_MODE_SCALE_FULLSCREEN: uint32_t = 1; // Full screen, ignore aspect 
pub const DRM_MODE_SCALE_CENTER: uint32_t = 2; // Centered, no scaling 
pub const DRM_MODE_SCALE_ASPECT: uint32_t = 3; // Full screen, preserve aspect 

// Dithering mode options 
pub const DRM_MODE_DITHERING_OFF: uint32_t = 0;
pub const DRM_MODE_DITHERING_ON: uint32_t = 1;
pub const DRM_MODE_DITHERING_AUTO: uint32_t = 2;

// Dirty info options 
pub const DRM_MODE_DIRTY_OFF: uint32_t = 0;
pub const DRM_MODE_DIRTY_ON: uint32_t = 1;
pub const DRM_MODE_DIRTY_ANNOTATE: uint32_t = 2;

pub const DRM_MODE_ENCODER_NONE: uint32_t = 0;
pub const DRM_MODE_ENCODER_DAC: uint32_t = 1;
pub const DRM_MODE_ENCODER_TMDS: uint32_t = 2;
pub const DRM_MODE_ENCODER_LVDS: uint32_t = 3;
pub const DRM_MODE_ENCODER_TVDAC: uint32_t = 4;
pub const DRM_MODE_ENCODER_VIRTUAL: uint32_t = 5;
pub const DRM_MODE_ENCODER_DSI: uint32_t = 6;
pub const DRM_MODE_ENCODER_DPMST: uint32_t = 7;

// This is for connectors with multiple signal types. 
// Try to match DRM_MODE_CONNECTOR_X as closely as possible. 
pub const DRM_MODE_SUBCONNECTOR_Automatic: uint32_t = 0;
pub const DRM_MODE_SUBCONNECTOR_Unknown: uint32_t = 0;
pub const DRM_MODE_SUBCONNECTOR_DVID: uint32_t = 3;
pub const DRM_MODE_SUBCONNECTOR_DVIA: uint32_t = 4;
pub const DRM_MODE_SUBCONNECTOR_Composite: uint32_t = 5;
pub const DRM_MODE_SUBCONNECTOR_SVIDEO: uint32_t = 6;
pub const DRM_MODE_SUBCONNECTOR_Component: uint32_t = 8;
pub const DRM_MODE_SUBCONNECTOR_SCART: uint32_t = 9;

pub const DRM_MODE_CONNECTOR_Unknown: uint32_t = 0;
pub const DRM_MODE_CONNECTOR_VGA: uint32_t = 1;
pub const DRM_MODE_CONNECTOR_DVII: uint32_t = 2;
pub const DRM_MODE_CONNECTOR_DVID: uint32_t = 3;
pub const DRM_MODE_CONNECTOR_DVIA: uint32_t = 4;
pub const DRM_MODE_CONNECTOR_Composite: uint32_t = 5;
pub const DRM_MODE_CONNECTOR_SVIDEO: uint32_t = 6;
pub const DRM_MODE_CONNECTOR_LVDS: uint32_t = 7;
pub const DRM_MODE_CONNECTOR_Component: uint32_t = 8;
pub const DRM_MODE_CONNECTOR_9PinDIN: uint32_t = 9;
pub const DRM_MODE_CONNECTOR_DisplayPort: uint32_t = 10;
pub const DRM_MODE_CONNECTOR_HDMIA: uint32_t = 11;
pub const DRM_MODE_CONNECTOR_HDMIB: uint32_t = 12;
pub const DRM_MODE_CONNECTOR_TV: uint32_t = 13;
pub const DRM_MODE_CONNECTOR_eDP: uint32_t = 14;
pub const DRM_MODE_CONNECTOR_VIRTUAL: uint32_t = 15;
pub const DRM_MODE_CONNECTOR_DSI: uint32_t = 16;

pub const DRM_MODE_PROP_PENDING: uint32_t = (1<<0);
pub const DRM_MODE_PROP_RANGE: uint32_t = (1<<1);
pub const DRM_MODE_PROP_IMMUTABLE: uint32_t = (1<<2);
pub const DRM_MODE_PROP_ENUM: uint32_t = (1<<3); // enumerated type with text strings 
pub const DRM_MODE_PROP_BLOB: uint32_t = (1<<4);
pub const DRM_MODE_PROP_BITMASK: uint32_t = (1<<5); // bitmask of enumerated types 

#[repr(C)]
pub struct drmModeRes {
    pub count_fbs: c_int,
    pub fbs: *const uint32_t,

    pub count_crtcs: c_int,
    pub crtcs: *const uint32_t,

    pub count_connectors: c_int,
    pub connectors: *const uint32_t,

    pub count_encoders: c_int,
    pub encoders: *const uint32_t,

    pub min_width: uint32_t,
    pub max_width: uint32_t,
    pub min_height: uint32_t,
    pub max_height: uint32_t
}

#[repr(C)]
pub struct drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vskew: uint16_t,

    pub vrefresh: uint32_t,

    pub flags: uint32_t,
    pub _type: uint32_t,    // 'type' is a keyword in Rust
    pub name: [c_char; DRM_DISPLAY_MODE_LEN as usize]
}

#[repr(C)]
pub struct drmModeFB {
    pub fb_id: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub pitch: uint32_t,
    pub bpp: uint32_t,
    pub depth: uint32_t,
    pub handle: uint32_t
}

// Defined in /usr/include/libdrm/drm.h as struct drm_clip_rect
#[repr(C)]
pub struct drmModeClip {
    pub x1: c_ushort,
    pub y1: c_ushort,
    pub x2: c_ushort,
    pub y2: c_ushort
}

#[repr(C)]
pub struct drmModePropertyBlob {
    pub id: uint32_t,
    pub length: uint32_t,
    pub data: *const c_void
}

// Defined in /usr/include/libdrm/drm_mode.h
#[repr(C)]
pub struct drm_mode_property_enum {
    pub value: uint64_t,
    pub name: [c_char; DRM_PROP_NAME_LEN as usize]
}

#[repr(C)]
pub struct drmModeProperty {
    pub prop_id: uint32_t,
    pub flags: uint32_t,
    pub name: [c_char; DRM_PROP_NAME_LEN as usize],
    pub count_values: c_int,
    pub values: *const uint64_t,
    pub count_enums: c_int,
    pub enums: *const drm_mode_property_enum,
    pub count_blobs: c_int,
    pub blob_ids: *const uint32_t
}

#[repr(C)]
pub struct drmModeCrtc {
    pub crtc_id: uint32_t,
    pub buffer_id: uint32_t,

    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub mode_valid: c_int,
    pub mode: drmModeModeInfo,

    pub gamma_size: c_int
}

#[repr(C)]
pub struct drmModeEncoder {
    pub encoder_id: uint32_t,
    pub encoder_type: uint32_t,
    pub crtc_id: uint32_t,
    pub possible_crtcs: uint32_t,
    pub possible_clones: uint32_t
}

#[repr(C)]
pub enum drmModeConnection {
    DRM_MODE_CONNECTED = 1,
    DRM_MODE_DISCONNECTED = 2,
    DRM_MODE_UNKNOWN = 3
}

#[repr(C)]
pub enum drmModeSubPixel {
    DRM_MODE_SUBPIXEL_UNKNOWN = 1,
    DRM_MODE_SUBPIXEL_HORIZONTAL_RGB = 2,
    DRM_MODE_SUBPIXEL_HORIZONTAL_BGR = 3,
    DRM_MODE_SUBPIXEL_VERTICAL_RGB = 4,
    DRM_MODE_SUBPIXEL_VERTICAL_BGR = 5,
    DRM_MODE_SUBPIXEL_NONE = 6
}

#[repr(C)]
pub struct drmModeConnector {
    pub connector_id: uint32_t,
    pub encoder_id: uint32_t,
    pub connector_type: uint32_t,
    pub connector_type_id: uint32_t,
    pub connection: drmModeConnection,
    pub mmWidth: uint32_t,
    pub mmHeight: uint32_t,
    pub subpixel: drmModeSubPixel,

    pub count_modes: c_int,
    pub modes: *const drmModeModeInfo,

    pub count_props: c_int,
    pub props: *const uint32_t,
    pub prop_values: *const uint64_t,

    pub count_encoders: c_int,
    pub encoders: *const uint32_t
}

#[repr(C)]
pub struct drmModeObjectProperties {
    pub count_props: uint32_t,
    pub props: *const uint32_t,
    pub prop_values: *const uint64_t
}

#[repr(C)]
pub struct drmModePlane {
    pub count_formats: uint32_t,
    pub formats: *const uint32_t,
    pub plane_id: uint32_t,

    pub crtc_id: uint32_t,
    pub fb_id: uint32_t,

    pub crtc_x: uint32_t,
    pub crtc_y: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,

    pub possible_crtcs: uint32_t,
    pub gamma_size: uint32_t
}

#[repr(C)]
pub struct drmModePlaneRes {
    pub count_planes: uint32_t,
    pub planes: *const uint32_t
}

#[link(name = "drm")]
extern {
    pub fn drmModeFreeModeInfo(ptr: *const drmModeModeInfo);
    pub fn drmModeFreeResources(ptr: *const drmModeRes);
    pub fn drmModeFreeFB(ptr: *const drmModeFB);
    pub fn drmModeFreeCrtc(ptr: *const drmModeCrtc);
    pub fn drmModeFreeConnector(ptr: *const drmModeConnector);
    pub fn drmModeFreeEncoder(ptr: *const drmModeEncoder);
    pub fn drmModeFreePlane(ptr: *const drmModePlane);
    pub fn drmModeFreePlaneResources(ptr: *const drmModePlaneRes);

    pub fn drmModeGetResources(fd: c_int) -> *const drmModeRes;

    pub fn drmModeGetFB(fd: c_int, buffer_id: uint32_t) -> *const drmModeFB;

    pub fn drmModeAddFB(fd: c_int, width: uint32_t, height: uint32_t, depth: uint8_t,
                        bpp: uint8_t, pitch: uint32_t, bo_handle: uint32_t,
                        buf_id: *mut uint32_t) -> c_int;
    pub fn drmModeAddFB2(fd: c_int, width: uint32_t, height: uint32_t,
                         pixel_format: uint32_t, bo_handles: [uint32_t; 4],
                         pitches: [uint32_t; 4], offsets: [uint32_t; 4],
                         buf_id: *const uint32_t, flags: uint32_t);

    pub fn drmModeRmFB(fd: c_int, bufferId: uint32_t) -> c_int;
    pub fn drmModeDirtyFB(fd: c_int, bufferId: uint32_t,
                          clips: *const drmModeClip, num_clips: uint32_t) -> c_int;

    pub fn drmModeGetCrtc(fd: c_int, crtcId: uint32_t) -> *const drmModeCrtc;

    pub fn drmModeSetCrtc(fd: c_int, crtcId: uint32_t, bufferId: uint32_t,
                          x: uint32_t, y: uint32_t, connectors: *const uint32_t, count: c_int,
                          mode: *const drmModeModeInfo) -> c_int;

    pub fn drmModeSetCursor(fd: c_int, crtcId: uint32_t, bo_handle: uint32_t,
                            width: uint32_t, height: uint32_t) -> c_int;
    pub fn drmModeSetCursor2(fd: c_int, crtcId: uint32_t, bo_handle: uint32_t,
                             width: uint32_t, height: uint32_t, hot_x: int32_t, hot_y: int32_t) -> c_int;

    pub fn drmModeMoveCursor(fd: c_int, crtcId: uint32_t, x: c_int, y: c_int) -> c_int;

    pub fn drmModeGetEncoder(fd: c_int, encoder_id: uint32_t) -> *const drmModeEncoder;

    pub fn drmModeGetConnector(fd: c_int, connector_id: uint32_t) -> *const drmModeConnector;

    pub fn drmModeAttachMode(fd: c_int, connectorId: uint32_t, mode_info: *const drmModeModeInfo) -> c_int;
    pub fn drmModeDetachMode(fd: c_int, connectorId: uint32_t, mode_info: *const drmModeModeInfo) -> c_int;

    pub fn drmModeGetProperty(fd: c_int, propertyId: uint32_t) -> *const drmModeProperty;
    pub fn drmModeFreeProperty(ptr: *const drmModeProperty);

    pub fn drmModeGetPropertyBlob(fd: c_int, blob_id: uint32_t) -> *const drmModePropertyBlob;
    pub fn drmModeFreePropertyBlob(ptr: *const drmModePropertyBlob);
    pub fn drmModeConnectorSetProperty(fd: c_int, connector_id: uint32_t, property_id: uint32_t,
                                       value: uint64_t) -> c_int;
    pub fn drmCheckModesettingSupported(busid: *const c_char) -> c_int;

    pub fn drmModeCrtcSetGamma(fd: c_int, crtc_id: uint32_t, size: uint32_t,
                               red: *const uint16_t, green: *const uint16_t, blue: *const uint16_t) -> c_int;
    pub fn drmModeCrtcGetGamma(fd: c_int, crtc_id: uint32_t, size: uint32_t,
                               red: *mut uint16_t, green: *mut uint16_t, blue: *mut uint16_t) -> c_int;
    pub fn drmModePageFlip(fd: c_int, crtc_id: uint32_t, fb_id: uint32_t,
                           flags: uint32_t, user_data: *const c_void) -> c_int;

    pub fn drmModeGetPlaneResources(fd: c_int) -> *const drmModePlaneRes;
    pub fn drmModeGetPlane(fd: c_int, plane_id: uint32_t) -> *const drmModePlane;
    pub fn drmModeSetPlane(fd: c_int, plane_id: uint32_t, crtc_id: uint32_t,
                           fb_id: uint32_t, flags: uint32_t,
                           crtc_x: int32_t, crtc_y: int32_t,
                           crtc_w: uint32_t, crtc_h: uint32_t,
                           src_x: uint32_t, src_y: uint32_t,
                           src_w: uint32_t, src_h: uint32_t) -> c_int;

    pub fn drmModeObjectGetProperties(fd: c_int,
                                      object_id: uint32_t,
                                      object_type: uint32_t) -> *const drmModeObjectProperties;

    pub fn drmModeFreeObjectProperties(ptr: *const drmModeObjectProperties);
    pub fn drmModeObjectSetProperty(fd: c_int, object_id: uint32_t,
                                    object_type: uint32_t, property_id: uint32_t,
                                    value: uint64_t) -> c_int;
}
