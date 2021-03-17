// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!
{% import "macros.rs" as rs %}

// Check for compatibility between `uniffi` and `uniffi_bindgen` versions.
// Note that we have an error message on the same line as the assertion.
// This is important, because if the assertion fails, the compiler only
// seems to show that single line as context for the user.
uniffi::assert_compatible_version!("{{ uniffi_version }}"); // Please check that you depend on version {{ uniffi_version }} of the `uniffi` crate.

{% include "RustBuffer.rs" %}

// We generate error mappings into ffi_support::ExternErrors
// so that the errors can propagate through the FFI
{% for e in ci.iter_error_definitions() %}
{% include "ErrorTemplate.rs" %}
{% endfor %}

// Enum defitions, corresponding to `enum` in UDL.
{% for e in ci.iter_enum_definitions() %}
{% include "EnumTemplate.rs" %}
{% endfor %}

// Record definitions, implemented as method-less structs, corresponding to `dictionary` objects.
{% for rec in ci.iter_record_definitions() %}
{% include "RecordTemplate.rs" %}
{% endfor %}

// Top level functions, corresponding to UDL `namespace` functions.
{%- for func in ci.iter_function_definitions() %}
{% include "TopLevelFunctionTemplate.rs" %}
{% endfor -%}

// Object definitions, correspoding to UDL `interface` definitions.
{% for obj in ci.iter_object_definitions() %}
{% include "ObjectTemplate.rs" %}
{% endfor %}

// Callback Interface defitions, corresponding to UDL `callback interface` definitions.
{% for cbi in ci.iter_callback_interface_definitions() %}
{% include "CallbackInterfaceTemplate.rs" %}
{% endfor %}

{%- import "macros.rs" as rs -%}
