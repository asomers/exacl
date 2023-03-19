var searchIndex = JSON.parse('{\
"exacl":{"doc":"exacl","t":"SSDEDSSSSSSSNSDNSSNSSSNSDSSSSSSSNNSSSSSLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFFFLLLLFLLLLLLLLLLLLLLLLLLLLLLLMMLLLLLLLLMLLLLLLFLLLLLLLLLLLLLLFLLLLFLLLLLLLLLLLLLLLLLLLLL","n":["ACCESS_ACL","APPEND","AclEntry","AclEntryKind","AclOption","CHOWN","DEFAULT","DEFAULT_ACL","DELETE","DELETE_CHILD","DIRECTORY_INHERIT","EXECUTE","Everyone","FILE_INHERIT","Flag","Group","INHERITED","LIMIT_INHERIT","Mask","NFS4_SPECIFIC","NFS4_SPECIFIC","ONLY_INHERIT","Other","POSIX_SPECIFIC","Perm","READ","READATTR","READEXTATTR","READSECURITY","READ_DATA","SYMLINK_ACL","SYNC","Unknown","User","WRITE","WRITEATTR","WRITEEXTATTR","WRITESECURITY","WRITE_DATA","all","all","all","allow","allow_group","allow_mask","allow_other","allow_user","bitand","bitand","bitand","bitand_assign","bitand_assign","bitand_assign","bitor","bitor","bitor","bitor_assign","bitor_assign","bitor_assign","bits","bits","bits","bitxor","bitxor","bitxor","bitxor_assign","bitxor_assign","bitxor_assign","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","cmp","cmp","cmp","complement","complement","complement","contains","contains","contains","default","default","default","deny_group","deny_user","difference","difference","difference","empty","empty","empty","eq","eq","eq","eq","eq","extend","extend","extend","flags","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from_bits","from_bits","from_bits","from_bits_truncate","from_bits_truncate","from_bits_truncate","from_bits_unchecked","from_bits_unchecked","from_bits_unchecked","from_iter","from_iter","from_iter","from_mode","from_reader","from_str","from_str","from_str","from_str","from_str","getfacl","hash","hash","hash","insert","insert","insert","intersection","intersection","intersection","intersects","intersects","intersects","into","into","into","into","into","is_all","is_all","is_all","is_empty","is_empty","is_empty","kind","name","not","not","not","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","perms","remove","remove","remove","set","set","set","setfacl","sub","sub","sub","sub_assign","sub_assign","sub_assign","symmetric_difference","symmetric_difference","symmetric_difference","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","to_string","to_writer","toggle","toggle","toggle","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","union","union","union"],"q":[[0,"exacl"]],"d":["Get/set the access ACL only (Linux and FreeBSD only).","APPEND_DATA permission for a file. Same as …","ACL entry with allow/deny semantics.","Kind of ACL entry (User, Group, Mask, Other, or Unknown).","Controls how ACL’s are accessed.","CHANGE_OWNER permission for a file or directory.","Specifies a default ACL entry on Linux.","Get/set the default ACL only (Linux and FreeBSD only).","DELETE permission for a file.","DELETE_CHILD permission for a directory.","Inherit to directories.","EXECUTE permission for a file. Same as SEARCH permission …","Entry represents a NFS “everyone” entry.","Inherit to files.","Represents ACL entry inheritance flags.","Entry represents a group.","ACL entry was inherited.","Clear the DIRECTORY_INHERIT flag in the ACL entry that is …","Entry represents a Posix.1e “mask” entry.","NFSv4 Specific Flags on FreeBSD.","All NFSv4 specific permissions.","Don’t consider this entry when processing the ACL. Just …","Entry represents a Posix.1e “other” entry.","Posix specific permissions.","Represents file access permissions.","READ_DATA permission for a file. Same as LIST_DIRECTORY …","READ_ATTRIBUTES permission for file or directory.","READ_EXTATTRIBUTES permission for a file or directory.","READ_SECURITY permission for a file or directory.","NFSv4 READ_DATA permission.","Get/set the ACL of the symlink itself (macOS only).","SYNCHRONIZE permission (unsupported).","Entry represents a possibly corrupt ACL entry, caused by …","Entry represents a user.","WRITE_DATA permission for a file. Same as ADD_FILE …","WRITE_ATTRIBUTES permission for a file or directory.","WRITE_EXTATTRIBUTES permission for a file or directory.","WRITE_SECURITY permission for a file or directory.","NFSv4 WRITE_DATA permission.","Returns the set containing all flags.","Returns the set containing all flags.","Returns the set containing all flags.","True if entry is allowed; false means deny. Linux only …","Construct an ALLOW access control entry for a group.","Construct an ALLOW access control entry for mask.","Construct an ALLOW access control entry for other.","Construct an ALLOW access control entry for a user.","Returns the intersection between the two sets of flags.","Returns the intersection between the two sets of flags.","Returns the intersection between the two sets of flags.","Disables all flags disabled in the set.","Disables all flags disabled in the set.","Disables all flags disabled in the set.","Returns the union of the two sets of flags.","Returns the union of the two sets of flags.","Returns the union of the two sets of flags.","Adds the set of flags.","Adds the set of flags.","Adds the set of flags.","Returns the raw value of the flags currently stored.","Returns the raw value of the flags currently stored.","Returns the raw value of the flags currently stored.","Returns the left flags, but with all the right flags …","Returns the left flags, but with all the right flags …","Returns the left flags, but with all the right flags …","Toggles the set of flags.","Toggles the set of flags.","Toggles the set of flags.","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the complement of this set of flags.","Returns the complement of this set of flags.","Returns the complement of this set of flags.","Returns <code>true</code> if all of the flags in <code>other</code> are contained …","Returns <code>true</code> if all of the flags in <code>other</code> are contained …","Returns <code>true</code> if all of the flags in <code>other</code> are contained …","","","","Construct a DENY access control entry for a group.","Construct a DENY access control entry for a user.","Returns the difference between the flags in <code>self</code> and <code>other</code>.","Returns the difference between the flags in <code>self</code> and <code>other</code>.","Returns the difference between the flags in <code>self</code> and <code>other</code>.","Returns an empty set of flags.","Returns an empty set of flags.","Returns an empty set of flags.","","","","","","","","","Flags indicating whether an entry is inherited, etc.","","","","","","","","Format an <code>AclEntry</code> 5-tuple: ::::","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Convert from underlying bit representation, unless that …","Convert from underlying bit representation, unless that …","Convert from underlying bit representation, unless that …","Convert from underlying bit representation, dropping any …","Convert from underlying bit representation, dropping any …","Convert from underlying bit representation, dropping any …","Convert from underlying bit representation, preserving all …","Convert from underlying bit representation, preserving all …","Convert from underlying bit representation, preserving all …","","","","Construct a minimal ACL from the traditional <code>mode</code> …","Read ACL entries from text.","Read ACL entries from text.","","","","","Get access control list (ACL) for a file or directory.","","","","Inserts the specified flags in-place.","Inserts the specified flags in-place.","Inserts the specified flags in-place.","Returns the intersection between the flags in <code>self</code> and …","Returns the intersection between the flags in <code>self</code> and …","Returns the intersection between the flags in <code>self</code> and …","Returns <code>true</code> if there are flags common to both <code>self</code> and …","Returns <code>true</code> if there are flags common to both <code>self</code> and …","Returns <code>true</code> if there are flags common to both <code>self</code> and …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if all flags are currently set.","Returns <code>true</code> if all flags are currently set.","Returns <code>true</code> if all flags are currently set.","Returns <code>true</code> if no flags are currently stored.","Returns <code>true</code> if no flags are currently stored.","Returns <code>true</code> if no flags are currently stored.","Kind of entry (User, Group, Other, Mask, Everyone, or …","Name of the principal being given access. You can use a …","Returns the complement of this set of flags.","Returns the complement of this set of flags.","Returns the complement of this set of flags.","","","","","","Permission bits for the entry.","Removes the specified flags in-place.","Removes the specified flags in-place.","Removes the specified flags in-place.","Inserts or removes the specified flags depending on the …","Inserts or removes the specified flags depending on the …","Inserts or removes the specified flags depending on the …","Set access control list (ACL) for specified files and …","Returns the set difference of the two sets of flags.","Returns the set difference of the two sets of flags.","Returns the set difference of the two sets of flags.","Disables all flags enabled in the set.","Disables all flags enabled in the set.","Disables all flags enabled in the set.","Returns the symmetric difference between the flags in <code>self</code> …","Returns the symmetric difference between the flags in <code>self</code> …","Returns the symmetric difference between the flags in <code>self</code> …","","","","","","Write ACL entries to text.","","","","","Write ACL entries to text.","Toggles the specified flags in-place.","Toggles the specified flags in-place.","Toggles the specified flags in-place.","","","","","","","","","","","","","","","","Returns the union of between the flags in <code>self</code> and <code>other</code>.","Returns the union of between the flags in <code>self</code> and <code>other</code>.","Returns the union of between the flags in <code>self</code> and <code>other</code>."],"i":[1,3,0,0,0,3,2,1,3,3,2,3,10,2,0,10,2,2,10,2,3,2,10,3,0,3,3,3,3,3,1,3,10,10,3,3,3,3,3,1,2,3,7,7,7,7,7,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,10,7,2,3,1,10,7,2,3,1,10,7,2,3,1,10,7,2,3,1,10,7,2,3,1,2,3,1,2,3,1,2,3,7,7,1,2,3,1,2,3,1,10,7,2,3,1,2,3,7,1,1,1,1,1,10,10,7,7,2,2,2,2,2,2,3,3,3,3,3,3,1,10,7,2,3,1,2,3,1,2,3,1,2,3,1,2,3,0,0,0,10,7,2,3,0,1,2,3,1,2,3,1,2,3,1,2,3,1,10,7,2,3,1,2,3,1,2,3,7,7,1,2,3,1,10,7,2,3,7,1,2,3,1,2,3,0,1,2,3,1,2,3,1,2,3,1,10,7,2,3,0,10,7,2,3,0,1,2,3,1,10,7,2,3,1,10,7,2,3,1,10,7,2,3,1,2,3],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],1],[[],2],[[],3],0,[[4,3,[6,[[5,[2]]]]],7],[[3,[6,[[5,[2]]]]],7],[[3,[6,[[5,[2]]]]],7],[[4,3,[6,[[5,[2]]]]],7],[[1,1],1],[[2,2],2],[[3,3],3],[[1,1]],[[2,2]],[[3,3]],[[1,1],1],[[2,2],2],[[3,3],3],[[1,1]],[[2,2]],[[3,3]],[1,8],[2,8],[3,9],[[1,1],1],[[2,2],2],[[3,3],3],[[1,1]],[[2,2]],[[3,3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[10,10],[7,7],[2,2],[3,3],[[]],[[]],[[]],[[]],[[]],[[1,1],11],[[10,10],11],[[7,7],11],[[2,2],11],[[3,3],11],[1,1],[2,2],[3,3],[[1,1],12],[[2,2],12],[[3,3],12],[[],1],[[],2],[[],3],[[4,3,[6,[[5,[2]]]]],7],[[4,3,[6,[[5,[2]]]]],7],[[1,1],1],[[2,2],2],[[3,3],3],[[],1],[[],2],[[],3],[[1,1],12],[[10,10],12],[[7,7],12],[[2,2],12],[[3,3],12],[[1,13]],[[2,13]],[[3,13]],0,[[1,14],15],[[1,14],15],[[1,14],15],[[1,14],15],[[1,14],15],[[10,14],15],[[10,14],15],[[7,14],15],[[7,14],15],[[2,14],15],[[2,14],15],[[2,14],15],[[2,14],15],[[2,14],15],[[2,14],15],[[3,14],15],[[3,14],15],[[3,14],15],[[3,14],15],[[3,14],15],[[3,14],15],[[]],[[]],[[]],[[]],[[]],[8,[[5,[1]]]],[8,[[5,[2]]]],[9,[[5,[3]]]],[8,1],[8,2],[9,3],[8,1],[8,2],[9,3],[13,1],[13,2],[13,3],[8,[[16,[7]]]],[17,[[18,[[16,[7]]]]]],[4,[[18,[[16,[7]]]]]],[4,[[19,[10]]]],[4,[[19,[7]]]],[4,[[19,[2]]]],[4,[[19,[3]]]],[[[21,[20]],[6,[[5,[1]]]]],[[18,[[16,[7]]]]]],[[1,22]],[[2,22]],[[3,22]],[[1,1]],[[2,2]],[[3,3]],[[1,1],1],[[2,2],2],[[3,3],3],[[1,1],12],[[2,2],12],[[3,3],12],[[]],[[]],[[]],[[]],[[]],[1,12],[2,12],[3,12],[1,12],[2,12],[3,12],0,0,[1,1],[2,2],[3,3],[[1,1],[[5,[11]]]],[[10,10],[[5,[11]]]],[[7,7],[[5,[11]]]],[[2,2],[[5,[11]]]],[[3,3],[[5,[11]]]],0,[[1,1]],[[2,2]],[[3,3]],[[1,1,12]],[[2,2,12]],[[3,3,12]],[[[6,[[5,[1]]]]],18],[[1,1],1],[[2,2],2],[[3,3],3],[[1,1]],[[2,2]],[[3,3]],[[1,1],1],[[2,2],2],[[3,3],3],[[]],[[]],[[]],[[]],[[]],[[],[[18,[23]]]],[[],23],[[],23],[[],23],[[],23],[24,18],[[1,1]],[[2,2]],[[3,3]],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],25],[[],25],[[],25],[[],25],[[],25],[[1,1],1],[[2,2],2],[[3,3],3]],"c":[],"p":[[3,"AclOption"],[3,"Flag"],[3,"Perm"],[15,"str"],[4,"Option"],[8,"Into"],[3,"AclEntry"],[15,"u32"],[6,"c_uint"],[4,"AclEntryKind"],[4,"Ordering"],[15,"bool"],[8,"IntoIterator"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[8,"Read"],[6,"Result"],[4,"Result"],[3,"Path"],[8,"AsRef"],[8,"Hasher"],[3,"String"],[8,"Write"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
