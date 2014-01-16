// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub fn to_return_suffix(ty: &str) -> ~str {
    match ty {
        "c_void" | "VOID" | "GLvoid" => ~"",
        ty_str => " -> " + ty_str.replace("*mut ", "*"),
    }
}

/// Converts a C style type definition to the Rust equivalent
pub fn to_rust_ty(ty: &str) -> &'static str {
    match ty {
        // gl.xml types
        "GLDEBUGPROC"               => "GLDEBUGPROC",
        "GLDEBUGPROCAMD"            => "GLDEBUGPROCAMD",
        "GLDEBUGPROCARB"            => "GLDEBUGPROCARB",
        "GLDEBUGPROCKHR"            => "GLDEBUGPROCKHR",
        "GLbitfield"                => "GLbitfield",
        "GLboolean"                 => "GLboolean",
        "GLbyte"                    => "GLbyte",
        "GLclampd"                  => "GLclampd",
        "GLclampf"                  => "GLclampf",
        "GLclampx"                  => "GLclampx",
        "GLdouble"                  => "GLdouble",
        "GLeglImageOES"             => "GLeglImageOES",
        "GLenum"                    => "GLenum",
        "GLfixed"                   => "GLfixed",
        "GLfloat"                   => "GLfloat",
        "GLhalfNV"                  => "GLhalfNV",
        "GLhandleARB"               => "GLhandleARB",
        "GLint"                     => "GLint",
        "GLint64EXT"                => "GLint64EXT",
        "GLintptr"                  => "GLintptr",
        "GLintptrARB"               => "GLintptrARB",
        "GLshort"                   => "GLshort",
        "GLsizei"                   => "GLsizei",
        "GLsizeiptr"                => "GLsizeiptr",
        "GLsizeiptrARB"             => "GLsizeiptrARB",
        "GLsync"                    => "GLsync",
        "GLubyte"                   => "GLubyte",
        "GLuint"                    => "GLuint",
        "GLuint64"                  => "GLuint64",
        "GLuint64EXT"               => "GLuint64EXT",
        "GLushort"                  => "GLushort",
        "GLvdpauSurfaceNV"          => "GLvdpauSurfaceNV",
        "void "                     => "c_void",
        "GLboolean *"               => "*mut GLboolean",
        "GLchar *"                  => "*mut GLchar",
        "GLcharARB *"               => "*mut GLcharARB",
        "GLdouble *"                => "*mut GLdouble",
        "GLenum *"                  => "*mut GLenum",
        "GLfixed *"                 => "*mut GLfixed",
        "GLfloat *"                 => "*mut GLfloat",
        "GLhandleARB *"             => "*mut GLhandleARB",
        "GLint *"                   => "*mut GLint",
        "GLint64 *"                 => "*mut GLint64",
        "GLint64EXT *"              => "*mut GLint64EXT",
        "GLsizei *"                 => "*mut GLsizei",
        "GLubyte *"                 => "*mut GLubyte",
        "GLuint *"                  => "*mut GLuint",
        "GLuint64 *"                => "*mut GLuint64",
        "GLuint64EXT *"             => "*mut GLuint64EXT",
        "GLushort *"                => "*mut GLushort",
        "GLvoid *"                  => "*mut GLvoid",
        "GLvoid **"                 => "**mut GLvoid",
        "void *"                    => "*mut c_void",
        "void **"                   => "**mut c_void",
        "const GLboolean *"         => "*GLboolean",
        "const GLbyte *"            => "*GLbyte",
        "const GLchar *"            => "*GLchar",
        "const GLcharARB *"         => "*GLcharARB",
        "const GLclampf *"          => "*GLclampf",
        "const GLdouble *"          => "*GLdouble",
        "const GLenum *"            => "*GLenum",
        "const GLfixed *"           => "*GLfixed",
        "const GLfloat *"           => "*GLfloat",
        "const GLhalfNV *"          => "*GLhalfNV",
        "const GLint *"             => "*GLint",
        "const GLint64EXT *"        => "*GLint64EXT",
        "const GLintptr *"          => "*GLintptr",
        "const GLshort *"           => "*GLshort",
        "const GLsizei *"           => "*GLsizei",
        "const GLsizeiptr *"        => "*GLsizeiptr",
        "const GLubyte *"           => "*GLubyte",
        "const GLuint *"            => "*GLuint",
        "const GLuint64 *"          => "*GLuint64",
        "const GLuint64EXT *"       => "*GLuint64EXT",
        "const GLushort *"          => "*GLushort",
        "const GLvdpauSurfaceNV *"  => "*GLvdpauSurfaceNV",
        "const GLvoid *"            => "*GLvoid",
        "const void *"              => "*c_void",
        "const void *const*"        => "**c_void",
        "const GLboolean **"        => "**GLboolean",
        "const GLchar **"           => "**GLchar",
        "const GLcharARB **"        => "**GLcharARB",
        "const GLvoid **"           => "**GLvoid",
        "const GLchar *const*"      => "**GLchar",
        "const GLvoid *const*"      => "**GLvoid",
        "struct _cl_context *"      => "*_cl_context",
        "struct _cl_event *"        => "*_cl_event",

        // glx.xml types
        "Bool"                      => "Bool",
        "Colormap"                  => "Colormap",
        "DMbuffer"                  => "DMbuffer",
        "Font"                      => "Font",
        "GLXContext"                => "GLXContext",
        "GLXContextID"              => "GLXContextID",
        "GLXDrawable"               => "GLXDrawable",
        "GLXFBConfig"               => "GLXFBConfig",
        "GLXFBConfigSGIX"           => "GLXFBConfigSGIX",
        "GLXPbuffer"                => "GLXPbuffer",
        "GLXPbufferSGIX"            => "GLXPbufferSGIX",
        "GLXPixmap"                 => "GLXPixmap",
        "GLXVideoCaptureDeviceNV"   => "GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV"          => "GLXVideoDeviceNV",
        "GLXVideoSourceSGIX"        => "GLXVideoSourceSGIX",
        "GLXWindow"                 => "GLXWindow",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        "Pixmap"                    => "Pixmap",
        "Status"                    => "Status",
        "VLNode"                    => "VLNode",
        "VLPath"                    => "VLPath",
        "VLServer"                  => "VLServer",
        "Window"                    => "Window",
        "__GLXextFuncPtr"           => "__GLXextFuncPtr",
        "const GLXContext"          => "const GLXContext",
        "float "                    => "c_float",
        "int "                      => "c_int",
        "int64_t"                   => "i64",
        "unsigned int "             => "c_uint",
        "unsigned long "            => "c_ulong",
        // "void "                     => "c_void",
        "DMparams *"                => "*mut DMparams",
        "Display *"                 => "*mut Display",
        "GLXFBConfig *"             => "*mut GLXFBConfig",
        "GLXFBConfigSGIX *"         => "*mut GLXFBConfigSGIX",
        "GLXHyperpipeConfigSGIX *"  => "*mut GLXHyperpipeConfigSGIX",
        "GLXHyperpipeNetworkSGIX *" => "*mut GLXHyperpipeNetworkSGIX",
        "GLXVideoCaptureDeviceNV *" => "*mut GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV *"        => "*mut GLXVideoDeviceNV",
        // "GLuint *"                  => "*mut GLuint",
        "XVisualInfo *"             => "*mut XVisualInfo",
        // "const GLubyte *"           => "*GLubyte",
        "const char *"              => "*c_char",
        "const int *"               => "*c_int",
        // "const void *"              => "*c_void",
        "int *"                     => "*mut c_int",
        "int32_t *"                 => "*mut i32",
        "int64_t *"                 => "*mut i64",
        "long *"                    => "*mut c_long",
        "unsigned int *"            => "*mut c_uint",
        "unsigned long *"           => "*mut c_ulong",
        // "void *"                    => "*mut c_void",

        // wgl.xml types
        "BOOL"                      => "BOOL",
        "DWORD"                     => "DWORD",
        "FLOAT"                     => "FLOAT",
        // "GLbitfield"                => "GLbitfield",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLfloat"                   => "GLfloat",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        // "GLushort"                  => "GLushort",
        "HANDLE"                    => "HANDLE",
        "HDC"                       => "HDC",
        "HENHMETAFILE"              => "HENHMETAFILE",
        "HGLRC"                     => "HGLRC",
        "HGPUNV"                    => "HGPUNV",
        "HPBUFFERARB"               => "HPBUFFERARB",
        "HPBUFFEREXT"               => "HPBUFFEREXT",
        "HPVIDEODEV"                => "HPVIDEODEV",
        "HVIDEOINPUTDEVICENV"       => "HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV"      => "HVIDEOOUTPUTDEVICENV",
        "INT"                       => "INT",
        "INT64"                     => "INT64",
        "LPCSTR"                    => "LPCSTR",
        "LPGLYPHMETRICSFLOAT"       => "LPGLYPHMETRICSFLOAT",
        "LPVOID"                    => "LPVOID",
        "PGPU_DEVICE"               => "PGPU_DEVICE",
        "PROC"                      => "PROC",
        "UINT"                      => "UINT",
        "VOID"                      => "VOID",
        // "int "                      => "c_int",
        // "unsigned int "             => "c_uint",
        // "void "                     => "c_void",
        "BOOL *"                    => "*mut BOOL",
        "DWORD *"                   => "*mut DWORD",
        "FLOAT *"                   => "*mut FLOAT",
        // "GLuint *"                  => "*mut GLuint",
        "HANDLE *"                  => "*mut HANDLE",
        "HGPUNV *"                  => "*mut HGPUNV",
        "HPVIDEODEV *"              => "*mut HPVIDEODEV",
        "HVIDEOINPUTDEVICENV *"     => "*mut HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV *"    => "*mut HVIDEOOUTPUTDEVICENV",
        "INT32 *"                   => "*mut INT32",
        "INT64 *"                   => "*mut INT64",
        "UINT *"                    => "*mut UINT",
        "USHORT *"                  => "*mut USHORT",
        "const COLORREF *"          => "*COLORREF",
        "const DWORD *"             => "*DWORD",
        "const FLOAT *"             => "*FLOAT",
        // "const GLushort *"          => "*GLushort",
        "const HANDLE *"            => "*HANDLE",
        "const HGPUNV *"            => "*HGPUNV",
        "const LAYERPLANEDESCRIPTOR *"  => "*LAYERPLANEDESCRIPTOR",
        "const LPVOID *"            => "*LPVOID",
        "const PIXELFORMATDESCRIPTOR *" => "*PIXELFORMATDESCRIPTOR",
        "const USHORT *"            => "*USHORT",
        // "const char *"              => "*c_char",
        // "const int *"               => "*c_int",
        "float *"                   => "*mut c_float",
        // "int *"                     => "*mut c_int",
        // "unsigned long *"           => "*mut c_ulong",
        // "void *"                    => "*mut c_void",

        // failure
        _ => fail!("Type conversion not implemented for `{}`", ty),
    }
}

