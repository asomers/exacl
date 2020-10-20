//! Utility functions and constants for the underlying system API.

use crate::flag::Flag;
use crate::perm::Perm;
use crate::qualifier::Qualifier;
use crate::sys::*;

use log::debug;
use scopeguard::defer;
use std::ffi::{c_void, CString};
use std::io;
use std::path::Path;
use std::ptr;
use uuid::Uuid;

// Re-export acl_entry_t and acl_t from crate::sys.
pub use crate::sys::{acl_entry_t, acl_t};

/// Free memory allocated by native acl_* routines.
pub(crate) fn xacl_free<T>(ptr: *mut T) {
    assert!(!ptr.is_null());
    let ret = unsafe { acl_free(ptr as *mut c_void) };
    assert_eq!(ret, 0);
}

/// Return true if path exists, even if it's a symlink to nowhere.
fn path_exists(path: &Path) -> bool {
    path.symlink_metadata().is_ok()
}

/// Get the native ACL for a specific file or directory.
///
/// If path is a symlink, get the link's ACL. Client must call xacl_free when
/// done.
pub(crate) fn xacl_get_file(path: &Path) -> io::Result<acl_t> {
    use std::os::unix::ffi::OsStrExt;
    let c_path = CString::new(path.as_os_str().as_bytes())?;

    let acl = unsafe { acl_get_link_np(c_path.as_ptr(), acl_type_t_ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let err = io::Error::last_os_error();
        debug!("acl_get_link_np({:?}) returned null, err={}", c_path, err);

        // acl_get_link_np can return NULL (ENOENT) if the file exists, but
        // there is no ACL. In the path exists, return an *empty* ACL.
        if let Some(code) = err.raw_os_error() {
            if code == ENOENT as i32 && path_exists(path) {
                debug!(" file exists! returning empty acl");
                return xacl_init(1);
            }
        }

        return Err(err);
    }

    Ok(acl)
}

pub(crate) fn xacl_set_file(path: &Path, acl: acl_t) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes())?;
    let ret = unsafe { acl_set_link_np(c_path.as_ptr(), acl_type_t_ACL_TYPE_EXTENDED, acl) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!(
            "acl_set_link_np({:?}) returned {}, err={}",
            c_path, ret, err
        );
        return Err(err);
    }

    Ok(())
}

/// Iterate over entries in a native ACL.
pub(crate) fn xacl_foreach<F: FnMut(acl_entry_t) -> io::Result<()>>(
    acl: acl_t,
    mut func: F,
) -> io::Result<()> {
    let mut entry: acl_entry_t = ptr::null_mut();
    let mut entry_id = acl_entry_id_t_ACL_FIRST_ENTRY;

    assert!(!acl.is_null());
    loop {
        let ret = unsafe { acl_get_entry(acl, entry_id, &mut entry) };
        if ret != 0 {
            // Errno is always EINVAL.
            break;
        }
        assert!(!entry.is_null());
        func(entry)?;
        entry_id = acl_entry_id_t_ACL_NEXT_ENTRY;
    }

    Ok(())
}

/// Create a new empty ACL with the given capacity.
///
/// Client must call xacl_free when done.
pub(crate) fn xacl_init(capacity: usize) -> io::Result<acl_t> {
    let acl = unsafe { acl_init(capacity as i32) }; // FIXME
    if acl.is_null() {
        let err = io::Error::last_os_error();
        debug!("acl_init({}) returned null, err={}", capacity, err);
        return Err(err);
    }
    Ok(acl)
}

/// Create a new entry in the specified ACL.
///
/// N.B. Memory reallocation may cause `acl` ptr to change.
pub(crate) fn xacl_create_entry(acl: &mut acl_t) -> io::Result<acl_entry_t> {
    let mut entry: acl_entry_t = ptr::null_mut();

    let ret = unsafe { acl_create_entry(&mut *acl, &mut entry) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!("acl_create_entry() returned {}, err={}", ret, err);
        return Err(err);
    }

    Ok(entry)
}

/// Get the GUID qualifier and resolve it to a User/Group if possible.
///
/// Only call this function for ACL_EXTENDED_ALLOW or ACL_EXTENDED_DENY.
fn xacl_get_qualifier(entry: acl_entry_t) -> io::Result<Qualifier> {
    let uuid_ptr = unsafe { acl_get_qualifier(entry) as *mut Uuid };
    if uuid_ptr.is_null() {
        let err = io::Error::last_os_error();
        debug!("acl_get_qualifier returned NULL, err={}", err);
        return Err(err);
    }
    defer! { xacl_free(uuid_ptr) }

    let guid = unsafe { *uuid_ptr };
    Qualifier::from_guid(guid)
}

