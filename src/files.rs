
enum FileType {
    File,
    Directory,
}

struct File {
    name:   String,
    modif:  String,
    perms:  String,
    user:   String,
    group:  String,
    size:   u32,
    f_type: FileType,
}

struct FileSystem {
    root: File,
}
