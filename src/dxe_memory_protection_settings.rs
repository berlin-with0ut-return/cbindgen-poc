use bitflags::bitflags;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl EfiGuid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        EfiGuid {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

pub const HOB_DXE_MEMORY_PROTECTION_SETTINGS_GUID: EfiGuid = EfiGuid {
    data1: 0x9ABFD639,
    data2: 0xD1D0,
    data3: 0x4EFF,
    data4: [0xBD, 0xB6, 0x7E, 0xC4, 0x19, 0x0D, 0x17, 0xD5],
};

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeNullDetectionPolicy {
    pub data: u8,
    pub fields: NullDetectionFields,
}

// todo_sherry: these aren't bitfields like they would be in C, and thus this doesn't work
// the way you'd want it to
bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NullDetectionFields: u8 {
        const UEFI_NULL_DETECTION = 0x01;
        const DISABLE_END_OF_DXE = 0x02;
        const DISABLE_READY_TO_BOOT = 0x04;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeHeapGuardPolicy {
    pub data: u8,
    pub fields: HeapGuardFields,
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HeapGuardFields: u8 {
        const UEFI_PAGE_GUARD = 0x01;
        const UEFI_POOL_GUARD = 0x02;
        const DIRECTION = 0x04;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeHeapGuardMemoryTypes {
    pub data: u32,
    pub fields: HeapGuardMemoryFields,
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HeapGuardMemoryFields: u32 {
        const EFI_RESERVED_MEMORY_TYPE = 0x00000001;
        const EFI_LOADER_CODE = 0x00000002;
        const EFI_LOADER_DATA = 0x00000004;
        const EFI_BOOT_SERVICES_CODE = 0x00000008;
        const EFI_BOOT_SERVICES_DATA = 0x00000010;
        const EFI_RUNTIME_SERVICES_CODE = 0x00000020;
        const EFI_RUNTIME_SERVICES_DATA = 0x00000040;
        const EFI_CONVENTIONAL_MEMORY = 0x00000080;
        const EFI_UNUSABLE_MEMORY = 0x00000100;
        const EFI_ACPI_RECLAIM_MEMORY = 0x00000200;
        const EFI_ACPI_MEMORY_NVS = 0x00000400;
        const EFI_MEMORY_MAPPED_IO = 0x00000800;
        const EFI_MEMORY_MAPPED_IO_PORT_SPACE = 0x00001000;
        const EFI_PAL_CODE = 0x00002000;
        const EFI_PERSISTENT_MEMORY = 0x00004000;
        const EFI_UNACCEPTED_MEMORY_TYPE = 0x00008000;
        const OEM_RESERVED = 0x00010000;
        const OS_RESERVED = 0x00020000;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeImageProtectionPolicy {
    pub data: u8,
    pub fields: ImageProtectionFields,
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ImageProtectionFields: u8 {
        const PROTECT_IMAGE_FROM_UNKNOWN = 0x01;
        const PROTECT_IMAGE_FROM_FV = 0x02;
        const RAISE_ERROR_IF_PROTECTION_FAILS = 0x04;
        const BLOCK_IMAGES_WITHOUT_NX_FLAG = 0x08;
    }
}

pub type DxeMemoryProtectionSettingsVersion = u8;

pub const DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION: DxeMemoryProtectionSettingsVersion = 7;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DxeMemoryProtectionSettings {
    pub struct_version: DxeMemoryProtectionSettingsVersion,
    pub cpu_stack_guard: bool,
    pub free_memory_read_protected: bool,
    pub null_pointer_detection_policy: DxeNullDetectionPolicy,
    pub heap_guard_policy: DxeHeapGuardPolicy,
    pub image_protection_policy: DxeImageProtectionPolicy,
    pub heap_guard_pool_type: DxeHeapGuardMemoryTypes,
    pub heap_guard_page_type: DxeHeapGuardMemoryTypes,
    pub nx_protection_policy: DxeHeapGuardMemoryTypes,
    pub stack_cookies: bool,
    pub install_memory_attribute_protocol: bool,
}

// todo_sherry: these really could be consts but bitflags isn't...
#[no_mangle]
pub extern "C" fn debug_settings() -> DxeMemoryProtectionSettings {
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: true,
        free_memory_read_protected: true,
        null_pointer_detection_policy: DxeNullDetectionPolicy {
            fields: NullDetectionFields::UEFI_NULL_DETECTION,
        },
        heap_guard_policy: DxeHeapGuardPolicy {
            fields: HeapGuardFields::UEFI_PAGE_GUARD | HeapGuardFields::UEFI_POOL_GUARD,
        },
        image_protection_policy: DxeImageProtectionPolicy {
            fields: ImageProtectionFields::PROTECT_IMAGE_FROM_UNKNOWN
                | ImageProtectionFields::PROTECT_IMAGE_FROM_FV
                | ImageProtectionFields::RAISE_ERROR_IF_PROTECTION_FAILS
                | ImageProtectionFields::BLOCK_IMAGES_WITHOUT_NX_FLAG,
        },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_RESERVED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_LOADER_CODE
                | HeapGuardMemoryFields::EFI_LOADER_DATA
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_UNUSABLE_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_RECLAIM_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_MEMORY_NVS
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO_PORT_SPACE
                | HeapGuardMemoryFields::EFI_PAL_CODE
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE
                | HeapGuardMemoryFields::OEM_RESERVED
                | HeapGuardMemoryFields::OS_RESERVED,
        },
        heap_guard_page_type: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_RESERVED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_LOADER_CODE
                | HeapGuardMemoryFields::EFI_LOADER_DATA
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_UNUSABLE_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_RECLAIM_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_MEMORY_NVS
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO_PORT_SPACE
                | HeapGuardMemoryFields::EFI_PAL_CODE
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE
                | HeapGuardMemoryFields::OEM_RESERVED
                | HeapGuardMemoryFields::OS_RESERVED,
        },
        nx_protection_policy: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_RESERVED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_LOADER_CODE
                | HeapGuardMemoryFields::EFI_LOADER_DATA
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_CONVENTIONAL_MEMORY
                | HeapGuardMemoryFields::EFI_UNUSABLE_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_RECLAIM_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_MEMORY_NVS
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO_PORT_SPACE
                | HeapGuardMemoryFields::EFI_PAL_CODE
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE
                | HeapGuardMemoryFields::OEM_RESERVED
                | HeapGuardMemoryFields::OS_RESERVED,
        },
        stack_cookies: true,
        install_memory_attribute_protocol: true,
    }
}

#[no_mangle]
pub extern "C" fn ship_mode_settings() -> DxeMemoryProtectionSettings {
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: true,
        free_memory_read_protected: true,
        null_pointer_detection_policy: DxeNullDetectionPolicy {
            fields: NullDetectionFields::UEFI_NULL_DETECTION,
        },
        heap_guard_policy: DxeHeapGuardPolicy {
            fields: HeapGuardFields::UEFI_PAGE_GUARD,
        },
        image_protection_policy: DxeImageProtectionPolicy {
            fields: ImageProtectionFields::PROTECT_IMAGE_FROM_FV,
        },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes { data: 0 },
        heap_guard_page_type: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA,
        },
        nx_protection_policy: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_RESERVED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_LOADER_CODE
                | HeapGuardMemoryFields::EFI_LOADER_DATA
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_CODE
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_CONVENTIONAL_MEMORY
                | HeapGuardMemoryFields::EFI_UNUSABLE_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_RECLAIM_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_MEMORY_NVS
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO_PORT_SPACE
                | HeapGuardMemoryFields::EFI_PAL_CODE
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_PERSISTENT_MEMORY
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE,
        },
        stack_cookies: true,
        install_memory_attribute_protocol: true,
    }
}

