use crate::inttypes::{UINT32, UINT8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Guid([UINT8; 16]);

impl Guid {
    pub const fn new(data: [UINT8; 16]) -> Self {
        Guid(data)
    }
}

#[no_mangle]
pub static HOB_DXE_MEMORY_PROTECTION_SETTINGS_GUID: Guid = Guid::new([
    0x9A, 0xBF, 0xD6, 0x39, 0xD1, 0xD0, 0x4E, 0xFF, 0xBD, 0xB6, 0x7E, 0xC4, 0x19, 0x0D, 0x17, 0xD5,
]);

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeNullDetectionPolicy {
    pub data: UINT8,
    pub fields: NullDetectionFields,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg(feature = "cbindgen")]
pub struct NullDetectionFields {
    /// cbindgen:bitfield=1
    pub uefi_null_detection: UINT8,
    /// cbindgen:bitfield=1
    pub disable_end_of_dxe: UINT8,
    /// cbindgen:bitfield=1
    pub disable_ready_to_boot: UINT8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeHeapGuardPolicy {
    pub data: UINT8,
    pub fields: HeapGuardFields,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg(feature = "cbindgen")]
pub struct HeapGuardFields {
    /// cbindgen:bitfield=1
    pub uefi_page_guard: UINT8,
    /// cbindgen:bitfield=1
    pub uefi_pool_guard: UINT8,
    /// cbindgen:bitfield=1
    pub direction: UINT8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeHeapGuardMemoryTypes {
    pub data: UINT32,
    pub fields: HeapGuardMemoryFields,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg(feature = "cbindgen")]
pub struct HeapGuardMemoryFields {
    /// cbindgen:bitfield=1
    pub efi_reserved_memory_type: UINT8,
    /// cbindgen:bitfield=1
    pub efi_loader_code: UINT8,
    /// cbindgen:bitfield=1
    pub efi_loader_data: UINT8,
    /// cbindgen:bitfield=1
    pub efi_boot_services_code: UINT8,
    /// cbindgen:bitfield=1
    pub efi_boot_services_data: UINT8,
    /// cbindgen:bitfield=1
    pub efi_runtime_services_code: UINT8,
    /// cbindgen:bitfield=1
    pub efi_runtime_services_data: UINT8,
    /// cbindgen:bitfield=1
    pub efi_conventional_memory: UINT8,
    /// cbindgen:bitfield=1
    pub efi_unusable_memory: UINT8,
    /// cbindgen:bitfield=1
    pub efi_acpi_reclaim_memory: UINT8,
    /// cbindgen:bitfield=1
    pub efi_acpi_memory_nvs: UINT8,
    /// cbindgen:bitfield=1
    pub efi_memory_mapped_io: UINT8,
    /// cbindgen:bitfield=1
    pub efi_memory_mapped_io_port_space: UINT8,
    /// cbindgen:bitfield=1
    pub efi_pal_code: UINT8,
    /// cbindgen:bitfield=1
    pub efi_persistent_memory: UINT8,
    /// cbindgen:bitfield=1
    pub efi_unaccepted_memory_type: UINT8,
    /// cbindgen:bitfield=1
    pub oem_reserved: UINT8,
    /// cbindgen:bitfield=1
    pub os_reserved: UINT8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DxeImageProtectionPolicy {
    pub data: UINT8,
    pub fields: ImageProtectionFields,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg(feature = "cbindgen")]
pub struct ImageProtectionFields {
    /// cbindgen:bitfield=1
    pub protect_image_from_unknown: UINT8,
    /// cbindgen:bitfield=1
    pub protect_image_from_fv: UINT8,
    /// cbindgen:bitfield=1
    pub raise_error_if_protection_fails: UINT8,
    /// cbindgen:bitfield=1
    pub block_images_without_nx_flag: UINT8,
}

pub type DxeMemoryProtectionSettingsVersion = UINT8;

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

pub const HEAP_GUARD_ALIGNED_TO_TAIL: UINT8 = 0;
pub const HEAP_GUARD_ALIGNED_TO_HEAD: UINT8 = 1;

// we need an extern C function to force cbindgen to generate headers
#[no_mangle]
#[cfg(feature = "cbindgen")]
pub extern "C" fn ship_mode_no_page_guard_settings() -> DxeMemoryProtectionSettings {
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: true,
        free_memory_read_protected: true,
        null_pointer_detection_policy: DxeNullDetectionPolicy {
            fields: NullDetectionFields {
                uefi_null_detection: 1,
                ..Default::default()
            },
        },
        heap_guard_policy: DxeHeapGuardPolicy { data: 0 },
        image_protection_policy: DxeImageProtectionPolicy { data: 0 },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes { data: 0 },
        heap_guard_page_type: DxeHeapGuardMemoryTypes { data: 0 },
        nx_protection_policy: DxeHeapGuardMemoryTypes { data: 0 },
        stack_cookies: true,
        install_memory_attribute_protocol: true,
    }
}

#[no_mangle]
#[cfg(feature = "cbindgen")]
pub static DXE_MEMORY_PROTECTION_SETTINGS_SHIP_MODE_NO_PAGE_GUARDS: DxeMemoryProtectionSettings =
    DxeMemoryProtectionSettings {
        struct_version: DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION,
        cpu_stack_guard: true,
        free_memory_read_protected: true,
        null_pointer_detection_policy: DxeNullDetectionPolicy {
            fields: NullDetectionFields {
                uefi_null_detection: 1,
                disable_end_of_dxe: 0,
                disable_ready_to_boot: 0,
            },
        },
        heap_guard_policy: DxeHeapGuardPolicy { data: 0 },
        image_protection_policy: DxeImageProtectionPolicy { data: 0 },
        heap_guard_pool_type: DxeHeapGuardMemoryTypes { data: 0 },
        heap_guard_page_type: DxeHeapGuardMemoryTypes { data: 0 },
        nx_protection_policy: DxeHeapGuardMemoryTypes { data: 0 },
        stack_cookies: true,
        install_memory_attribute_protocol: true,
    };
