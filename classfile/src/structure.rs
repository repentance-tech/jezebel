use bitflags::bitflags;
use core::fmt;

bitflags! {
    pub struct AccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_string = match &self.major {
            45 => "JDK 1.1",
            46 => "JDK 1.2",
            47 => "JDK 1.3",
            48 => "JDK 1.4",
            49 => "SE 5",
            50 => "SE 6",
            51 => "SE 7",
            52 => "SE 8",
            53 => "SE 9",
            54 => "SE 10",
            55 => "SE 11",
            56 => "SE 12",
            57 => "SE 13",
            58 => "SE 14",
            59 => "SE 15",
            60 => "SE 16",
            61 => "SE 17",
            62 => "SE 18",
            63 => "SE 19",
            _ => "unknown",
        };

        write!(
            f,
            "major: {} ({}) minor: {}",
            &self.major, version_string, &self.minor
        )
    }
}

#[derive(Debug)]
pub enum ConstantPoolInfo {
    Class(usize),           // name index
    FieldRef(usize, usize), // class index, name_and_type index
    MethodRef(usize, usize),
    InterfaceMethodRef(usize, usize),
    String(usize), // index to utf8 value
    Integer(u32),
    Float(u32),
    Long(u32, u32),
    Double(u32, u32),
    NameAndType(usize, usize),
    Utf(u16, Vec<u8>),
    MethodHandle(u8, usize),
    MethodType(usize),
    Dynamic(usize, usize),
    InvokeDynamic(usize, usize),
    Module(usize),
    Package(usize),
}

pub struct ClassFile {
    pub version: Version,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: AccessFlags,
    pub this_class: usize,
    pub super_class: usize,
    pub interfaces: Vec<usize>,
}
