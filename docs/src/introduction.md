# Introduction

Sova
Syscall interceptor, based on [LD_PRELOAD](https://www.baeldung.com/linux/ld_preload-trick-what-is) mechanism.

With that mechanism syscall can be catched, processed, and then allowed or denied.

Works only with dynamic linked executables for Unix/Linux systems.