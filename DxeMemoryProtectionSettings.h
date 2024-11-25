#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Guid Guid;

typedef uint8_t UINT8;

typedef UINT8 DxeMemoryProtectionSettingsVersion;

typedef struct NullDetectionFields {
  UINT8 uefi_null_detection: 1;
  UINT8 disable_end_of_dxe: 1;
  UINT8 disable_ready_to_boot: 1;
} NullDetectionFields;

typedef union DxeNullDetectionPolicy {
  UINT8 data;
  struct NullDetectionFields fields;
} DxeNullDetectionPolicy;

typedef struct HeapGuardFields {
  UINT8 uefi_page_guard: 1;
  UINT8 uefi_pool_guard: 1;
  UINT8 direction: 1;
} HeapGuardFields;

typedef union DxeHeapGuardPolicy {
  UINT8 data;
  struct HeapGuardFields fields;
} DxeHeapGuardPolicy;

typedef struct ImageProtectionFields {
  UINT8 protect_image_from_unknown: 1;
  UINT8 protect_image_from_fv: 1;
  UINT8 raise_error_if_protection_fails: 1;
  UINT8 block_images_without_nx_flag: 1;
} ImageProtectionFields;

typedef union DxeImageProtectionPolicy {
  UINT8 data;
  struct ImageProtectionFields fields;
} DxeImageProtectionPolicy;

typedef uint32_t UINT32;

typedef struct HeapGuardMemoryFields {
  UINT8 efi_reserved_memory_type: 1;
  UINT8 efi_loader_code: 1;
  UINT8 efi_loader_data: 1;
  UINT8 efi_boot_services_code: 1;
  UINT8 efi_boot_services_data: 1;
  UINT8 efi_runtime_services_code: 1;
  UINT8 efi_runtime_services_data: 1;
  UINT8 efi_conventional_memory: 1;
  UINT8 efi_unusable_memory: 1;
  UINT8 efi_acpi_reclaim_memory: 1;
  UINT8 efi_acpi_memory_nvs: 1;
  UINT8 efi_memory_mapped_io: 1;
  UINT8 efi_memory_mapped_io_port_space: 1;
  UINT8 efi_pal_code: 1;
  UINT8 efi_persistent_memory: 1;
  UINT8 efi_unaccepted_memory_type: 1;
  UINT8 oem_reserved: 1;
  UINT8 os_reserved: 1;
} HeapGuardMemoryFields;

typedef union DxeHeapGuardMemoryTypes {
  UINT32 data;
  struct HeapGuardMemoryFields fields;
} DxeHeapGuardMemoryTypes;

typedef struct DxeMemoryProtectionSettings {
  DxeMemoryProtectionSettingsVersion struct_version;
  bool cpu_stack_guard;
  bool free_memory_read_protected;
  union DxeNullDetectionPolicy null_pointer_detection_policy;
  union DxeHeapGuardPolicy heap_guard_policy;
  union DxeImageProtectionPolicy image_protection_policy;
  union DxeHeapGuardMemoryTypes heap_guard_pool_type;
  union DxeHeapGuardMemoryTypes heap_guard_page_type;
  union DxeHeapGuardMemoryTypes nx_protection_policy;
  bool stack_cookies;
  bool install_memory_attribute_protocol;
} DxeMemoryProtectionSettings;

#define DXE_MEMORY_PROTECTION_SETTINGS_CURRENT_VERSION 7

#define HEAP_GUARD_ALIGNED_TO_HEAD 1

#define HEAP_GUARD_ALIGNED_TO_TAIL 0

extern const struct DxeMemoryProtectionSettings DXE_MEMORY_PROTECTION_SETTINGS_SHIP_MODE_NO_PAGE_GUARDS;

extern const struct Guid HOB_DXE_MEMORY_PROTECTION_SETTINGS_GUID;

struct DxeMemoryProtectionSettings ship_mode_no_page_guard_settings(void);
