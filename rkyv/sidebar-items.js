initSidebarItems({"derive":[["Archive","Derives `Archive` for the labeled type."],["Deserialize","Derives `Deserialize` for the labeled type."],["Serialize","Derives `Serialize` for the labeled type."]],"enum":[["Unreachable","An error that can never be produced"]],"macro":[["offset_of","Calculates the offset of the specified field from the start of the named struct."],["offset_of_tuple","Calculates the offset of the specified field from the start of the tuple."],["project_struct","Maps a mutable `MaybeUninit` struct reference to a mutable `MaybeUninit` field reference."],["project_tuple","Maps a mutable `MaybeUninit` tuple reference to a mutable `MaybeUninit` index reference."]],"mod":[["core_impl","[`Archive`] implementations for core types."],["de","Deserialization traits, deserializers, and adapters."],["ser","Serialization traits, serializers, and adapters."],["std_impl","[`Archive`] implementations for std types."],["util","Utilities for common archive operations."]],"struct":[["Infallible","A fallible type that cannot produce errors"],["RawRelPtr","An untyped pointer which resolves relative to its position in memory."],["RelPtr","A pointer which resolves to relative to its position in memory."]],"trait":[["Archive","A type that can be used without deserializing."],["ArchiveCopy","An [`Archive`] type that is a bitwise copy of itself and without additional processing."],["ArchivePointee","An archived type with associated metadata for its relative pointer."],["ArchiveUnsized","A counterpart of [`Archive`] that’s suitable for unsized types."],["Deserialize","Converts a type back from its archived form."],["DeserializeUnsized","A counterpart of [`Deserialize`] that’s suitable for unsized types."],["Fallible","Contains the error type for traits with methods that can fail"],["Serialize","Converts a type to its archived form."],["SerializeUnsized","A counterpart of [`Serialize`] that’s suitable for unsized types."]],"type":[["Archived","Alias for the archived version of some [`Archive`] type."],["ArchivedIsize","The type used for offsets in relative pointers."],["ArchivedMetadata","Alias for the archived metadata for some [`ArchiveUnsized`] type."],["ArchivedUsize","The type used for sizes in archived types."],["MetadataResolver","Alias for the metadata resolver for some [`ArchiveUnsized`] type."],["Resolver","Alias for the resolver for some [`Archive`] type."]]});