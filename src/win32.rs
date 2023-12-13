use std::ptr::null_mut;

macro_rules! default_zeroed 
{
    ($t:ty) =>
    {
        impl $t
        {
            #[inline]
            #[must_use]
            pub fn default() -> Self
            {
                unsafe {core::mem::zeroed()}
            }
        }
    };
}

pub type UINT = u32;
pub type LONG_PTR = isize;
pub type UINT_PTR = usize;
pub type INT_PTR = isize;
pub type LRESULT = LONG_PTR;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type HWND = LPVOID;
pub type LPVOID = *mut core::ffi::c_void;
pub type PVOID = *mut core::ffi::c_void;
pub type HANDLE = PVOID;
pub type HINSTANCE = HANDLE;
pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type LPCWSTR = *const WCHAR;
pub type WCHAR = wchar_t;
pub type wchar_t = u16;
pub type HMODULE = HINSTANCE;
pub type ATOM = WORD;
pub type WORD = u16;
pub type DWORD = u32;
pub type HMENU = HANDLE;
pub type BOOL = i32;
pub type LONG = i32;
pub type LPMSG = *mut MSG;
pub type HDC = HANDLE;
pub type BYTE = u8;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPWSTR = *mut WCHAR;
pub type ULONG_PTR = usize;
pub type LPCVOID = *const core::ffi::c_void;
pub type va_list = *mut c_char;
pub type c_char = i8;
pub type HLOCAL = HANDLE;

pub const SW_SHOW: i32 = 5;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU  
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: i32 = -2147483648i32;// 0x80000000_u32 as i32;

pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const COLOR_WINDOW: u32 = 5;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_NCCREATE: u32 = 0x0081;
pub const IDC_ARROW: LPWSTR = MAKEINTRESOURCEW(32512);

pub const WM_SETCURSOR: u32 = 0x0020;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;
pub const WM_QUIT:u32 = 0x0012;
pub const GWLP_USERDATA: i32 = -21;

pub type WNDPROC = Option
<
    unsafe extern "system" //stdcall
    fn (hWnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM) -> LRESULT
>;

#[repr(C)]
pub struct WNDCLASSW
{
     style: UINT,
    pub lpfnWndProc: WNDPROC,
     cbClsExtra: i32,
     cbWndExtra: i32,
    pub hInstance: HINSTANCE,
     hIcon: HICON,
    pub hCursor: HCURSOR,
     hbrBackground: HBRUSH,
     lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}
#[repr(C)]
pub struct MSG
{
    hwnd: HWND, 
    pub message: UINT,
    pub wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD
}

#[repr(C)]
pub struct POINT 
{
    x: LONG,
    y: LONG
}
#[repr(C)]
pub struct PAINTSTRUCT 
{
    hdc: HDC,
    fErase: BOOL,
    pub rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserver: BYTE

}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RECT 
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG
}

#[repr(C)]
pub struct CREATESTRUCTW {
  pub lpCreateParams: LPVOID,
  hInstance: HINSTANCE,
  hMenu: HMENU,
  hwndParent: HWND,
  cy: i32,
  cx: i32,
  y: i32,
  x: i32,
  style: LONG,
  lpszName: LPCWSTR,
  lpszClass: LPCWSTR,
  dwExStyle: DWORD,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub DWORD);

default_zeroed!(WNDCLASSW);
default_zeroed!(MSG);
default_zeroed!(POINT);
default_zeroed!(PAINTSTRUCT);
default_zeroed!(CREATESTRUCTW);

pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR
{
    i as ULONG_PTR as LPWSTR
}
pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
}
pub fn get_process_handle() -> HMODULE 
{
    // Safety: as per the MSDN docs.
    unsafe { GetModuleHandleW(std::ptr::null()) }
}
pub fn get_load_cursorw(cursor: IDCursor) -> Result<HCURSOR, Win32Error>
{

    let hcursor = unsafe { LoadCursorW(null_mut(), MAKEINTRESOURCEW(cursor as WORD)) };
    if hcursor.is_null()
    {
        return Err(get_last_error())
    }
    else
    {
        return Ok(hcursor)
    }    
}
pub unsafe fn register_wndclass(wc: &WNDCLASSW) -> Result<ATOM, ()>
{
    let atom = unsafe {RegisterClassW(wc)};
    if atom  == 0
    {
        Err(())
    }
    else
    {
        Ok(atom)
    }
}
pub fn get_last_error() -> Win32Error {
    Win32Error(unsafe { GetLastError() })
}
impl core::fmt::Display for Win32Error
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 & (1 << 29) > 0 
        { 
            return write!(f, "Win32ApplicationError({})", self.0);
        }
        let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
        | FORMAT_MESSAGE_FROM_SYSTEM
        | FORMAT_MESSAGE_IGNORE_INSERTS;
      let lpSource = null_mut();
      let dwMessageId = self.0; //self.0
      let dwLanguageId = 0;
      let mut buffer: *mut u16 = null_mut();
      let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
      let nSize = 0;
      let Arguments = null_mut();
      let tchar_count_excluding_null = unsafe {
        FormatMessageW(
          dwFlags,
          lpSource,
          dwMessageId,
          dwLanguageId,
          lpBuffer,
          nSize,
          Arguments,
        )
      };
      
    if tchar_count_excluding_null == 0 || buffer.is_null()
    {
        // some sort of problem happened. we can't usefully get_last_error since
        // Display formatting doesn't let you give an error value.
        return Err(core::fmt::Error);
    }
    else 
    {
        struct OnDropLocalFree(HLOCAL);
        impl Drop for OnDropLocalFree 
        {
            fn drop(&mut self) 
            {
                unsafe { LocalFree(self.0) };
            }
        }
        let _on_drop = OnDropLocalFree(buffer as HLOCAL);
        let buffer_slice: &[u16] = unsafe {core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)};
        for decode_result in core::char::decode_utf16(buffer_slice.iter().copied())
        {
            let ch = decode_result.unwrap_or('�');
            write!(f, "{}", ch)?;
        }
        return Ok(());
      }
    }
}
pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}

pub unsafe fn create_app_window
(
            lpClassName: &str,
            lpWindowName: &str,
            position: Option<[i32;2]>,
            [width, height]:[i32;2], 
            lParam: LPVOID,
)-> Result<HWND, Win32Error>
{
    let class_name = wide_null(lpClassName);
    let window_name = wide_null(lpWindowName);

    let (x, y) = match position 
    {
        Some([x, y]) => (x, y),
        None => (CW_USEDEFAULT, CW_USEDEFAULT),
    };
    let hwnd =  CreateWindowExW(0,                              // Optional window styles.
        class_name.as_ptr(),                     // Window class
        window_name.as_ptr(),    // Window text
        WS_OVERLAPPEDWINDOW,            // Window style
    
        // Size and position
        x, 
        y,
        width, 
        height,
    
        core::ptr::null_mut(),       // Parent window    
        core::ptr::null_mut(),       // Menu
        get_process_handle(),  // Instance handle
        lParam       // Additional application data
    ) ;
    if hwnd.is_null() 
    {
        Err(get_last_error())
    }
    else 
    {
        Ok(hwnd)
    }
}
#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error>
{
    let mut msg = MSG::default();
    let msg_output = unsafe {GetMessageW( &mut msg, null_mut(), 0, 0)};
    if msg_output == -1
    {
        Err(get_last_error())
    }
    else
    {
        Ok(msg)
    }
}
pub fn translate_message(msg: &MSG) -> bool
{
    0 != unsafe {TranslateMessage(msg)}
}
pub fn set_last_error(e: Win32Error) 
{
    unsafe { SetLastError(e.0) }
}
pub unsafe fn set_window_userdata<T>(
    hwnd: HWND, ptr: *mut T,
  ) -> Result<*mut T, Win32Error> 
  {
    set_last_error(Win32Error(0));
    let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR);
    if out == 0 
    {
        // if output is 0, it's only a "real" error if the last_error is non-zero
        let last_error = get_last_error();
        if last_error.0 != 0 
        {
            Err(last_error)
        } 
        else 
        {
            Ok(out as *mut T)
        }
    }
    else 
    {
        Ok(out as *mut T)
    }
}
pub unsafe fn get_window_userdata<T>(hWnd: HWND) -> Result<*mut T, Win32Error>
{
    set_last_error(Win32Error(0));
    let out = GetWindowLongPtrW(hWnd, GWLP_USERDATA);
    if out == 0
    {
        let last_error = get_last_error();
        if last_error.0 != 0
        {
            Err(last_error)
        }
        else
        {
            Ok(out as *mut T)
        }
    }
    else
    {
        Ok(out as *mut T)
    }
}
pub fn post_quit_message()
{
    unsafe { PostQuitMessage(0) };
}
pub fn begin_paint(hWnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error>
{
    let mut ps = PAINTSTRUCT::default();
    let hdc = unsafe {BeginPaint(hWnd, &mut ps)};
    if hdc.is_null()
    {
        Err(get_last_error())
    }
    else
    {
        Ok((hdc,ps))
    }
}
/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
  _3dDarkShadow = 21,
  _3dLight = 22,
  ActiveBorder = 10,
  ActiveCaption = 2,
  AppWorkspace = 12,
  /// Button face, also "3D face" color.
  ButtonFace = 15,
  /// Button highlight, also "3D highlight" color.
  ButtonHighlight = 20,
  /// Button shadow, also "3D shadow" color.
  ButtonShadow = 16,
  ButtonText = 18,
  CaptionText = 9,
  /// Desktop background color
  Desktop = 1,
  GradientActiveCaption = 27,
  GradientInactiveCaption = 28,
  GrayText = 17,
  Highlight = 13,
  HighlightText = 14,
  HotLight = 26,
  InactiveBorder = 11,
  InactiveCaption = 3,
  InactiveCaptionText = 19,
  InfoBackground = 24,
  InfoText = 23,
  Menu = 4,
  MenuHighlight = 29,
  MenuBar = 30,
  MenuText = 7,
  ScrollBar = 0,
  Window = 5,
  WindowFrame = 6,
  WindowText = 8,
}

