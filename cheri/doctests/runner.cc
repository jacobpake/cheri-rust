#define TEST_NAME "RUST"
#include "cheri.h"
#include "cheri.hh"
#include "fail-simulator-on-error.h"
#include <cstdlib>

using Test = ConditionalDebug<true,
#ifdef TEST_NAME
                              TEST_NAME " test"
#else
                              "Test runner"
#endif
                              >;

#define TEST(cond, msg, ...) Test::Invariant((cond), msg, ##__VA_ARGS__)

extern "C" void __rust_main();

/* Things that Rust expects from us */
extern "C" void cheriot_print_str(char *s) { printf("%s", s); }

extern "C" void *cheriot_alloc(size_t size) {
  // debug_log("Trying to allocate {} bytes!", size);
  Timeout timeout{5};
  void *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);

  TEST(CHERI::Capability{ret}.is_valid(),
       "Allocation is invalid, got pointer: {} -- {}", ret, (int)ret);
  return ret;
}

extern "C" void cheriot_free(void *ptr) { free(ptr); }

int __attribute__((cheriot_compartment("test_runner"))) run() {
  __rust_main();
  return 0;
}
