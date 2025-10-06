pub fn windows_reserved_names_are_allowed() -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use windows_sys::Win32::Storage::FileSystem::GetFullPathNameW;

    let test_file_name: Vec<_> = OsStr::new("aux.rs").encode_wide().collect();

    let buffer_length =
        unsafe { GetFullPathNameW(test_file_name.as_ptr(), 0, ptr::null_mut(), ptr::null_mut()) };

    if buffer_length == 0 {
        // This means the call failed, so we'll conservatively assume reserved names are not allowed.
        return false;
    }

    let mut buffer = vec![0u16; buffer_length as usize];

    let result = unsafe {
        GetFullPathNameW(
            test_file_name.as_ptr(),
            buffer_length,
            buffer.as_mut_ptr(),
            ptr::null_mut(),
        )
    };

    if result == 0 {
        // Once again, conservatively assume reserved names are not allowed if the
        // GetFullPathNameW call failed.
        return false;
    }

    // Under the old rules, a file name like aux.rs would get converted into \\.\aux, so
    // we detect this case by checking if the string starts with \\.\
    //
    // Otherwise, the filename will be something like C:\Users\Foo\Documents\aux.rs
    let prefix: Vec<_> = OsStr::new("\\\\.\\").encode_wide().collect();
    if buffer.starts_with(&prefix) {
        false
    } else {
        true
    }
}

pub fn windows_reserved_names_are_allowed2() -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use windows_sys::Win32::Storage::FileSystem::GetFullPathNameW;

    let test_file_name: Vec<_> = OsStr::new("aux.rs").encode_wide().chain([0]).collect();

    let buffer_length =
        unsafe { GetFullPathNameW(test_file_name.as_ptr(), 0, ptr::null_mut(), ptr::null_mut()) };

    if buffer_length == 0 {
        // This means the call failed, so we'll conservatively assume reserved names are not allowed.
        return false;
    }

    let mut buffer = vec![0u16; buffer_length as usize];

    let result = unsafe {
        GetFullPathNameW(
            test_file_name.as_ptr(),
            buffer_length,
            buffer.as_mut_ptr(),
            ptr::null_mut(),
        )
    };

    if result == 0 {
        // Once again, conservatively assume reserved names are not allowed if the
        // GetFullPathNameW call failed.
        return false;
    }

    // Under the old rules, a file name like aux.rs would get converted into \\.\aux, so
    // we detect this case by checking if the string starts with \\.\
    //
    // Otherwise, the filename will be something like C:\Users\Foo\Documents\aux.rs
    let prefix: Vec<_> = OsStr::new("\\\\.\\").encode_wide().collect();
    if buffer.starts_with(&prefix) {
        false
    } else {
        true
    }
}

#[test]
fn test_aux() {
    assert_eq!(windows_reserved_names_are_allowed(), true);
}

#[test]
fn test_auxnul() {
    assert_eq!(windows_reserved_names_are_allowed2(), true);
}
