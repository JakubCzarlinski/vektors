#pragma once

// MinGW installs the Windows SDK header in lowercase on case-sensitive hosts,
// while the unchanged upstream test suite also uses Microsoft's `Windows.h`.
#include <windows.h>
