//! Implements the `Qualifier` type for internal use

use crate::failx::*;
use crate::sys::{
    getgrgid_r, getgrnam_r, getpwnam_r, getpwuid_r, gid_t, group, passwd, size_t, uid_t,
};
#[cfg(target_os = "macos")]
use crate::sys::{id_t, mbr_gid_to_uuid, mbr_uid_to_uuid, mbr_uuid_to_id, sg};

use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::ptr;
#[cfg(target_os = "macos")]
use uuid::Uuid;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
const OWNER_NAME: &str = "";
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
const OTHER_NAME: &str = "";
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
const MASK_NAME: &str = "";
#[cfg(target_os = "freebsd")]
const EVERYONE_NAME: &str = "";

/// A Qualifier specifies the principal that is allowed/denied access to a
/// resource.
#[derive(Debug, PartialEq)]
pub enum Qualifier {
    User(uid_t),
    Group(gid_t),

    #[cfg(target_os = "macos")]
    Guid(Uuid),

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    UserObj,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    GroupObj,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    Other,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    Mask,
    #[cfg(target_os = "freebsd")]
    Everyone,

    Unknown(String),
}

impl Qualifier {
    /// Create qualifier object from a GUID.
    #[cfg(target_os = "macos")]
    pub fn from_guid(guid: Uuid) -> io::Result<Qualifier> {
        let (id_c, idtype) = match guid_to_id(guid) {
            Ok(info) => info,
            Err(err) => {
                if let Some(sg::ENOENT) = err.raw_os_error() {
                    return Ok(Qualifier::Guid(guid));
                }
                return Err(err);
            }
        };

        let qualifier = match idtype {
            sg::ID_TYPE_UID => Qualifier::User(id_c),
            sg::ID_TYPE_GID => Qualifier::Group(id_c),
            _ => Qualifier::Unknown(guid.to_string()),
        };

        Ok(qualifier)
    }