pub type Src = &'static [&'static str];

pub static GL_ALIASES: Src = &[
    "// Common types from OpenGL 1.1",
    "pub type GLenum = c_uint;",
    "pub type GLboolean = c_uchar;",
    "pub type GLbitfield = c_uint;",
    "pub type GLvoid = c_void;",
    "pub type GLbyte = c_char;",
    "pub type GLshort = c_short;",
    "pub type GLint = c_int;",
    "pub type GLclampx = c_int;",
    "pub type GLubyte = c_uchar;",
    "pub type GLushort = c_ushort;",
    "pub type GLuint = c_uint;",
    "pub type GLsizei = c_int;",
    "pub type GLfloat = c_float;",
    "pub type GLclampf = c_float;",
    "pub type GLdouble = c_double;",
    "pub type GLclampd = c_double;",
    "pub type GLeglImageOES = *c_void;",
    "pub type GLchar = c_char;",
    "pub type GLcharARB = c_char;",
    "",
    "#[cfg(target_os = \"macos\")]",
    "pub type GLhandleARB = *c_void;",
    "#[cfg(not(target_os = \"macos\"))]",
    "pub type GLhandleARB = c_uint;",
    "",
    "pub type GLhalfARB = c_ushort;",
    "pub type GLhalf = c_ushort;",
    "",
    "// Must be 32 bits",
    "pub type GLfixed = GLint;",
    "",
    "pub type GLintptr = ptrdiff_t;",
    "pub type GLsizeiptr = ptrdiff_t;",
    "pub type GLint64 = i64;",
    "pub type GLuint64 = u64;",
    "pub type GLintptrARB = ptrdiff_t;",
    "pub type GLsizeiptrARB = ptrdiff_t;",
    "pub type GLint64EXT = i64;",
    "pub type GLuint64EXT = u64;",
    "",
    "pub struct __GLsync;",
    "pub type GLsync = *__GLsync;",
    "",
    "// compatible with OpenCL cl_context",
    "pub struct _cl_context;",
    "pub struct _cl_event;",
    "",
    "pub type GLDEBUGPROC = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);",
    "pub type GLDEBUGPROCARB = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);",
    "pub type GLDEBUGPROCKHR = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);",
    "",
    // "// GLES 1 types",
    // "pub type GLclampx = i32;",
    // "",
    // "// GLES 1/2 types (tagged for GLES 1)",
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",
    // "",
    // "// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)",
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLint64EXT = i64;",
    // "pub type GLuint64EXT = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",
    // "",
    // "// GLES 2 types (none currently)",
    // "",
    "// Vendor extension types",
    "pub type GLDEBUGPROCAMD = extern \"system\" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);",
    "pub type GLhalfNV = c_ushort;",
    "pub type GLvdpauSurfaceNV = GLintptr;",
];

