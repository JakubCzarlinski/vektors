#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

typedef uint32_t (*validate_string_fn)(int max_length, const char *string);

static void report(validate_string_fn validate, const char *name, int max_length, const uint8_t *string) {
    printf("%s=%u\n", name, validate(max_length, (const char *)string));
}

int main(void) {
    void *loader = dlopen("libvulkan.so.1", RTLD_NOW | RTLD_LOCAL);
    if (loader == NULL) {
        fprintf(stderr, "dlopen: %s\n", dlerror());
        return 1;
    }
    void *symbol = dlsym(loader, "vk_string_validate");
    validate_string_fn validate = NULL;
    memcpy(&validate, &symbol, sizeof(validate));
    if (validate == NULL) {
        fprintf(stderr, "dlsym: %s\n", dlerror());
        return 2;
    }

    const uint8_t truncated_two[] = {0xc2, 0};
    const uint8_t truncated_three[] = {0xe2, 0};
    const uint8_t truncated_four[] = {0xf0, 0};
    const uint8_t short_three[] = {0xe2, 0x82, 0};
    const uint8_t valid_ascii[] = {'a', 'b', 'c', 0};
    const uint8_t valid_two[] = {0xc2, 0xa9, 0};
    const uint8_t invalid_lead[] = {0x80, 0};
    uint8_t overlong[258];
    memset(overlong, 'a', sizeof(overlong));
    overlong[sizeof(overlong) - 1] = 0;

    printf("null=%u\n", validate(256, NULL));
    report(validate, "truncated_two", 256, truncated_two);
    report(validate, "truncated_three", 256, truncated_three);
    report(validate, "truncated_four", 256, truncated_four);
    report(validate, "short_three", 256, short_three);
    report(validate, "valid_ascii", 256, valid_ascii);
    report(validate, "valid_two", 256, valid_two);
    report(validate, "invalid_lead", 256, invalid_lead);
    report(validate, "overlong", 256, overlong);

    return dlclose(loader) == 0 ? 0 : 3;
}