    /// Create qualifier object from a user name.
    #[cfg(target_os = "macos")]
    pub fn user_named(name: &str) -> io::Result<Qualifier> {
        match str_to_uid(name) {
            Ok(uid) => Ok(Qualifier::User(uid)),
            Err(err) => {
                // Try to parse name as a GUID.
                if let Ok(uuid) = Uuid::parse_str(name) {
                    Qualifier::from_guid(uuid)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Create qualifier object from a user name.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn user_named(name: &str) -> io::Result<Qualifier> {
        match name {
            OWNER_NAME => Ok(Qualifier::UserObj),
            s => match str_to_uid(s) {
                Ok(uid) => Ok(Qualifier::User(uid)),
                Err(err) => Err(err),
            },
        }
    }

    /// Create qualifier object from a group name.
    #[cfg(target_os = "macos")]
    pub fn group_named(name: &str) -> io::Result<Qualifier> {
        match str_to_gid(name) {
            Ok(gid) => Ok(Qualifier::Group(gid)),
            Err(err) => {
                if let Ok(uuid) = Uuid::parse_str(name) {
                    Qualifier::from_guid(uuid)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Create qualifier object from a group name.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn group_named(name: &str) -> io::Result<Qualifier> {
        match name {
            OWNER_NAME => Ok(Qualifier::GroupObj),
            s => match str_to_gid(s) {
                Ok(gid) => Ok(Qualifier::Group(gid)),
                Err(err) => Err(err),
            },
        }
    }

    /// Create qualifier from mask.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn mask_named(name: &str) -> io::Result<Qualifier> {
        match name {
            MASK_NAME => Ok(Qualifier::Mask),
            s => fail_custom(&format!("unknown mask name: {:?}", s)),
        }
    }

    /// Create qualifier from other.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn other_named(name: &str) -> io::Result<Qualifier> {
        match name {
            OTHER_NAME => Ok(Qualifier::Other),
            s => fail_custom(&format!("unknown other name: {:?}", s)),
        }
    }

    /// Create qualifier from everyone.
    #[cfg(target_os = "freebsd")]
    pub fn everyone_named(name: &str) -> io::Result<Qualifier> {
        match name {
            EVERYONE_NAME => Ok(Qualifier::Everyone),
            s => fail_custom(&format!("unknown everyone name: {:?}", s)),
        }
    }

    /// Return the GUID for the user/group.
    #[cfg(target_os = "macos")]
    pub fn guid(&self) -> io::Result<Uuid> {
        match self {
            Qualifier::User(uid) => uid_to_guid(*uid),
            Qualifier::Group(gid) => gid_to_guid(*gid),
            Qualifier::Guid(guid) => Ok(*guid),
            Qualifier::Unknown(tag) => fail_custom(&format!("unknown tag: {:?}", tag)),
        }
    }

    /// Return the name of the user/group.
    pub fn name(&self) -> String {
        match self {
            Qualifier::User(uid) => uid_to_str(*uid),
            Qualifier::Group(gid) => gid_to_str(*gid),
            #[cfg(target_os = "macos")]
            Qualifier::Guid(guid) => guid.to_string(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::UserObj | Qualifier::GroupObj => OWNER_NAME.to_string(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::Other => OTHER_NAME.to_string(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::Mask => MASK_NAME.to_string(),
            #[cfg(target_os = "freebsd")]
            Qualifier::Everyone => EVERYONE_NAME.to_string(),

            Qualifier::Unknown(s) => s.clone(),
        }
    }
}

impl fmt::Display for Qualifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Qualifier::User(uid) => write!(f, "user:{}", uid),
            Qualifier::Group(gid) => write!(f, "group:{}", gid),
            #[cfg(target_os = "macos")]
            Qualifier::Guid(guid) => write!(f, "guid:{}", guid),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::UserObj => write!(f, "user"),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::GroupObj => write!(f, "group"),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::Other => write!(f, "other"),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            Qualifier::Mask => write!(f, "mask"),
            #[cfg(target_os = "freebsd")]
            Qualifier::Everyone => write!(f, "everyone"),
            Qualifier::Unknown(s) => write!(f, "unknown:{}", s),
        }
    }
}

/// Convert user name to uid.
fn str_to_uid(name: &str) -> io::Result<uid_t> {
    let mut pwd = mem::MaybeUninit::<passwd>::uninit();
    let mut buf = Vec::<i8>::with_capacity(4096);
    let mut result = ptr::null_mut();

    let cstr = CString::new(name)?;
    let ret = unsafe {
        getpwnam_r(
            cstr.as_ptr(),
            pwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.capacity() as size_t,
            &mut result,
        )
    };

    if ret == 0 && !result.is_null() {
        let uid = unsafe { pwd.assume_init().pw_uid };
        return Ok(uid);
    }

    // Try to parse name as a decimal user ID.
    if let Ok(num) = name.parse::<u32>() {
        return Ok(num);
    }

    fail_custom(&format!("unknown user name: {:?}", name))
}

/// Convert group name to gid.
fn str_to_gid(name: &str) -> io::Result<gid_t> {
    let mut grp = mem::MaybeUninit::<group>::uninit();
    let mut buf = Vec::<i8>::with_capacity(4096);
    let mut result = ptr::null_mut();

    let cstr = CString::new(name)?;
    let ret = unsafe {
        getgrnam_r(
            cstr.as_ptr(),
            grp.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.capacity() as size_t,
            &mut result,
        )
    };

    if ret == 0 && !result.is_null() {
        let gid = unsafe { grp.assume_init().gr_gid };
        return Ok(gid);
    }

    // Try to parse name as a decimal group ID.
    if let Ok(num) = name.parse::<u32>() {
        return Ok(num);
    }

    fail_custom(&format!("unknown group name: {:?}", name))
}

/// Convert uid to user name.
fn uid_to_str(uid: uid_t) -> String {
    let mut pwd = mem::MaybeUninit::<passwd>::uninit();
    let mut buf = Vec::<i8>::with_capacity(4096);
    let mut result = ptr::null_mut();

    let ret = unsafe {
        getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.capacity() as size_t,
            &mut result,
        )
    };

    if ret == 0 && !result.is_null() {
        let cstr = unsafe { CStr::from_ptr(pwd.assume_init().pw_name) };
        cstr.to_string_lossy().into_owned()
    } else {
        uid.to_string()
    }
}

/// Convert gid to group name.
fn gid_to_str(gid: gid_t) -> String {
    let mut grp = mem::MaybeUninit::<group>::uninit();
    let mut buf = Vec::<i8>::with_capacity(4096);
    let mut result = ptr::null_mut();

    let ret = unsafe {
        getgrgid_r(
            gid,
            grp.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.capacity() as size_t,
            &mut result,
        )
    };

    if ret == 0 && !result.is_null() {
        let cstr = unsafe { CStr::from_ptr(grp.assume_init().gr_name) };
        cstr.to_string_lossy().into_owned()
    } else {
        gid.to_string()
    }
}

/// Convert uid to GUID.
#[cfg(target_os = "macos")]
fn uid_to_guid(uid: uid_t) -> io::Result<Uuid> {
    let guid = Uuid::nil();

    // On error, returns one of {EIO, ENOENT, EAUTH, EINVAL, ENOMEM}.
    let ret = unsafe { mbr_uid_to_uuid(uid, guid.as_bytes().as_ptr() as *mut u8) };
    if ret != 0 {
        return fail_from_err(ret, "mbr_uid_to_uuid", uid);
    }

    Ok(guid)
}

/// Convert gid to GUID.
#[cfg(target_os = "macos")]
fn gid_to_guid(gid: gid_t) -> io::Result<Uuid> {
    let guid = Uuid::nil();

    // On error, returns one of {EIO, ENOENT, EAUTH, EINVAL, ENOMEM}.
    let ret = unsafe { mbr_gid_to_uuid(gid, guid.as_bytes().as_ptr() as *mut u8) };
    if ret != 0 {
        return fail_from_err(ret, "mbr_gid_to_uuid", gid);
    }

    Ok(guid)
}

/// Convert GUID to uid/gid.
#[cfg(target_os = "macos")]
fn guid_to_id(guid: Uuid) -> io::Result<(id_t, i32)> {
    let mut id_c: id_t = 0;
    let mut idtype: i32 = 0;
    let guid_ptr = guid.as_bytes().as_ptr() as *mut u8;

    // On error, returns one of {EIO, ENOENT, EAUTH, EINVAL, ENOMEM}.
    let ret = unsafe { mbr_uuid_to_id(guid_ptr, &mut id_c, &mut idtype) };
    if ret != 0 {
        return fail_from_err(ret, "mbr_uuid_to_id", guid);
    }

    Ok((id_c, idtype))
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod qualifier_tests {
    use super::*;

    #[test]
    fn test_str_to_uid() {
        let msg = str_to_uid("").unwrap_err().to_string();
        assert_eq!(msg, "unknown user name: \"\"");

        let msg = str_to_uid("non_existant").unwrap_err().to_string();
        assert_eq!(msg, "unknown user name: \"non_existant\"");

        assert_eq!(str_to_uid("500").ok(), Some(500));

        #[cfg(target_os = "macos")]
        assert_eq!(str_to_uid("_spotlight").ok(), Some(89));

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        assert_eq!(str_to_uid("daemon").ok(), Some(1));
    }

    #[test]
    fn test_str_to_gid() {
        let msg = str_to_gid("").unwrap_err().to_string();
        assert_eq!(msg, "unknown group name: \"\"");

        let msg = str_to_gid("non_existant").unwrap_err().to_string();
        assert_eq!(msg, "unknown group name: \"non_existant\"");

        assert_eq!(str_to_gid("500").ok(), Some(500));

        #[cfg(target_os = "macos")]
        assert_eq!(str_to_gid("_spotlight").ok(), Some(89));

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        assert_eq!(str_to_gid("daemon").ok(), Some(1));
    }

    #[test]
    fn test_uid_to_str() {
        assert_eq!(uid_to_str(1500), "1500");

        #[cfg(target_os = "macos")]
        assert_eq!(uid_to_str(89), "_spotlight");

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        assert_eq!(uid_to_str(1), "daemon");
    }

    #[test]
    fn test_gid_to_str() {
        assert_eq!(gid_to_str(1500), "1500");

        #[cfg(target_os = "macos")]
        assert_eq!(gid_to_str(89), "_spotlight");

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        assert_eq!(gid_to_str(1), "daemon");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_uid_to_guid() {
        assert_eq!(
            uid_to_guid(89).ok(),
            Some(Uuid::parse_str("ffffeeee-dddd-cccc-bbbb-aaaa00000059").unwrap())
        );

        assert_eq!(
            uid_to_guid(1500).ok(),
            Some(Uuid::parse_str("ffffeeee-dddd-cccc-bbbb-aaaa000005dc").unwrap())
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_gid_to_guid() {
        assert_eq!(
            gid_to_guid(89).ok(),
            Some(Uuid::parse_str("abcdefab-cdef-abcd-efab-cdef00000059").unwrap())
        );

        assert_eq!(
            gid_to_guid(1500).ok(),
            Some(Uuid::parse_str("aaaabbbb-cccc-dddd-eeee-ffff000005dc").unwrap())
        );

        assert_eq!(
            gid_to_guid(20).ok(),
            Some(Uuid::parse_str("abcdefab-cdef-abcd-efab-cdef00000014").unwrap())
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_guid_to_id() {
        assert_eq!(
            guid_to_id(Uuid::parse_str("ffffeeee-dddd-cccc-bbbb-aaaa00000059").unwrap()).ok(),
            Some((89, sg::ID_TYPE_UID))
        );

        assert_eq!(
            guid_to_id(Uuid::parse_str("ffffeeee-dddd-cccc-bbbb-aaaa000005dc").unwrap()).ok(),
            Some((1500, sg::ID_TYPE_UID))
        );

        assert_eq!(
            guid_to_id(Uuid::parse_str("abcdefab-cdef-abcd-efab-cdef00000059").unwrap()).ok(),
            Some((89, sg::ID_TYPE_GID))
        );

        assert_eq!(
            guid_to_id(Uuid::parse_str("aaaabbbb-cccc-dddd-eeee-ffff000005dc").unwrap()).ok(),
            Some((1500, sg::ID_TYPE_GID))
        );

        assert_eq!(
            guid_to_id(Uuid::parse_str("abcdefab-cdef-abcd-efab-cdef00000014").unwrap()).ok(),
            Some((20, sg::ID_TYPE_GID))
        );

        let err = guid_to_id(Uuid::nil()).unwrap_err();
        assert_eq!(err.raw_os_error().unwrap(), sg::ENOENT);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_from_guid() {
        let user =
            Qualifier::from_guid(Uuid::parse_str("ffffeeee-dddd-cccc-bbbb-aaaa00000059").unwrap())
                .ok();
        assert_eq!(user, Some(Qualifier::User(89)));

        let group =
            Qualifier::from_guid(Uuid::parse_str("abcdefab-cdef-abcd-efab-cdef00000059").unwrap())
                .ok();
        assert_eq!(group, Some(Qualifier::Group(89)));

        let user = Qualifier::from_guid(Uuid::nil()).ok();
        assert_eq!(user, Some(Qualifier::Guid(Uuid::nil())));
    }

    #[test]
    fn test_user_named() {
        let user = Qualifier::user_named("89").ok();
        assert_eq!(user, Some(Qualifier::User(89)));

        #[cfg(target_os = "macos")]
        {
            let user = Qualifier::user_named("_spotlight").ok();
            assert_eq!(user, Some(Qualifier::User(89)));

            let user = Qualifier::user_named("ffffeeee-dddd-cccc-bbbb-aaaa00000059").ok();
            assert_eq!(user, Some(Qualifier::User(89)));
        }

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            let user = Qualifier::user_named("daemon").ok();
            assert_eq!(user, Some(Qualifier::User(1)));
        }
    }

    #[test]
    fn test_group_named() {
        let group = Qualifier::group_named("89").ok();
        assert_eq!(group, Some(Qualifier::Group(89)));

        #[cfg(target_os = "macos")]
        {
            let group = Qualifier::group_named("_spotlight").ok();
            assert_eq!(group, Some(Qualifier::Group(89)));

            let group = Qualifier::group_named("abcdefab-cdef-abcd-efab-cdef00000059").ok();
            assert_eq!(group, Some(Qualifier::Group(89)));
        }

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            let group = Qualifier::group_named("daemon").ok();
            assert_eq!(group, Some(Qualifier::Group(1)));
        }
    }
}