pub static X_ALIASES: Src = &[
    "pub type XID = c_ulong;",
    "pub type Bool = c_int;         // Not sure if this is correct...",
    "pub struct Display;",
];

pub static GLX_ALIASES: Src = &[
    "pub type GLXFBConfigID = XID;",
    "pub type GLXFBConfig = *c_void;",
    "pub type GLXContextID = XID;",
    "pub type GLXContext = *c_void;",
    "pub type GLXPixmap = XID;",
    "pub type GLXDrawable = XID;",
    "pub type GLXWindow = XID;",
    "pub type GLXPbuffer = XID;",
    "pub type __GLXextFuncPtr = extern \"system\" fn();",
    "pub type GLXVideoCaptureDeviceNV = XID;",
    "pub type GLXVideoDeviceNV = unsigned int;",
    "pub type GLXVideoSourceSGIX = XID;",
    "pub type GLXFBConfigIDSGIX = XID;",
    "pub type GLXFBConfigSGIX = *c_void;",
    "pub type GLXPbufferSGIX = XID;",
    "",
    "pub struct GLXPbufferClobberEvent {",
    "    event_type: c_int,          // GLX_DAMAGED or GLX_SAVED",
    "    draw_type: c_int,           // GLX_WINDOW or GLX_PBUFFER",
    "    serial: c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came for SendEvent request",
    "    display: *Display,          // display the event was read from",
    "    drawable: GLXDrawable,      // XID of Drawable",
    "    buffer_mask: c_uint,        // mask indicating which buffers are affected",
    "    aux_buffer: c_uint,         // which aux buffer was affected",
    "    x: c_int,",
    "    y: c_int,",
    "    width: c_int,",
    "    height: c_int,",
    "    count: c_int,               // if nonzero, at least this many more",
    "}",
    "",
    "pub struct GLXBufferSwapComplete {",
    "    type: c_int,",
    "    serial: c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came from a SendEvent request",
    "    display: *Display,          // Display the event was read from",
    "    drawable: GLXDrawable,      // drawable on which event was requested in event mask",
    "    event_type: c_int,",
    "    ust: i64,",
    "    msc: i64,",
    "    sbc: i64,",
    "}",
    "",
    "// typedef union __GLXEvent {",
    "//     GLXPbufferClobberEvent glxpbufferclobber;",
    "//     GLXBufferSwapComplete glxbufferswapcomplete;",
    "//     long pad[24];",
    "// } GLXEvent;",
    "",
    "pub struct GLXBufferClobberEventSGIX {",
    "    type: c_int,",
    "    serial: c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came for SendEvent request",
    "    display: *Display,          // display the event was read from",
    "    drawable: GLXDrawable,      // i.d. of Drawable",
    "    event_type: c_int,          // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX",
    "    draw_type: c_int,           // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX",
    "    mask: c_uint,               // mask indicating which buffers are affected",
    "    x: c_int,",
    "    y: c_int,",
    "    width: c_int,",
    "    height: c_int,",
    "    count: c_int,               // if nonzero, at least this many more",
    "}",
    "",
    "pub struct GLXHyperpipeNetworkSGIX {",
    "    pipeName: [c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    networkId: c_int,",
    "}",
    "",
    "pub struct GLXHyperpipeConfigSGIX {",
    "    pipeName: [c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    channel: c_int,",
    "    participationType: c_uint,",
    "    timeSlice: c_int,",
    "}",
    "",
    "pub struct GLXPipeRect {",
    "    pipeName: [c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    srcXOrigin: c_int,",
    "    srcYOrigin: c_int,",
    "    srcWidth: c_int,",
    "    srcHeight: c_int,",
    "    destXOrigin: c_int,",
    "    destYOrigin: c_int,",
    "    destWidth: c_int,",
    "    destHeight: c_int,",
    "}",
    "",
    "pub struct GLXPipeRectLimits {",
    "    pipeName: [c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    XOrigin: c_int,",
    "    YOrigin: c_int,",
    "    maxHeight: c_int,",
    "    maxWidth: c_int,",
    "}",
];

