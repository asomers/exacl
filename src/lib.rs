//! # exacl
//!
//! Manipulate file system access control lists (ACL) on `macOS` and `Linux`.
//!
//! ## Example
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use exacl::{getfacl, setfacl, AclEntry, Perm};
//!
//! // Get the ACL from "./tmp/foo".
//! let mut acl = getfacl("./tmp/foo", None)?;
//!
//! // Print the contents of the ACL.
//! for entry in &acl {
//!     println!("{}", entry);
//! }
//!
//! // Add an ACL entry to the end.
//! acl.push(AclEntry::allow_user("some_user", Perm::READ, None));
//!
//! // Sort the ACL in canonical order.
//! acl.sort();
//!
//! // Set the ACL for "./tmp/foo".
//! setfacl(&["./tmp/foo"], &acl, None)?;
//!
//! # Ok(()) }
//! ```
//!
//! ## High Level API
//!
//! This module provides two high level functions, [`getfacl`] and [`setfacl`].
//!
//! - [`getfacl`] retrieves the ACL for a file or directory.
//! - [`setfacl`] sets the ACL for files or directories.
//!
//! On Linux, the ACL contains entries for the default ACL, if present.
//!
//! Both [`getfacl`] and [`setfacl`] work with a `Vec<AclEntry>`. The
//! [`AclEntry`] structure contains five fields:
//!
//! - kind : [`AclEntryKind`] - the kind of entry (User, Group, Other, Mask,
//!     or Unknown).
//! - name : [`String`] - name of the principal being given access. You can
//!     use a user/group name, decimal uid/gid, or UUID (on macOS).
//! - perms : [`Perm`] - permission bits for the entry.
//! - flags : [`Flag`] - flags indicating whether an entry is inherited, etc.
//! - allow : [`bool`] - true if entry is allowed; false means deny. Linux only
//!     supports allow=true.
//!
//! [`AclEntry`] supports an ordering that corresponds to ACL canonical order. An
//! ACL in canonical order has deny entries first, and inherited entries last.
//! On Linux, entries for file-owner sort before named users. You can sort a
//! vector of `AclEntry` to put the ACL in canonical order.
//!
//! ## Low Level API
//!
//! The low level API is appropriate if you need finer grained control over
//! the ACL.
//!
//! - Manipulate the access ACL and default ACL independently on Linux.
//! - Manipulate the ACL's own flags on macOS.
//! - Use the platform specific text formats.
//!
//! The low level API uses the [`Acl`] class which wraps the native ACL object.
//! Each [`Acl`] is immutable once constructed. To manipulate its contents, you
//! can retrieve a mutable vector of [`AclEntry`], modify the vector's contents,
//! then create a new [`Acl`].

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod acl;
mod aclentry;
mod bititer;
mod failx;
mod flag;
mod format;
mod perm;
mod qualifier;
mod sys;
mod util;

// Export Acl, AclOption, AclEntry, AclEntryKind, Flag and Perm.
pub use acl::{Acl, AclOption};
pub use aclentry::{AclEntry, AclEntryKind};
pub use flag::Flag;
pub use perm::Perm;

use failx::{custom_err, fail_custom};
use std::io::{self, BufRead};
use std::path::Path;

/// Get access control list (ACL) for a file or directory.
///
/// On success, returns a vector of [`AclEntry`] with all access control entries
/// for the specified path. The semantics and permissions of the access control
/// list depend on the underlying platform.
///
/// # macOS
///
/// The ACL only includes the extended entries beyond the normal permssion mode
/// of the file. macOS provides several ACL entry flags to specify how entries
/// may be inherited by directory sub-items. If there's no extended ACL for a
/// file, this function may return zero entries.
///
/// If `path` points to a symlink, `getfacl` returns the ACL of the file pointed
/// to by the symlink. Use [`AclOption::SYMLINK_ACL`] to obtain the ACL of a symlink
/// itself.
///
/// [`AclOption::DEFAULT_ACL`] option is not supported on macOS.
///
/// # Linux
///
/// The ACL includes entries related to the permission mode of the file. These
/// are marked with empty names ("").
///
/// Both the access ACL and the default ACL are returned in one list, with
/// the default ACL entries indicated by a [`Flag::DEFAULT`] flag.
///
/// If `path` points to a symlink, `getfacl` returns the ACL of the file pointed
/// to by the symlink. [`AclOption::SYMLINK_ACL`] is not supported on Linux.
///
/// [`AclOption::DEFAULT_ACL`] causes `getfacl` to only include entries for the
/// default ACL, if present for a directory path. When called with
/// [`AclOption::DEFAULT_ACL`], `getfacl` may return zero entries.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use exacl::getfacl;
///
/// let entries = getfacl("./tmp/foo", None)?;
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an [`io::Error`] on failure.