/// Get tag and qualifier from the entry.
pub(crate) fn xacl_get_tag_qualifier(entry: acl_entry_t) -> io::Result<(bool, Qualifier)> {
    let mut tag = 0;
    let ret = unsafe { acl_get_tag_type(entry, &mut tag) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!("acl_get_tag_type() returned {}, err={}", ret, err);
        return Err(err);
    }

    #[allow(non_upper_case_globals)]
    let result = match tag {
        acl_tag_t_ACL_EXTENDED_ALLOW => (true, xacl_get_qualifier(entry)?),
        acl_tag_t_ACL_EXTENDED_DENY => (false, xacl_get_qualifier(entry)?),
        _ => (false, Qualifier::Unknown(tag.to_string())),
    };

    Ok(result)
}

// Get permissions from the entry.
pub(crate) fn xacl_get_perm(entry: acl_entry_t) -> io::Result<Perm> {
    let mut permset: acl_permset_t = std::ptr::null_mut();
    let ret = unsafe { acl_get_permset(entry, &mut permset) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!("acl_get_permset() returned {}, err={}", ret, err);
        return Err(err);
    }
    assert!(!permset.is_null());

    let mut perms = Perm::empty();
    for_each_1bit(Perm::all().bits(), |perm| {
        let res = unsafe { acl_get_perm_np(permset, perm) };
        debug_assert!(res >= 0 && res <= 1);
        if res == 1 {
            perms |= Perm::from_bits(perm).expect("unexpected permission bit");
        }
    });

    Ok(perms)
}

/// Get flags from the entry.
pub(crate) fn xacl_get_flags(entry: acl_entry_t) -> io::Result<Flag> {
    let mut flagset: acl_flagset_t = std::ptr::null_mut();
    let ret = unsafe { acl_get_flagset_np(entry as *mut c_void, &mut flagset) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!("acl_get_flagset_np() returned {}, err={}", ret, err);
        return Err(err);
    }
    assert!(!flagset.is_null());

    let mut flags = Flag::empty();
    for_each_1bit(Flag::all().bits(), |flag| {
        let res = unsafe { acl_get_flag_np(flagset, flag) };
        debug_assert!(res >= 0 && res <= 1);
        if res == 1 {
            flags |= Flag::from_bits(flag).expect("unexpected flag bit");
        }
    });

    Ok(flags)
}

/// Set qualifier for entry.
fn xacl_set_qualifier(entry: acl_entry_t, qualifier: &Qualifier) -> io::Result<()> {
    // Translate qualifier User/Group to guid.
    let guid = qualifier.guid()?;

    let ret = unsafe { acl_set_qualifier(entry, guid.as_bytes().as_ptr() as *mut c_void) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        debug!("acl_set_qualifier() returned {}, err={}", ret, err);
        return Err(err);
    }

    Ok(())
}

/// Set tag and qualifier for ACL entry.
pub(crate) fn xacl_set_tag_qualifier(
    entry: acl_entry_t,
    allow: bool,
    qualifier: &Qualifier,
) -> io::Result<()> {
    let tag = if let Qualifier::Unknown(_) = qualifier {
        debug_assert!(!allow);
        acl_tag_t_ACL_EXTENDED_DENY
    } else if allow {
        acl_tag_t_ACL_EXTENDED_ALLOW
    } else {
        acl_tag_t_ACL_EXTENDED_DENY
    };

    let ret = unsafe { acl_set_tag_type(entry, tag) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }

    xacl_set_qualifier(entry, &qualifier)?;

    Ok(())
}

/// Set permissions for the entry.
pub(crate) fn xacl_set_perm(entry: acl_entry_t, perms: Perm) -> io::Result<()> {
    let mut permset: acl_permset_t = std::ptr::null_mut();
    let ret = unsafe { acl_get_permset(entry, &mut permset) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }
    assert!(!permset.is_null());

    let ret = unsafe { acl_clear_perms(permset) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }

    for_each_1bit(perms.bits(), |perm| {
        let ret = unsafe { acl_add_perm(permset, perm) };
        debug_assert!(ret == 0);
    });

    Ok(())
}

pub(crate) fn xacl_set_flags(entry: acl_entry_t, flags: Flag) -> io::Result<()> {
    let mut flagset: acl_flagset_t = std::ptr::null_mut();
    let ret = unsafe { acl_get_flagset_np(entry as *mut c_void, &mut flagset) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }
    assert!(!flagset.is_null());

    let ret = unsafe { acl_clear_flags_np(flagset) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }

    for_each_1bit(flags.bits(), |flag| {
        let ret = unsafe { acl_add_flag_np(flagset, flag) };
        debug_assert!(ret == 0);
    });

    Ok(())
}

/// Apply a function for each 1 bit in the bitset.
fn for_each_1bit<F: FnMut(u32)>(bits: u32, mut func: F) {
    for i in 0..32 {
        if (1 << i) & bits != 0 {
            func(1 << i);
        }
    }
}

#[test]
fn test_for_each_1bit() {
    let mut v = Vec::new();

    for_each_1bit(0, |b| v.push(b));
    assert_eq!(v.len(), 0);

    for_each_1bit(1 + 2 + 4 + 32 + 2048, |b| v.push(b));
    assert_eq!(v, vec![1, 2, 4, 32, 2048]);
}