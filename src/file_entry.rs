pub mod zip_file_entry;

// TODO: Create a struct that represents a zip file entry that can be converted straight into a byte slice and that can also spit out its size
// TODO: This struct will contain the header as well as the data slice
// TODO: Decide on return types in terms of copying data vs transferring ownership etc
// The header will be a transferring of ownership because its made up of various variables
// The body may have to be a copy anyway