pub static WIN_ALIASES: Src = &[
    "// From WinNT.h",
    "pub type CHAR = c_char;",
    "pub type HANDLE = PVOID;",
    "pub type LONG = c_long;",
    "pub type LPCSTR = *c_char;",
    "pub type VOID = c_void;",
    "",
    "// From Windef.h",
    "pub type BOOL = c_int;",
    "pub type BYTE = c_uchar;",
    "pub type COLORREF = DWORD;",
    "pub type FLOAT = c_float;",
    "pub type HDC = HANDLE;",
    "pub type HENHMETAFILE = HANDLE;",
    "pub type HGLRC = *c_void;",
    "pub type INT = c_int;",
    "pub type LPVOID = *c_void;",
    "pub type PROC = extern \"system\" fn();     // Not sure about this one :/",
    "pub struct RECT {",
    "    left: LONG,",
    "    top: LONG,",
    "    right: LONG,",
    "    bottom: LONG,",
    "}",
    "pub type UINT = c_uint;",
    "pub type USHORT = c_ushort;",
    "pub type WORD = c_ushort;",
    "",
    "// From BaseTsd.h",
    "pub type INT32 = i32;",
    "pub type INT64 = i64;",
    "",
    "// From IntSafe.h",
    "pub type DWORD = c_ulong;",
    "",
    "// From Wingdi.h",
    "pub struct POINTFLOAT {",
    "    x: FLOAT,",
    "    y: FLOAT,",
    "} ",
    "pub struct GLYPHMETRICSFLOAT {",
    "    gmfBlackBoxX: FLOAT,",
    "    gmfBlackBoxY: FLOAT,",
    "    gmfptGlyphOrigin: POINTFLOAT,",
    "    gmfCellIncX: FLOAT,",
    "    gmfCellIncY: FLOAT,",
    "}",
    "pub type LPGLYPHMETRICSFLOAT = *GLYPHMETRICSFLOAT;",
    "pub struct LAYERPLANEDESCRIPTOR {",
    "    nSize: WORD,",
    "    nVersion: WORD,",
    "    dwFlags: DWORD,",
    "    iPixelType: BYTE,",
    "    cColorBits: BYTE,",
    "    cRedBits: BYTE,",
    "    cRedShift: BYTE,",
    "    cGreenBits: BYTE,",
    "    cGreenShift: BYTE,",
    "    cBlueBits: BYTE,",
    "    cBlueShift: BYTE,",
    "    cAlphaBits: BYTE,",
    "    cAlphaShift: BYTE,",
    "    cAccumBits: BYTE,",
    "    cAccumRedBits: BYTE,",
    "    cAccumGreenBits: BYTE,",
    "    cAccumBlueBits: BYTE,",
    "    cAccumAlphaBits: BYTE,",
    "    cDepthBits: BYTE,",
    "    cStencilBits: BYTE,",
    "    cAuxBuffers: BYTE,",
    "    iLayerType: BYTE,",
    "    bReserved: BYTE,",
    "    crTransparent: COLORREF,",
    "}",
    "pub struct PIXELFORMATDESCRIPTOR {",
    "    nSize: WORD,",
    "    nVersion: WORD,",
    "    dwFlags: DWORD,",
    "    iPixelType: BYTE,",
    "    cColorBits: BYTE,",
    "    cRedBits: BYTE,",
    "    cRedShift: BYTE,",
    "    cGreenBits: BYTE,",
    "    cGreenShift: BYTE,",
    "    cBlueBits: BYTE,",
    "    cBlueShift: BYTE,",
    "    cAlphaBits: BYTE,",
    "    cAlphaShift: BYTE,",
    "    cAccumBits: BYTE,",
    "    cAccumRedBits: BYTE,",
    "    cAccumGreenBits: BYTE,",
    "    cAccumBlueBits: BYTE,",
    "    cAccumAlphaBits: BYTE,",
    "    cDepthBits: BYTE,",
    "    cStencilBits: BYTE,",
    "    cAuxBuffers: BYTE,",
    "    iLayerType: BYTE,",
    "    bReserved: BYTE,",
    "    dwLayerMask: DWORD,",
    "    dwVisibleMask: DWORD,",
    "    dwDamageMask: DWORD,",
    "}",
];

pub static WGL_ALIASES: Src = &[
    "// From WinNT.h",
    "// #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name",
    "macro_rules! DECLARE_HANDLE(",
    "    ($name:ident) => (",
    "        pub type $name = *c_void;",
    "    )",
    ")",
    "",
    "pub struct _GPU_DEVICE {",
    "    cb: DWORD,",
    "    DeviceName: [CHAR, ..32],",
    "    DeviceString: [CHAR, ..128],",
    "    Flags: DWORD,",
    "    rcVirtualScreen: RECT,",
    "}",
    "DECLARE_HANDLE!(HPBUFFERARB)",
    "DECLARE_HANDLE!(HPBUFFEREXT)",
    "DECLARE_HANDLE!(HVIDEOOUTPUTDEVICENV)",
    "DECLARE_HANDLE!(HPVIDEODEV)",
    "DECLARE_HANDLE!(HPGPUNV)",
    "DECLARE_HANDLE!(HGPUNV)",
    "DECLARE_HANDLE!(HVIDEOINPUTDEVICENV)",
    "pub struct GPU_DEVICE(_GPU_DEVICE);",
    "pub struct PGPU_DEVICE(*_GPU_DEVICE);",
];
