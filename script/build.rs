fn main() {
    let elf_path = sp1_build::build_program("../program");
    println!("ELF built at: {:?}", elf_path);
}