pub fn getfacl<P, O>(path: P, options: O) -> io::Result<Vec<AclEntry>>
where
    P: AsRef<Path>,
    O: Into<Option<AclOption>>,
{
    _getfacl(path.as_ref(), options.into().unwrap_or_default())
}

#[cfg(target_os = "macos")]
fn _getfacl(path: &Path, options: AclOption) -> io::Result<Vec<AclEntry>> {
    Acl::read(path, options)?.entries()
}

#[cfg(not(target_os = "macos"))]
fn _getfacl(path: &Path, options: AclOption) -> io::Result<Vec<AclEntry>> {
    if options.contains(AclOption::DEFAULT_ACL) {
        Acl::read(path, options)?.entries()
    } else {
        let mut entries = Acl::read(path, options)?.entries()?;
        let mut default = Acl::read(
            path,
            options | AclOption::DEFAULT_ACL | AclOption::IGNORE_EXPECTED_FILE_ERR,
        )?
        .entries()?;

        entries.append(&mut default);
        Ok(entries)
    }
}

/// Set access control list (ACL) for specified files and directories.
///
/// Sets the ACL for the specified paths using the given access control entries.
/// The semantics and permissions of the access control list depend on the
/// underlying platform.
///
/// # macOS
///
/// The ACL contains extended entries beyond the usual mode permission bits.
/// An entry may allow or deny access to a specific user or group.
/// To specify inherited entries, use the provided [Flag] values.
///
/// ### macOS Example
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use exacl::{setfacl, AclEntry, Flag, Perm};
///
/// let entries = vec![
///     AclEntry::allow_user("some_user", Perm::READ | Perm::WRITE, None),
///     AclEntry::deny_group("some_group", Perm::WRITE, None)
/// ];
///
/// setfacl(&["./tmp/foo"], &entries, None)?;
/// # Ok(()) }
/// ```
///
/// # Linux
///
/// Each entry can only allow access; denying access using allow=false is not
/// supported on Linux.
///
/// The ACL *must* contain entries for the permssion modes of the file. Use
/// the [`AclEntry::allow_other`] and [`AclEntry::allow_mask`] functions to
/// specify the mode's other and mask permissions. Use "" as the name for the
/// file owner and group owner.
///
/// If an ACL contains a named user or group, there should be a
/// [`AclEntryKind::Mask`] entry included. If a one entry is not provided, one
/// will be computed.
///
/// The access control entries may include entries for the default ACL, if one
/// is desired. When `setfacl` is called with no [`Flag::DEFAULT`] entries, it
/// deletes the default ACL.
///
/// ### Linux Example
///
/// ```ignore
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use exacl::{setfacl, AclEntry, Flag, Perm};
///
/// let entries = vec![
///     AclEntry::allow_user("", Perm::READ | Perm::WRITE, None),
///     AclEntry::allow_group("", Perm::READ, None),
///     AclEntry::allow_other(Perm::empty(), None),
///     AclEntry::allow_user("some_user", Perm::READ | Perm::WRITE, None),
/// ];
///
/// setfacl(&["./tmp/foo"], &entries, None)?;
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an [`io::Error`] on failure.

pub fn setfacl<P, O>(paths: &[P], entries: &[AclEntry], options: O) -> io::Result<()>
where
    P: AsRef<Path>,
    O: Into<Option<AclOption>>,
{
    _setfacl(paths, entries, options.into().unwrap_or_default())
}

