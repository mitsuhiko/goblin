//! Miscellaneous constants used inside of and when constructing, Mach-o binaries
// Convienence constants for return values from dyld_get_sdk_version() and friends.
pub const DYLD_MACOSX_VERSION_10_4: u32 = 0x000A0400;
pub const DYLD_MACOSX_VERSION_10_5: u32 = 0x000A0500;
pub const DYLD_MACOSX_VERSION_10_6: u32 = 0x000A0600;
pub const DYLD_MACOSX_VERSION_10_7: u32 = 0x000A0700;
pub const DYLD_MACOSX_VERSION_10_8: u32 = 0x000A0800;
pub const DYLD_MACOSX_VERSION_10_9: u32 = 0x000A0900;
pub const DYLD_MACOSX_VERSION_10_10: u32 = 0x000A0A00;
pub const DYLD_MACOSX_VERSION_10_11: u32 = 0x000A0B00;
pub const DYLD_MACOSX_VERSION_10_12: u32 = 0x000A0C00;
pub const DYLD_MACOSX_VERSION_10_13: u32 = 0x000A0D00;

pub const DYLD_IOS_VERSION_2_0: u32 = 0x00020000;
pub const DYLD_IOS_VERSION_2_1: u32 = 0x00020100;
pub const DYLD_IOS_VERSION_2_2: u32 = 0x00020200;
pub const DYLD_IOS_VERSION_3_0: u32 = 0x00030000;
pub const DYLD_IOS_VERSION_3_1: u32 = 0x00030100;
pub const DYLD_IOS_VERSION_3_2: u32 = 0x00030200;
pub const DYLD_IOS_VERSION_4_0: u32 = 0x00040000;
pub const DYLD_IOS_VERSION_4_1: u32 = 0x00040100;
pub const DYLD_IOS_VERSION_4_2: u32 = 0x00040200;
pub const DYLD_IOS_VERSION_4_3: u32 = 0x00040300;
pub const DYLD_IOS_VERSION_5_0: u32 = 0x00050000;
pub const DYLD_IOS_VERSION_5_1: u32 = 0x00050100;
pub const DYLD_IOS_VERSION_6_0: u32 = 0x00060000;
pub const DYLD_IOS_VERSION_6_1: u32 = 0x00060100;
pub const DYLD_IOS_VERSION_7_0: u32 = 0x00070000;
pub const DYLD_IOS_VERSION_7_1: u32 = 0x00070100;
pub const DYLD_IOS_VERSION_8_0: u32 = 0x00080000;
pub const DYLD_IOS_VERSION_9_0: u32 = 0x00090000;
pub const DYLD_IOS_VERSION_10_0: u32 = 0x000A0000;
pub const DYLD_IOS_VERSION_11_0: u32 = 0x000B0000;

// Segment and Section Constants

// The flags field of a section structure is separated into two parts a section
// type and section attributes.  The section types are mutually exclusive (it
// can only have one type) but the section attributes are not (it may have more
// than one attribute).
/// 256 section types
pub const SECTION_TYPE: u32 = 0x000000ff;
///  24 section attributes
pub const SECTION_ATTRIBUTES: u32 = 0xffffff00;

// Constants for the type of a section
/// regular section
pub const S_REGULAR: u32 = 0x0;
/// zero fill on demand section
pub const S_ZEROFILL: u32 = 0x1;
/// section with only literal C strings
pub const S_CSTRING_LITERALS: u32 = 0x2;
/// section with only 4 byte literals
pub const S_4BYTE_LITERALS: u32 = 0x3;
/// section with only 8 byte literals
pub const S_8BYTE_LITERALS: u32 = 0x4;
/// section with only pointers to
pub const S_LITERAL_POINTERS: u32 = 0x5;

// literals
// For the two types of symbol pointers sections and the symbol stubs section
// they have indirect symbol table entries.  For each of the entries in the
// section the indirect symbol table entries, in corresponding order in the
// indirect symbol table, start at the index stored in the reserved1 field
// of the section structure.  Since the indirect symbol table entries
// correspond to the entries in the section the number of indirect symbol table
// entries is inferred from the size of the section divided by the size of the
// entries in the section.  For symbol pointers sections the size of the entries
// in the section is 4 bytes and for symbol stubs sections the byte size of the
// stubs is stored in the reserved2 field of the section structure.
/// section with only non-lazy symbol pointers
pub const S_NON_LAZY_SYMBOL_POINTERS: u32 = 0x6;
/// section with only lazy symbol pointers
pub const S_LAZY_SYMBOL_POINTERS: u32 = 0x7;
/// section with only symbol stubs, byte size of stub in the reserved2 field
pub const S_SYMBOL_STUBS: u32 = 0x8;
/// section with only function pointers for initialization
pub const S_MOD_INIT_FUNC_POINTERS: u32 = 0x9;
/// section with only function pointers for termination
pub const S_MOD_TERM_FUNC_POINTERS: u32 = 0xa;
/// section contains symbols that are to be coalesced
pub const S_COALESCED: u32 = 0xb;
/// zero fill on demand section that can be larger than 4 gigabytes)
pub const S_GB_ZEROFILL: u32 = 0xc;
/// section with only pairs of function pointers for interposing
pub const S_INTERPOSING: u32 = 0xd;
/// section with only 16 byte literals
pub const S_16BYTE_LITERALS: u32 = 0xe;
/// section contains DTrace Object Format
pub const S_DTRACE_DOF: u32 = 0xf;
/// section with only lazy symbol pointers to lazy loaded dylibs
pub const S_LAZY_DYLIB_SYMBOL_POINTERS: u32 = 0x10;