pub fn fillrect_with_syscolor(hdc: HDC, rect: &RECT, color: SysColor) -> Result<&RECT, Win32Error>
{
    if unsafe { FillRect(hdc, rect, (color as u32 + 3) as HBRUSH) } != 0
    {
        Ok(rect)
    }
    else
    {
        Err(get_last_error())
    }
}
pub unsafe fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) 
{
    EndPaint(hwnd, ps);
}
pub unsafe fn do_some_painting<F, T>(hWnd: HWND, f: F) -> Result<T, Win32Error>
where F: FnOnce(HDC, bool, RECT) -> Result<T, Win32Error>
{
    let(hdc, ps) = begin_paint(hWnd)?;
    let output = f(hdc, ps.fErase != 0, ps.rcPaint);
    end_paint(hWnd, &ps);
    output
}
// pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
#[link(name = "Kernel32")]
extern "system"
{
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetLastError() -> DWORD;
    pub fn FormatMessageW(dwFlags: DWORD,
                          lpSource: LPCVOID,
                          dwMessageId: DWORD,
                          dwLanguageId: DWORD,
                          lpBuffer: LPWSTR,
                          nSize: DWORD,
                          arguments: va_list) -> DWORD; // * const??
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    pub fn SetLastError(dwErrorCode: DWORD) -> ();
}

#[link(name = "User32")]
extern "system"
{
    pub fn RegisterClassW(unnamedParam: *const WNDCLASSW) -> ATOM;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    pub fn CreateWindowExW( dwExStyle: DWORD,
            lpClassName: LPCWSTR,
            lpWindowName: LPCWSTR,
            dwStyle: DWORD,
            x: i32,
            y: i32,
            nWidth: i32,
            nHeight: i32,
            hWndParent: HWND,
            hMenu: HMENU,
            hInstance: HINSTANCE,
            lParam: LPVOID) -> HWND;

    pub fn DefWindowProcW( hWnd: HWND,
           Msg: UINT,
           wParam: WPARAM ,
           lParam: LPARAM ) -> LRESULT;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG ) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG ) -> LRESULT;
    
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> i32;
    pub fn EndPaint(hWnd: HWND, lpPaint:*const PAINTSTRUCT) -> BOOL;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetWindowLongPtrW(hwnd: HWND, nIndex: i32, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn GetWindowLongPtrW(hwnd: HWND, nIndex: i32) -> LONG_PTR;
}