#[cfg(target_os = "macos")]
fn _setfacl<P>(paths: &[P], entries: &[AclEntry], options: AclOption) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let acl = Acl::from_entries(entries).map_err(|err| custom_err("Invalid ACL", &err))?;
    for path in paths {
        acl.write(path.as_ref(), options)?;
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn _setfacl<P>(paths: &[P], entries: &[AclEntry], options: AclOption) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if options.contains(AclOption::DEFAULT_ACL) {
        let acl = Acl::from_entries(entries).map_err(|err| custom_err("Invalid ACL", &err))?;

        for path in paths {
            acl.write(path.as_ref(), options)?;
        }
    } else {
        let (access_acl, default_acl) =
            Acl::from_unified_entries(entries).map_err(|err| custom_err("Invalid ACL", &err))?;

        if access_acl.is_empty() {
            fail_custom("Invalid ACL: missing required entries")?;
        }

        for path in paths {
            let path = path.as_ref();
            // Try to set default acl first. This will fail if path is not
            // a directory and default_acl is non-empty. This ordering
            // avoids leaving the file's ACL in a partially changed state
            // after an error (simply because it was a non-directory).
            default_acl.write(
                path,
                options | AclOption::DEFAULT_ACL | AclOption::IGNORE_EXPECTED_FILE_ERR,
            )?;
            access_acl.write(path, options)?;
        }
    }

    Ok(())
}

/// Write ACL entries to text.
///
/// Each ACL entry is printed on a separate line. The five fields are separated
/// by colons:
///
/// ```text
///   <allow>:<flags>:<kind>:<name>:<perms>
///
///   <allow> - one of "allow" or "deny"
///   <flags> - comma-separated list of flags
///   <kind>  - one of "user", "group", "other", "mask", "unknown"
///   <name>  - user/group name (or decimal id if not known)
///   <perms> - comma-separated list of permissions
/// ```
///
/// # Sample Output
///
/// ```text
/// allow::group:admin:read,write
/// ```
///
/// # Errors
///
/// Returns an [`io::Error`] on failure.
pub fn to_writer<W: io::Write>(mut writer: W, entries: &[AclEntry]) -> io::Result<()> {
    for entry in entries {
        writeln!(writer, "{}", entry)?;
    }

    Ok(())
}

/// Read ACL entries from text.
///
/// Each ACL entry is presented on a separate line. A comment begins with `#`
/// and proceeds to the end of the line. Within a field, leading or trailing
/// white space are ignored.
///
/// ```text
///   Three allowed forms:
///
///   <allow>:<flags>:<kind>:<name>:<perms>
///   <flags>:<kind>:<name>:<perms>
///   <kind>:<name>:<perms>
///
///   <allow> - one of "allow" or "deny"
///   <flags> - comma-separated list of flags
///   <kind>  - one of "user", "group", "other", "mask", "unknown"
///   <name>  - user/group name (decimal id accepted)
///   <perms> - comma-separated list of permissions
/// ```
///
/// Supported flags and permissions vary by platform.
///
/// Supported abbreviations:  d = default, r = read, w = write, x = execute,
/// u = user, g = group, o = other, m = mask
///
/// # Sample Input
///
/// ```text
/// allow::group:admin:read,write
/// g:admin:rw  # ignored
/// d:u:chip:rw
/// deny:file_inherit:user:chet:rwx
/// ```
///
/// # Errors
///
/// Returns an [`io::Error`] on failure.
pub fn from_reader<R: io::Read>(reader: R) -> io::Result<Vec<AclEntry>> {
    let mut result = Vec::<AclEntry>::new();
    let buf = io::BufReader::new(reader);

    for line_result in buf.lines() {
        let line = line_result?;

        let src_line = trim_comment(&line).trim();
        if !src_line.is_empty() {
            result.push(src_line.parse::<AclEntry>()?);
        }
    }

    Ok(result)
}

/// Return line with end of line comment removed.
fn trim_comment(line: &str) -> &str {
    line.find('#').map_or(line, |n| &line[0..n])
}