#[no_mangle]
pub extern "C" fn ship_mode_no_page_guard_settings() -> DxeMemoryProtectionSettings {
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: true,
        free_memory_read_protected: true,
        null_pointer_detection_policy: DxeNullDetectionPolicy {
            fields: NullDetectionFields::UEFI_NULL_DETECTION,
        },
        heap_guard_policy: DxeHeapGuardPolicy { data: 0 },
        image_protection_policy: DxeImageProtectionPolicy {
            fields: ImageProtectionFields::PROTECT_IMAGE_FROM_FV,
        },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes { data: 0 },
        heap_guard_page_type: DxeHeapGuardMemoryTypes { data: 0 },
        nx_protection_policy: DxeHeapGuardMemoryTypes {
            fields: HeapGuardMemoryFields::EFI_RESERVED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_LOADER_DATA
                | HeapGuardMemoryFields::EFI_BOOT_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_RUNTIME_SERVICES_DATA
                | HeapGuardMemoryFields::EFI_CONVENTIONAL_MEMORY
                | HeapGuardMemoryFields::EFI_UNUSABLE_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_RECLAIM_MEMORY
                | HeapGuardMemoryFields::EFI_ACPI_MEMORY_NVS
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO
                | HeapGuardMemoryFields::EFI_MEMORY_MAPPED_IO_PORT_SPACE
                | HeapGuardMemoryFields::EFI_PAL_CODE
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE
                | HeapGuardMemoryFields::EFI_PERSISTENT_MEMORY
                | HeapGuardMemoryFields::EFI_UNACCEPTED_MEMORY_TYPE,
        },
        stack_cookies: true,
        install_memory_attribute_protocol: true,
    }
}

#[no_mangle]
pub extern "C" fn protection_off_settings() -> DxeMemoryProtectionSettings {
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: false,
        free_memory_read_protected: false,
        null_pointer_detection_policy: DxeNullDetectionPolicy { data: 0 },
        heap_guard_policy: DxeHeapGuardPolicy { data: 0 },
        image_protection_policy: DxeImageProtectionPolicy { data: 0 },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes { data: 0 },
        heap_guard_page_type: DxeHeapGuardMemoryTypes { data: 0 },
        nx_protection_policy: DxeHeapGuardMemoryTypes { data: 0 },
        stack_cookies: false,
        install_memory_attribute_protocol: false,
    }
}

pub const HEAP_GUARD_ALIGNED_TO_TAIL: u8 = 0;
pub const HEAP_GUARD_ALIGNED_TO_HEAD: u8 = 1;
