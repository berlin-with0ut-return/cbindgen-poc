#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_GUARD_ALIGNED_TO_HEAD 1

#define HEAP_GUARD_ALIGNED_TO_TAIL 0

typedef uint8_t DxeMemoryProtectionSettingsVersion;

typedef struct NullDetectionFields {
  uint8_t bits;
} NullDetectionFields;
#define NullDetectionFields_UEFI_NULL_DETECTION (NullDetectionFields){ .bits = (uint8_t)1 }
#define NullDetectionFields_DISABLE_END_OF_DXE (NullDetectionFields){ .bits = (uint8_t)2 }
#define NullDetectionFields_DISABLE_READY_TO_BOOT (NullDetectionFields){ .bits = (uint8_t)4 }

typedef union DxeNullDetectionPolicy {
  uint8_t data;
  struct NullDetectionFields fields;
} DxeNullDetectionPolicy;

typedef struct HeapGuardFields {
  uint8_t bits;
} HeapGuardFields;
#define HeapGuardFields_UEFI_PAGE_GUARD (HeapGuardFields){ .bits = (uint8_t)1 }
#define HeapGuardFields_UEFI_POOL_GUARD (HeapGuardFields){ .bits = (uint8_t)2 }
#define HeapGuardFields_DIRECTION (HeapGuardFields){ .bits = (uint8_t)4 }

typedef union DxeHeapGuardPolicy {
  uint8_t data;
  struct HeapGuardFields fields;
} DxeHeapGuardPolicy;

typedef struct ImageProtectionFields {
  uint8_t bits;
} ImageProtectionFields;
#define ImageProtectionFields_PROTECT_IMAGE_FROM_UNKNOWN (ImageProtectionFields){ .bits = (uint8_t)1 }
#define ImageProtectionFields_PROTECT_IMAGE_FROM_FV (ImageProtectionFields){ .bits = (uint8_t)2 }
#define ImageProtectionFields_RAISE_ERROR_IF_PROTECTION_FAILS (ImageProtectionFields){ .bits = (uint8_t)4 }
#define ImageProtectionFields_BLOCK_IMAGES_WITHOUT_NX_FLAG (ImageProtectionFields){ .bits = (uint8_t)8 }

typedef union DxeImageProtectionPolicy {
  uint8_t data;
  struct ImageProtectionFields fields;
} DxeImageProtectionPolicy;

typedef struct HeapGuardMemoryFields {
  uint32_t bits;
} HeapGuardMemoryFields;
#define HeapGuardMemoryFields_EFI_RESERVED_MEMORY_TYPE (HeapGuardMemoryFields){ .bits = (uint32_t)1 }
#define HeapGuardMemoryFields_EFI_LOADER_CODE (HeapGuardMemoryFields){ .bits = (uint32_t)2 }
#define HeapGuardMemoryFields_EFI_LOADER_DATA (HeapGuardMemoryFields){ .bits = (uint32_t)4 }
#define HeapGuardMemoryFields_EFI_BOOT_SERVICES_CODE (HeapGuardMemoryFields){ .bits = (uint32_t)8 }
#define HeapGuardMemoryFields_EFI_BOOT_SERVICES_DATA (HeapGuardMemoryFields){ .bits = (uint32_t)16 }
#define HeapGuardMemoryFields_EFI_RUNTIME_SERVICES_CODE (HeapGuardMemoryFields){ .bits = (uint32_t)32 }
#define HeapGuardMemoryFields_EFI_RUNTIME_SERVICES_DATA (HeapGuardMemoryFields){ .bits = (uint32_t)64 }
#define HeapGuardMemoryFields_EFI_CONVENTIONAL_MEMORY (HeapGuardMemoryFields){ .bits = (uint32_t)128 }
#define HeapGuardMemoryFields_EFI_UNUSABLE_MEMORY (HeapGuardMemoryFields){ .bits = (uint32_t)256 }
#define HeapGuardMemoryFields_EFI_ACPI_RECLAIM_MEMORY (HeapGuardMemoryFields){ .bits = (uint32_t)512 }
#define HeapGuardMemoryFields_EFI_ACPI_MEMORY_NVS (HeapGuardMemoryFields){ .bits = (uint32_t)1024 }
#define HeapGuardMemoryFields_EFI_MEMORY_MAPPED_IO (HeapGuardMemoryFields){ .bits = (uint32_t)2048 }
#define HeapGuardMemoryFields_EFI_MEMORY_MAPPED_IO_PORT_SPACE (HeapGuardMemoryFields){ .bits = (uint32_t)4096 }
#define HeapGuardMemoryFields_EFI_PAL_CODE (HeapGuardMemoryFields){ .bits = (uint32_t)8192 }
#define HeapGuardMemoryFields_EFI_PERSISTENT_MEMORY (HeapGuardMemoryFields){ .bits = (uint32_t)16384 }
#define HeapGuardMemoryFields_EFI_UNACCEPTED_MEMORY_TYPE (HeapGuardMemoryFields){ .bits = (uint32_t)32768 }
#define HeapGuardMemoryFields_OEM_RESERVED (HeapGuardMemoryFields){ .bits = (uint32_t)65536 }
#define HeapGuardMemoryFields_OS_RESERVED (HeapGuardMemoryFields){ .bits = (uint32_t)131072 }

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

struct DxeMemoryProtectionSettings debug_settings(void);

struct DxeMemoryProtectionSettings protection_off_settings(void);

struct DxeMemoryProtectionSettings ship_mode_no_page_guard_settings(void);

struct DxeMemoryProtectionSettings ship_mode_settings(void);
