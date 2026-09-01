#include <stdatomic.h>
#include <stddef.h>
#include <stdint.h>

extern void *__libc_malloc(size_t size);
extern void *__libc_calloc(size_t count, size_t size);
extern void *__libc_realloc(void *pointer, size_t size);
extern void __libc_free(void *pointer);

static _Atomic uint64_t allocation_calls;
static _Atomic uint64_t allocated_bytes;
static _Atomic uint64_t free_calls;
static _Atomic int counting;

void loader_bench_alloc_reset(void) {
    atomic_store_explicit(&allocation_calls, 0, memory_order_relaxed);
    atomic_store_explicit(&allocated_bytes, 0, memory_order_relaxed);
    atomic_store_explicit(&free_calls, 0, memory_order_relaxed);
    atomic_store_explicit(&counting, 1, memory_order_release);
}

void loader_bench_alloc_snapshot(uint64_t *allocations, uint64_t *bytes, uint64_t *frees) {
    atomic_store_explicit(&counting, 0, memory_order_release);
    *allocations = atomic_load_explicit(&allocation_calls, memory_order_relaxed);
    *bytes = atomic_load_explicit(&allocated_bytes, memory_order_relaxed);
    *frees = atomic_load_explicit(&free_calls, memory_order_relaxed);
}

void *malloc(size_t size) {
    void *pointer = __libc_malloc(size);
    if (pointer != NULL && atomic_load_explicit(&counting, memory_order_acquire)) {
        atomic_fetch_add_explicit(&allocation_calls, 1, memory_order_relaxed);
        atomic_fetch_add_explicit(&allocated_bytes, size, memory_order_relaxed);
    }
    return pointer;
}

void *calloc(size_t count, size_t size) {
    void *pointer = __libc_calloc(count, size);
    if (pointer != NULL && atomic_load_explicit(&counting, memory_order_acquire)) {
        atomic_fetch_add_explicit(&allocation_calls, 1, memory_order_relaxed);
        atomic_fetch_add_explicit(&allocated_bytes, count * size, memory_order_relaxed);
    }
    return pointer;
}

void *realloc(void *old_pointer, size_t size) {
    void *pointer = __libc_realloc(old_pointer, size);
    if (pointer != NULL && atomic_load_explicit(&counting, memory_order_acquire)) {
        atomic_fetch_add_explicit(&allocation_calls, 1, memory_order_relaxed);
        atomic_fetch_add_explicit(&allocated_bytes, size, memory_order_relaxed);
    }
    return pointer;
}

void free(void *pointer) {
    if (pointer != NULL && atomic_load_explicit(&counting, memory_order_acquire)) {
        atomic_fetch_add_explicit(&free_calls, 1, memory_order_relaxed);
    }
    __libc_free(pointer);
}
