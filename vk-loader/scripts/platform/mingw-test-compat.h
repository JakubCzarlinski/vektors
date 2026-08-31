#pragma once

#include <algorithm>

// Vulkan-Loader's Windows tests use the MSVC debug-report API. MinGW does not
// provide it, and the tests only need these calls to compile when run under
// Wine; no loader behavior depends on their return values.
#if defined(__MINGW32__)
#define _CRT_ASSERT 0
#define _CRT_WARN 1
#define _CRT_ERROR 2
#define _CRTDBG_MODE_FILE 0
#define _CRTDBG_FILE_STDERR nullptr
static inline int _CrtSetReportMode(int, int) { return 0; }
static inline void *_CrtSetReportFile(int, void *) { return nullptr; }
#endif
