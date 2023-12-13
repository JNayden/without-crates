use core::panic;
use std::{ptr::{null, null_mut}, thread::sleep};
use without_crates::win32::*;

fn main()
{
    let class_name = "Sample Window Class";
    let class_name_wn = wide_null(class_name);
    
    let mut wc = WNDCLASSW::default();
    
    wc.lpfnWndProc = Some(WindowProc);
    wc.hInstance = get_process_handle();
    wc.lpszClassName = class_name_wn.as_ptr();
    //wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    wc.hCursor = get_load_cursorw(IDCursor::Arrow).unwrap();
    let _atom = unsafe {register_wndclass(&wc).unwrap_or_else(|()|
    {
        let last_error  = unsafe {GetLastError()};
        panic!("Could not register the windows class: {}", last_error);
    })};
    // let lparam:*mut i32 = (&mut 5) as *mut _; // in case we want someday our program to stop unexpectedly
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
    let hwnd = 
    unsafe 
    { 
        create_app_window(
            class_name, 
            "Sample Window Name",
            None, 
            [200, 200], 
            lparam.cast()
        )
    };
    let _previously_visible = unsafe {ShowWindow(hwnd.unwrap(), SW_SHOW)};
    
    // WNDPROC to dispatch messagess
    // GetMEssage
    loop
    {
        match get_any_message() 
        {
            Ok(msg) => 
            {
                if msg.message == WM_QUIT
                {
                    std::process::exit(msg.wParam as i32);
                }
                unsafe
                {
                    translate_message(&msg);
                    DispatchMessageW(&msg);
                }
            },
            Err(e) => panic!("Error when getting from the message queue: {}", e),
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
                let create_struct:*mut CREATESTRUCTW = lParam as *mut _;
                if create_struct.is_null()
                {
                    return 0;
                }
                let num_ptr = (*create_struct).lpCreateParams as *mut i32;
                // println!("Why: {}",*(num_ptr as *mut i32));
                println!("Windows give permission to windows proc to prepare the window for creation {} {}", lParam, wParam);
                return set_window_userdata::<i32>(hWnd, num_ptr).is_ok() as LRESULT;
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
            WM_DESTROY =>
            {
                match get_window_userdata::<i32>(hWnd)
                {
                    Ok(ptr) =>
                    {
                        // Box::from_raw(ptr)
                    },
                    Ok(_) => 
                    {
                        println!("No pointer for clearing");   
                    },
                    Err(e) =>
                    {
                        println!("Error when extractin' the userdata ptr {}", e);
                    }
                }
                post_quit_message();
            }
            WM_PAINT => 
            {
                match get_window_userdata::<i32>(hWnd) 
                {
                    Ok(ptr) => 
                    {
                        println!("Op: {}",*ptr);
                        *ptr += 1;
                    },
                    Ok(_) =>
                    {
                        println!("There is not information to extract extract from userdata");
                    }
                    Err(e) => 
                    {
                        println!("Error when extractin' the userdata ptr {}", e);
                    },
                }
                do_some_painting(hWnd, |hdc, _erase_bg, target_rect|
                {
                    let _ = fillrect_with_syscolor(hdc, &target_rect, SysColor::Window);
                    Ok(())
                }).unwrap_or_else(|e| println!("error during painting: {}", e))
            }
            _ => 
            {
                return DefWindowProcW(hWnd, Msg, wParam, lParam);
            }
        }
        0
    }