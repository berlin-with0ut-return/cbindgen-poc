#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_GUARD_ALIGNED_TO_HEAD 1

#define HEAP_GUARD_ALIGNED_TO_TAIL 0

typedef uint8_t DxeMemoryProtectionSettingsVersion;

typedef union DxeNullDetectionPolicy {
  uint8_t data;
  NullDetectionFields fields;
} DxeNullDetectionPolicy;

typedef union DxeHeapGuardPolicy {
  uint8_t data;
  HeapGuardFields fields;
} DxeHeapGuardPolicy;

typedef union DxeImageProtectionPolicy {
  uint8_t data;
  ImageProtectionFields fields;
} DxeImageProtectionPolicy;

typedef union DxeHeapGuardMemoryTypes {
  uint32_t data;
  HeapGuardMemoryFields fields;
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
