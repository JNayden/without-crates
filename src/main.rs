use core::panic;
use std::{ptr::{null, null_mut}, thread::sleep};

macro_rules! default_zeroed 
{
    ($t:ty) =>
    {
        impl $t
        {
            #[inline]
            #[must_use]
            fn default() -> Self
            {
                unsafe {core::mem::zeroed()}
            }
        }
    };
}

type UINT = u32;
type LONG_PTR = isize;
type UINT_PTR = usize;
type INT_PTR = isize;
type LRESULT = LONG_PTR;
type WPARAM = UINT_PTR;
type LPARAM = LONG_PTR;
type HWND = LPVOID;
type LPVOID = *mut core::ffi::c_void;
type PVOID = *mut core::ffi::c_void;
type HANDLE = PVOID;
type HINSTANCE = HANDLE;
type HICON = HANDLE;
type HCURSOR = HICON;
type HBRUSH = HANDLE;
type LPCWSTR = *const WCHAR;
type WCHAR = wchar_t;
type wchar_t = u16;
type HMODULE = HINSTANCE;
type ATOM = WORD;
type WORD = u16;
type DWORD = u32;
type HMENU = HANDLE;
type BOOL = i32;
type LONG = i32;
type LPMSG = *mut MSG;
type HDC = HANDLE;
type BYTE = u8;
type LPPAINTSTRUCT = *mut PAINTSTRUCT;
type LPWSTR = *mut WCHAR;
type ULONG_PTR = usize;
// #define CALLBACK __stdcall
const SW_SHOW: i32 = 5;

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU  
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
const CW_USEDEFAULT: i32 = -2147483648i32;// 0x80000000_u32 as i32;

const WM_CLOSE: u32 = 0x0010;
const WM_DESTROY: u32 = 0x0002;
const WM_PAINT: u32 = 0x000F;
const COLOR_WINDOW: u32 = 5;
const WM_CREATE: u32 = 0x0001;
const WM_NCCREATE: u32 = 0x0081;
const IDC_ARROW: LPWSTR = MAKEINTRESOURCEW(32512);

const WM_SETCURSOR: u32 = 0x0020;
type WNDPROC = Option
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
    lpfnWndProc: WNDPROC,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}
#[repr(C)]
pub struct MSG
{
    hwnd: HWND, 
    message: UINT,
    wParam: WPARAM,
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
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserver: BYTE

}
#[repr(C)]
pub struct RECT 
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG
}
default_zeroed!(WNDCLASSW);
default_zeroed!(MSG);
default_zeroed!(POINT);
default_zeroed!(PAINTSTRUCT);

pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR
{
    i as ULONG_PTR as LPWSTR
}
fn main()
{
    let window_name = "Sample Window Class";
    let hi = unsafe{GetModuleHandleW(core::ptr::null())};
    let mut wc = WNDCLASSW::default();
    let class_name = wide_null(window_name);
    wc.lpfnWndProc = Some(WindowProc);
    wc.hInstance = hi;
    wc.lpszClassName = class_name.as_ptr();
   
    wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };

    let atom = unsafe {RegisterClassW(&wc)};
    if atom == 0
    {
        let last_error  = unsafe {GetLastError()};
        panic!("Could not register the windows class: {}", last_error);
    }

    let hwnd = unsafe { CreateWindowExW(0,                              // Optional window styles.
        wc.lpszClassName,                     // Window class
        wide_null("Learn to Program Windows").as_ptr(),    // Window text
        WS_OVERLAPPEDWINDOW,            // Window style
    
        // Size and position
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
    
        core::ptr::null_mut(),       // Parent window    
        core::ptr::null_mut(),       // Menu
        wc.hInstance,  // Instance handle
        core::ptr::null_mut()       // Additional application data
    ) };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
      }
    let _previously_visible = unsafe {ShowWindow(hwnd, SW_SHOW)};

    // WNDPROC to dispatch messagess
    // GetMEssage
    let mut msg = MSG::default();
    
    loop
    {
        let message = unsafe {GetMessageW( &mut msg, null_mut(), 0, 0)};
        if message == 0
        {
            //break;
            panic!("Received special message WM_QUIT")
        }
        else if message == -1
        {
            let last_error  = unsafe {GetLastError()};
            panic!("Could not register the windows class: {}", last_error);
        }
        else
        {
            unsafe {TranslateMessage(&msg)};
            unsafe {DispatchMessageW(&msg)};
        }
    }
}
pub unsafe extern "system" fn WindowProc(hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM ,
    lParam: LPARAM ) -> LRESULT
    {
        match Msg  
        {
            WM_NCCREATE =>
            {
                println!("Windows give permission to windows proc to prepare the window for creation {} {}", lParam, wParam);
                return 1;   
            }
            WM_CREATE =>
            {
                println!("Window was created before was become visible \npointer to the CREATESTRUCT: {}\nwParam: {}", lParam, wParam);          
            }
            
            WM_CLOSE => 
            {
                println!("Cleaned up the box.");
                DestroyWindow(hWnd);
            }
            WM_DESTROY =>{PostQuitMessage(0);}
            WM_PAINT => 
            {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hWnd, &mut ps);
                let filledRect = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 3) as HBRUSH);
                EndPaint(hWnd, &ps);
            }
            _ => 
            {
                return DefWindowProcW(hWnd, Msg, wParam, lParam);
            }
        }
        0
    }
#[link(name = "Kernel32")]
extern "system"
{
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetLastError() -> DWORD;
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
}