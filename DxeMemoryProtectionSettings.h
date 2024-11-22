#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_GUARD_ALIGNED_TO_HEAD 1

#define HEAP_GUARD_ALIGNED_TO_TAIL 0

typedef uint8_t DxeMemoryProtectionSettingsVersion;

typedef struct NullDetectionFields {
  uint8_t uefi_null_detection: 1;
  uint8_t disable_end_of_dxe: 1;
  uint8_t disable_ready_to_boot: 1;
} NullDetectionFields;

typedef union DxeNullDetectionPolicy {
  uint8_t data;
  struct NullDetectionFields fields;
} DxeNullDetectionPolicy;

typedef struct HeapGuardFields {
  uint8_t uefi_page_guard: 1;
  uint8_t uefi_pool_guard: 1;
  uint8_t direction: 1;
} HeapGuardFields;

typedef union DxeHeapGuardPolicy {
  uint8_t data;
  struct HeapGuardFields fields;
} DxeHeapGuardPolicy;

typedef struct ImageProtectionFields {
  uint8_t protect_image_from_unknown: 1;
  uint8_t protect_image_from_fv: 1;
  uint8_t raise_error_if_protection_fails: 1;
  uint8_t block_images_without_nx_flag: 1;
} ImageProtectionFields;

typedef union DxeImageProtectionPolicy {
  uint8_t data;
  struct ImageProtectionFields fields;
} DxeImageProtectionPolicy;

typedef struct HeapGuardMemoryFields {
  uint8_t efi_reserved_memory_type: 1;
  uint8_t efi_loader_code: 1;
  uint8_t efi_loader_data: 1;
  uint8_t efi_boot_services_code: 1;
  uint8_t efi_boot_services_data: 1;
  uint8_t efi_runtime_services_code: 1;
  uint8_t efi_runtime_services_data: 1;
  uint8_t efi_conventional_memory: 1;
  uint8_t efi_unusable_memory: 1;
  uint8_t efi_acpi_reclaim_memory: 1;
  uint8_t efi_acpi_memory_nvs: 1;
  uint8_t efi_memory_mapped_io: 1;
  uint8_t efi_memory_mapped_io_port_space: 1;
  uint8_t efi_pal_code: 1;
  uint8_t efi_persistent_memory: 1;
  uint8_t efi_unaccepted_memory_type: 1;
  uint8_t oem_reserved: 1;
  uint8_t os_reserved: 1;
} HeapGuardMemoryFields;

typedef union DxeHeapGuardMemoryTypes {
  uint32_t data;
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

struct DxeMemoryProtectionSettings ship_mode_no_page_guard_settings(void);
