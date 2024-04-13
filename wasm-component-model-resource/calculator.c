// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
#include "calculator.h"


__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Helper Functions

__attribute__((__import_module__("vscode:example/types"), __import_name__("[resource-drop]machine")))
extern void __wasm_import_exports_vscode_example_types_machine_drop(int32_t handle);

void exports_vscode_example_types_machine_drop_own(exports_vscode_example_types_own_machine_t handle) {
  __wasm_import_exports_vscode_example_types_machine_drop(handle.__handle);
}

__attribute__(( __import_module__("[export]vscode:example/types"), __import_name__("[resource-new]machine")))
extern int32_t __wasm_import_exports_vscode_example_types_machine_new(int32_t);

__attribute__((__import_module__("[export]vscode:example/types"), __import_name__("[resource-rep]machine")))
extern int32_t __wasm_import_exports_vscode_example_types_machine_rep(int32_t);

exports_vscode_example_types_own_machine_t exports_vscode_example_types_machine_new(exports_vscode_example_types_machine_t *rep) {
  return (exports_vscode_example_types_own_machine_t) { __wasm_import_exports_vscode_example_types_machine_new((int32_t) rep) };
}

exports_vscode_example_types_machine_t* exports_vscode_example_types_machine_rep(exports_vscode_example_types_own_machine_t handle) {
  return (exports_vscode_example_types_machine_t*) __wasm_import_exports_vscode_example_types_machine_rep(handle.__handle);
}

__attribute__((__export_name__("vscode:example/types#[dtor]machine")))
void __wasm_export_exports_vscode_example_types_machine_dtor(exports_vscode_example_types_machine_t* arg) {
  exports_vscode_example_types_machine_destructor(arg);
}

// Component Adapters

__attribute__((__export_name__("vscode:example/types#[constructor]machine")))
int32_t __wasm_export_exports_vscode_example_types_constructor_machine(int32_t arg, int32_t arg0, int32_t arg1) {
  exports_vscode_example_types_own_machine_t ret = exports_vscode_example_types_constructor_machine((uint32_t) (arg), (uint32_t) (arg0), arg1);
  return (ret).__handle;
}

__attribute__((__export_name__("vscode:example/types#[method]machine.execute")))
int32_t __wasm_export_exports_vscode_example_types_method_machine_execute(int32_t arg) {
  uint32_t ret = exports_vscode_example_types_method_machine_execute(((exports_vscode_example_types_machine_t*) arg));
  return (int32_t) (ret);
}

extern void __component_type_object_force_link_calculator(void);
void __component_type_object_force_link_calculator_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_calculator();
}