// Section types to support thread local variables
/// template of initial  values for TLVs
pub const S_THREAD_LOCAL_REGULAR: u32 = 0x11;
/// template of initial  values for TLVs
pub const S_THREAD_LOCAL_ZEROFILL: u32 = 0x12;
/// TLV descriptors
pub const S_THREAD_LOCAL_VARIABLES: u32 = 0x13;
/// pointers to TLV  descriptors
pub const S_THREAD_LOCAL_VARIABLE_POINTERS: u32 = 0x14;
/// functions to call to initialize TLV values
pub const S_THREAD_LOCAL_INIT_FUNCTION_POINTERS: u32 = 0x15;

// Constants for the section attributes part of the flags field of a section
// structure.
/// User setable attributes
pub const SECTION_ATTRIBUTES_USR: u32 = 0xff000000;
/// section contains only true machine instructions
pub const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x80000000;
/// section contains coalesced symbols that are not to be in a ranlib table of contents
pub const S_ATTR_NO_TOC: u32 = 0x40000000;
/// ok to strip static symbols in this section in files with the MH_DYLDLINK flag
pub const S_ATTR_STRIP_STATIC_SYMS: u32 = 0x20000000;
/// no dead stripping
pub const S_ATTR_NO_DEAD_STRIP: u32 = 0x10000000;
/// blocks are live if they reference live blocks
pub const S_ATTR_LIVE_SUPPORT: u32 = 0x08000000;
/// Used with i386 code stubs written on by dyld
pub const S_ATTR_SELF_MODIFYING_CODE: u32 = 0x04000000;

// If a segment contains any sections marked with S_ATTR_DEBUG then all
// sections in that segment must have this attribute.  No section other than
// a section marked with this attribute may reference the contents of this
// section.  A section with this attribute may contain no symbols and must have
// a section type S_REGULAR.  The static linker will not copy section contents
// from sections with this attribute into its output file.  These sections
// generally contain DWARF debugging info.
/// debug section
pub const S_ATTR_DEBUG: u32 = 0x02000000;
/// system setable attributes
pub const SECTION_ATTRIBUTES_SYS: u32 = 0x00ffff00;
/// section contains some machine instructions
pub const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x00000400;
/// section has external relocation entries
pub const S_ATTR_EXT_RELOC: u32 = 0x00000200;
/// section has local relocation entries
pub const S_ATTR_LOC_RELOC: u32 = 0x00000100;

// The names of segments and sections in them are mostly meaningless to the
// link-editor.  But there are few things to support traditional UNIX
// executables that require the link-editor and assembler to use some names
// agreed upon by convention.
// The initial protection of the "__TEXT" segment has write protection turned
// off (not writeable).
// The link-editor will allocate common symbols at the end of the "__common"
// section in the "__DATA" segment.  It will create the section and segment
// if needed.

// The currently known segment names and the section names in those segments
/// the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files
pub const SEG_PAGEZERO: &'static str = "__PAGEZERO";
/// the tradition UNIX text segment
pub const SEG_TEXT: &'static str = "__TEXT";
/// the real text part of the text section no headers, and no padding
pub const SECT_TEXT: &'static str = "__text";
/// the fvmlib initialization section
pub const SECT_FVMLIB_INIT0: &'static str = "__fvmlib_init0";
/// the section following the fvmlib initialization section
pub const SECT_FVMLIB_INIT1: &'static str = "__fvmlib_init1";
/// the tradition UNIX data segment
pub const SEG_DATA: &'static str = "__DATA";
/// the real initialized data section no padding, no bss overlap
pub const SECT_DATA: &'static str = "__data";
/// the real uninitialized data sectionno padding
pub const SECT_BSS: &'static str = "__bss";
/// the section common symbols are allocated in by the link editor
pub const SECT_COMMON: &'static str = "__common";
/// objective-C runtime segment
pub const SEG_OBJC: &'static str = "__OBJC";
/// symbol table
pub const SECT_OBJC_SYMBOLS: &'static str = "__symbol_table";
/// module information
pub const SECT_OBJC_MODULES: &'static str = "__module_info";
/// string table
pub const SECT_OBJC_STRINGS: &'static str = "__selector_strs";
/// string table
pub const SECT_OBJC_REFS: &'static str = "__selector_refs";
/// the icon segment
pub const SEG_ICON: &'static str = "__ICON";
/// the icon headers
pub const SECT_ICON_HEADER: &'static str = "__header";
/// the icons in tiff format
pub const SECT_ICON_TIFF: &'static str = "__tiff";
/// the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only
pub const SEG_LINKEDIT: &'static str = "__LINKEDIT";
/// the unix stack segment
pub const SEG_UNIXSTACK: &'static str = "__UNIXSTACK";
/// the segment for the self (dyld) modifing code stubs that has read, write and execute permissions
pub const SEG_IMPORT: &'static str = "__IMPORT";

pub mod cputype {

    pub type CpuType = u32;

    /// the mask for CPU feature flags
    pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;

    pub const CPU_ARCH_MASK: CpuType = 0xff000000;
    pub const CPU_ARCH_ABI64: CpuType = 0x01000000;
    pub const CPU_TYPE_X86: CpuType = 7;
    pub const CPU_TYPE_ARM: CpuType = 12;
    pub const CPU_TYPE_X86_64: CpuType = CPU_TYPE_X86 | CPU_ARCH_ABI64;
    pub const CPU_TYPE_ARM64: CpuType = CPU_TYPE_ARM | CPU_ARCH_ABI64;

    #[inline(always)]
    pub fn cpu_type_to_str(cputype: CpuType) -> &'static str {
        match cputype {
            CPU_TYPE_ARM64 => "ARM64",
            CPU_TYPE_X86_64 => "x86_64",
            CPU_TYPE_ARM => "ARM",
            CPU_TYPE_X86 => "x86",
            _ => "UNIMPLEMENTED CPUTYPE",
        }
    }
}
