extern crate cc;

fn main() {
	cc::Build::new()
        .file("src/c_file.c")
        .compile("libdoublec.a");
   	cc::Build::new()
        .file("src/cpp_file.cpp")
        .cpp(true)
        .compile("libdoublecpp.a");
}
