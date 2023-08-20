// // Result is an interface that provides the result of plugin execution
// pub trait Result {
//     // The highest CNI specification result version the result supports
//     // without having to convert
//     fn version(&self) -> String;

//     // Returns the result converted into the requested CNI specification
//     // result version, or an error if conversion failed
//     // fn getAsVersion(version: String) -> (Result, crate::cni::errors::Error);

//     // Prints the result in JSON format to stdout
//     fn print(&self) -> crate::cni::error::Error;

//     // Prints the result in JSON format to provided writer
//     // fn printTo(writer: io.Writer) -> error;
// }
