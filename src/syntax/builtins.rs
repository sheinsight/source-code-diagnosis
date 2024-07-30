
use super::{compat::Compat, support::Support};

pub struct Builtins<'a>{
    
      pub aggregate_error: Compat<'a>,
    
      pub aggregate_error_aggregate_error: Compat<'a>,
    
      pub aggregate_error_errors: Compat<'a>,
    
      pub aggregate_error_serializable_object: Compat<'a>,
    
      pub array: Compat<'a>,
    
      pub array_array: Compat<'a>,
    
      pub array_at: Compat<'a>,
    
      pub array_concat: Compat<'a>,
    
      pub array_copy_with_in: Compat<'a>,
    
      pub array_entries: Compat<'a>,
    
      pub array_every: Compat<'a>,
    
      pub array_fill: Compat<'a>,
    
      pub array_filter: Compat<'a>,
    
      pub array_find: Compat<'a>,
    
      pub array_find_index: Compat<'a>,
    
      pub array_find_last: Compat<'a>,
    
      pub array_find_last_index: Compat<'a>,
    
      pub array_flat: Compat<'a>,
    
      pub array_flat_map: Compat<'a>,
    
      pub array_foreach: Compat<'a>,
    
      pub array_from: Compat<'a>,
    
      pub array_from_async: Compat<'a>,
    
      pub array_includes: Compat<'a>,
    
      pub array_index_of: Compat<'a>,
    
      pub array_is_array: Compat<'a>,
    
      pub array_join: Compat<'a>,
    
      pub array_keys: Compat<'a>,
    
      pub array_last_index_of: Compat<'a>,
    
      pub array_length: Compat<'a>,
    
      pub array_map: Compat<'a>,
    
      pub array_of: Compat<'a>,
    
      pub array_pop: Compat<'a>,
    
      pub array_push: Compat<'a>,
    
      pub array_reduce: Compat<'a>,
    
      pub array_reduce_right: Compat<'a>,
    
      pub array_reverse: Compat<'a>,
    
      pub array_shift: Compat<'a>,
    
      pub array_slice: Compat<'a>,
    
      pub array_some: Compat<'a>,
    
      pub array_sort: Compat<'a>,
    
      pub array_sort_stable_sorting: Compat<'a>,
    
      pub array_splice: Compat<'a>,
    
      pub array_to_locale_string: Compat<'a>,
    
      pub array_to_locale_string_locales_parameter: Compat<'a>,
    
      pub array_to_locale_string_options_parameter: Compat<'a>,
    
      pub array_to_reversed: Compat<'a>,
    
      pub array_to_sorted: Compat<'a>,
    
      pub array_to_spliced: Compat<'a>,
    
      pub array_to_string: Compat<'a>,
    
      pub array_un_shift: Compat<'a>,
    
      pub array_values: Compat<'a>,
    
      pub array_with: Compat<'a>,
    
      pub array_iterator_symbol: Compat<'a>,
    
      pub array_species_symbol: Compat<'a>,
    
      pub array_unscopables_symbol: Compat<'a>,
    
      pub array_buffer: Compat<'a>,
    
      pub array_buffer_array_buffer: Compat<'a>,
    
      pub array_buffer_array_buffer_max_byte_length_option: Compat<'a>,
    
      pub array_buffer_byte_length: Compat<'a>,
    
      pub array_buffer_detached: Compat<'a>,
    
      pub array_buffer_is_view: Compat<'a>,
    
      pub array_buffer_max_byte_length: Compat<'a>,
    
      pub array_buffer_resizable: Compat<'a>,
    
      pub array_buffer_resize: Compat<'a>,
    
      pub array_buffer_slice: Compat<'a>,
    
      pub array_buffer_transfer: Compat<'a>,
    
      pub array_buffer_transfer_to_fixed_length: Compat<'a>,
    
      pub array_buffer_species_symbol: Compat<'a>,
    
      pub async_function: Compat<'a>,
    
      pub async_function_async_function: Compat<'a>,
    
      pub async_generator: Compat<'a>,
    
      pub async_generator_next: Compat<'a>,
    
      pub async_generator_return: Compat<'a>,
    
      pub async_generator_throw: Compat<'a>,
    
      pub async_generator_function: Compat<'a>,
    
      pub async_generator_function_async_generator_function: Compat<'a>,
    
      pub async_iterator: Compat<'a>,
    
      pub async_iterator_async_iterator_symbol: Compat<'a>,
    
      pub atomics: Compat<'a>,
    
      pub atomics_atomic_operations_on_non_shared_buffers: Compat<'a>,
    
      pub atomics_add: Compat<'a>,
    
      pub atomics_and: Compat<'a>,
    
      pub atomics_compare_exchange: Compat<'a>,
    
      pub atomics_exchange: Compat<'a>,
    
      pub atomics_is_lock_free: Compat<'a>,
    
      pub atomics_load: Compat<'a>,
    
      pub atomics_notify: Compat<'a>,
    
      pub atomics_or: Compat<'a>,
    
      pub atomics_store: Compat<'a>,
    
      pub atomics_sub: Compat<'a>,
    
      pub atomics_wait: Compat<'a>,
    
      pub atomics_wait_async: Compat<'a>,
    
      pub atomics_xor: Compat<'a>,
    
      pub bigint: Compat<'a>,
    
      pub bigint_bigint: Compat<'a>,
    
      pub bigint_as_int_n: Compat<'a>,
    
      pub bigint_as_uint_n: Compat<'a>,
    
      pub bigint_to_locale_string: Compat<'a>,
    
      pub bigint_to_locale_string_locales_parameter: Compat<'a>,
    
      pub bigint_to_locale_string_options_parameter: Compat<'a>,
    
      pub bigint_to_string: Compat<'a>,
    
      pub bigint_value_of: Compat<'a>,
    
      pub big_int_64_array: Compat<'a>,
    
      pub big_int_64_array_big_int_64_array: Compat<'a>,
    
      pub big_uint_64_array: Compat<'a>,
    
      pub big_uint_64_array_big_uint_64_array: Compat<'a>,
    
      pub boolean: Compat<'a>,
    
      pub boolean_boolean: Compat<'a>,
    
      pub boolean_to_string: Compat<'a>,
    
      pub boolean_value_of: Compat<'a>,
    
      pub data_view: Compat<'a>,
    
      pub data_view_data_view: Compat<'a>,
    
      pub data_view_data_view_shared_array_buffer_support: Compat<'a>,
    
      pub data_view_buffer: Compat<'a>,
    
      pub data_view_byte_length: Compat<'a>,
    
      pub data_view_byte_offset: Compat<'a>,
    
      pub data_view_get_big_int_64: Compat<'a>,
    
      pub data_view_get_big_uint_64: Compat<'a>,
    
      pub data_view_get_float_16: Compat<'a>,
    
      pub data_view_get_float_32: Compat<'a>,
    
      pub data_view_get_float_64: Compat<'a>,
    
      pub data_view_get_int_16: Compat<'a>,
    
      pub data_view_get_int_32: Compat<'a>,
    
      pub data_view_get_int_8: Compat<'a>,
    
      pub data_view_get_uint_16: Compat<'a>,
    
      pub data_view_get_uint_32: Compat<'a>,
    
      pub data_view_get_uint_8: Compat<'a>,
    
      pub data_view_set_big_int_64: Compat<'a>,
    
      pub data_view_set_big_uint_64: Compat<'a>,
    
      pub data_view_set_float_16: Compat<'a>,
    
      pub data_view_set_float_32: Compat<'a>,
    
      pub data_view_set_float_64: Compat<'a>,
    
      pub data_view_set_int_16: Compat<'a>,
    
      pub data_view_set_int_32: Compat<'a>,
    
      pub data_view_set_int_8: Compat<'a>,
    
      pub data_view_set_uint_16: Compat<'a>,
    
      pub data_view_set_uint_32: Compat<'a>,
    
      pub data_view_set_uint_8: Compat<'a>,
    
      pub date: Compat<'a>,
    
      pub date_date: Compat<'a>,
    
      pub date_utc: Compat<'a>,
    
      pub date_utc_optional_month_index: Compat<'a>,
    
      pub date_get_date: Compat<'a>,
    
      pub date_get_day: Compat<'a>,
    
      pub date_get_full_year: Compat<'a>,
    
      pub date_get_hours: Compat<'a>,
    
      pub date_get_milliseconds: Compat<'a>,
    
      pub date_get_minutes: Compat<'a>,
    
      pub date_get_month: Compat<'a>,
    
      pub date_get_seconds: Compat<'a>,
    
      pub date_get_time: Compat<'a>,
    
      pub date_get_timezone_offset: Compat<'a>,
    
      pub date_get_utc_date: Compat<'a>,
    
      pub date_get_utc_day: Compat<'a>,
    
      pub date_get_utc_full_year: Compat<'a>,
    
      pub date_get_utc_hours: Compat<'a>,
    
      pub date_get_utc_milliseconds: Compat<'a>,
    
      pub date_get_utc_minutes: Compat<'a>,
    
      pub date_get_utc_month: Compat<'a>,
    
      pub date_get_utc_seconds: Compat<'a>,
    
      pub date_get_year: Compat<'a>,
    
      pub date_now: Compat<'a>,
    
      pub date_parse: Compat<'a>,
    
      pub date_parse_iso_8601: Compat<'a>,
    
      pub date_set_date: Compat<'a>,
    
      pub date_set_full_year: Compat<'a>,
    
      pub date_set_hours: Compat<'a>,
    
      pub date_set_milliseconds: Compat<'a>,
    
      pub date_set_minutes: Compat<'a>,
    
      pub date_set_month: Compat<'a>,
    
      pub date_set_seconds: Compat<'a>,
    
      pub date_set_time: Compat<'a>,
    
      pub date_set_utc_date: Compat<'a>,
    
      pub date_set_utc_full_year: Compat<'a>,
    
      pub date_set_utc_hours: Compat<'a>,
    
      pub date_set_utc_milliseconds: Compat<'a>,
    
      pub date_set_utc_minutes: Compat<'a>,
    
      pub date_set_utc_month: Compat<'a>,
    
      pub date_set_utc_seconds: Compat<'a>,
    
      pub date_set_year: Compat<'a>,
    
      pub date_to_date_string: Compat<'a>,
    
      pub date_to_gmt_string: Compat<'a>,
    
      pub date_to_iso_string: Compat<'a>,
    
      pub date_to_json: Compat<'a>,
    
      pub date_to_locale_date_string: Compat<'a>,
    
      pub date_to_locale_date_string_iana_time_zone_names: Compat<'a>,
    
      pub date_to_locale_date_string_locales_parameter: Compat<'a>,
    
      pub date_to_locale_date_string_options_parameter: Compat<'a>,
    
      pub date_to_locale_string: Compat<'a>,
    
      pub date_to_locale_string_iana_time_zone_names: Compat<'a>,
    
      pub date_to_locale_string_locales_parameter: Compat<'a>,
    
      pub date_to_locale_string_options_parameter: Compat<'a>,
    
      pub date_to_locale_time_string: Compat<'a>,
    
      pub date_to_locale_time_string_iana_time_zone_names: Compat<'a>,
    
      pub date_to_locale_time_string_locales_parameter: Compat<'a>,
    
      pub date_to_locale_time_string_options_parameter: Compat<'a>,
    
      pub date_to_string: Compat<'a>,
    
      pub date_to_time_string: Compat<'a>,
    
      pub date_to_utc_string: Compat<'a>,
    
      pub date_value_of: Compat<'a>,
    
      pub date_to_primitive_symbol: Compat<'a>,
    
      pub error: Compat<'a>,
    
      pub error_error: Compat<'a>,
    
      pub error_error_filename_parameter: Compat<'a>,
    
      pub error_error_linenumber_parameter: Compat<'a>,
    
      pub error_error_options_cause_parameter: Compat<'a>,
    
      pub error_cause: Compat<'a>,
    
      pub error_cause_displayed_in_console: Compat<'a>,
    
      pub error_column_number: Compat<'a>,
    
      pub error_filename: Compat<'a>,
    
      pub error_linenumber: Compat<'a>,
    
      pub error_message: Compat<'a>,
    
      pub error_name: Compat<'a>,
    
      pub error_serializable_object: Compat<'a>,
    
      pub error_stack: Compat<'a>,
    
      pub error_to_string: Compat<'a>,
    
      pub eval_error: Compat<'a>,
    
      pub eval_error_eval_error: Compat<'a>,
    
      pub eval_error_serializable_object: Compat<'a>,
    
      pub finalization_registry: Compat<'a>,
    
      pub finalization_registry_finalization_registry: Compat<'a>,
    
      pub finalization_registry_register: Compat<'a>,
    
      pub finalization_registry_symbol_as_target: Compat<'a>,
    
      pub finalization_registry_unregister: Compat<'a>,
    
      pub float16array: Compat<'a>,
    
      pub float16array_float16array: Compat<'a>,
    
      pub float32array: Compat<'a>,
    
      pub float32array_float32array: Compat<'a>,
    
      pub float32array_float32array_constructor_without_parameters: Compat<'a>,
    
      pub float32array_float32array_iterable_allowed: Compat<'a>,
    
      pub float64array: Compat<'a>,
    
      pub float64array_float64array: Compat<'a>,
    
      pub float64array_float64array_constructor_without_parameters: Compat<'a>,
    
      pub float64array_float64array_iterable_allowed: Compat<'a>,
    
      pub function: Compat<'a>,
    
      pub function_function: Compat<'a>,
    
      pub function_apply: Compat<'a>,
    
      pub function_apply_generic_arrays_as_arguments: Compat<'a>,
    
      pub function_arguments: Compat<'a>,
    
      pub function_bind: Compat<'a>,
    
      pub function_call: Compat<'a>,
    
      pub function_caller: Compat<'a>,
    
      pub function_display_name: Compat<'a>,
    
      pub function_length: Compat<'a>,
    
      pub function_length_configurable_true: Compat<'a>,
    
      pub function_name: Compat<'a>,
    
      pub function_name_configurable_true: Compat<'a>,
    
      pub function_name_inferred_names: Compat<'a>,
    
      pub function_to_string: Compat<'a>,
    
      pub function_to_string_to_string_revision: Compat<'a>,
    
      pub function_has_instance_symbol: Compat<'a>,
    
      pub generator: Compat<'a>,
    
      pub generator_next: Compat<'a>,
    
      pub generator_return: Compat<'a>,
    
      pub generator_throw: Compat<'a>,
    
      pub generator_function: Compat<'a>,
    
      pub generator_function_generator_function: Compat<'a>,
    
      pub int16array: Compat<'a>,
    
      pub int16array_int16array: Compat<'a>,
    
      pub int16array_int16array_constructor_without_parameters: Compat<'a>,
    
      pub int16array_int16array_iterable_allowed: Compat<'a>,
    
      pub int32array: Compat<'a>,
    
      pub int32array_int32array: Compat<'a>,
    
      pub int32array_int32array_constructor_without_parameters: Compat<'a>,
    
      pub int32array_int32array_iterable_allowed: Compat<'a>,
    
      pub int8array: Compat<'a>,
    
      pub int8array_int8array: Compat<'a>,
    
      pub int8array_int8array_constructor_without_parameters: Compat<'a>,
    
      pub int8array_int8array_iterable_allowed: Compat<'a>,
    
      pub internal_error: Compat<'a>,
    
      pub internal_error_internal_error: Compat<'a>,
    
      pub intl: Compat<'a>,
    
      pub intl_get_canonical_locales: Compat<'a>,
    
      pub intl_supported_values_of: Compat<'a>,
    
      pub iterator: Compat<'a>,
    
      pub iterator_iterator: Compat<'a>,
    
      pub iterator_drop: Compat<'a>,
    
      pub iterator_every: Compat<'a>,
    
      pub iterator_filter: Compat<'a>,
    
      pub iterator_find: Compat<'a>,
    
      pub iterator_flat_map: Compat<'a>,
    
      pub iterator_foreach: Compat<'a>,
    
      pub iterator_from: Compat<'a>,
    
      pub iterator_map: Compat<'a>,
    
      pub iterator_reduce: Compat<'a>,
    
      pub iterator_some: Compat<'a>,
    
      pub iterator_take: Compat<'a>,
    
      pub iterator_to_array: Compat<'a>,
    
      pub iterator_iterator_symbol: Compat<'a>,
    
      pub json: Compat<'a>,
    
      pub json_is_raw_json: Compat<'a>,
    
      pub json_json_superset: Compat<'a>,
    
      pub json_parse: Compat<'a>,
    
      pub json_parse_reviver_parameter_context_argument: Compat<'a>,
    
      pub json_raw_json: Compat<'a>,
    
      pub json_stringify: Compat<'a>,
    
      pub json_stringify_well_formed_stringify: Compat<'a>,
    
      pub map: Compat<'a>,
    
      pub map_map: Compat<'a>,
    
      pub map_map_iterable_allowed: Compat<'a>,
    
      pub map_map_null_allowed: Compat<'a>,
    
      pub map_clear: Compat<'a>,
    
      pub map_delete: Compat<'a>,
    
      pub map_entries: Compat<'a>,
    
      pub map_foreach: Compat<'a>,
    
      pub map_get: Compat<'a>,
    
      pub map_group_by: Compat<'a>,
    
      pub map_has: Compat<'a>,
    
      pub map_key_equality_for_zeros: Compat<'a>,
    
      pub map_keys: Compat<'a>,
    
      pub map_set: Compat<'a>,
    
      pub map_size: Compat<'a>,
    
      pub map_values: Compat<'a>,
    
      pub map_iterator_symbol: Compat<'a>,
    
      pub map_species_symbol: Compat<'a>,
    
      pub math: Compat<'a>,
    
      pub math_e: Compat<'a>,
    
      pub math_ln10: Compat<'a>,
    
      pub math_ln2: Compat<'a>,
    
      pub math_log10e: Compat<'a>,
    
      pub math_log2e: Compat<'a>,
    
      pub math_pi: Compat<'a>,
    
      pub math_sqrt1_2: Compat<'a>,
    
      pub math_sqrt2: Compat<'a>,
    
      pub math_abs: Compat<'a>,
    
      pub math_acos: Compat<'a>,
    
      pub math_acosh: Compat<'a>,
    
      pub math_asin: Compat<'a>,
    
      pub math_asinh: Compat<'a>,
    
      pub math_atan: Compat<'a>,
    
      pub math_atan2: Compat<'a>,
    
      pub math_atanh: Compat<'a>,
    
      pub math_cbrt: Compat<'a>,
    
      pub math_ceil: Compat<'a>,
    
      pub math_clz32: Compat<'a>,
    
      pub math_cos: Compat<'a>,
    
      pub math_cosh: Compat<'a>,
    
      pub math_exp: Compat<'a>,
    
      pub math_expm1: Compat<'a>,
    
      pub math_f16round: Compat<'a>,
    
      pub math_floor: Compat<'a>,
    
      pub math_fround: Compat<'a>,
    
      pub math_hypot: Compat<'a>,
    
      pub math_imul: Compat<'a>,
    
      pub math_log: Compat<'a>,
    
      pub math_log10: Compat<'a>,
    
      pub math_log1p: Compat<'a>,
    
      pub math_log2: Compat<'a>,
    
      pub math_max: Compat<'a>,
    
      pub math_min: Compat<'a>,
    
      pub math_pow: Compat<'a>,
    
      pub math_random: Compat<'a>,
    
      pub math_round: Compat<'a>,
    
      pub math_sign: Compat<'a>,
    
      pub math_sin: Compat<'a>,
    
      pub math_sinh: Compat<'a>,
    
      pub math_sqrt: Compat<'a>,
    
      pub math_tan: Compat<'a>,
    
      pub math_tanh: Compat<'a>,
    
      pub math_trunc: Compat<'a>,
    
      pub number: Compat<'a>,
    
      pub number_epsilon: Compat<'a>,
    
      pub number_max_safe_integer: Compat<'a>,
    
      pub number_max_value: Compat<'a>,
    
      pub number_min_safe_integer: Compat<'a>,
    
      pub number_min_value: Compat<'a>,
    
      pub number_nan: Compat<'a>,
    
      pub number_negative_infinity: Compat<'a>,
    
      pub number_number: Compat<'a>,
    
      pub number_positive_infinity: Compat<'a>,
    
      pub number_is_finite: Compat<'a>,
    
      pub number_is_integer: Compat<'a>,
    
      pub number_is_nan: Compat<'a>,
    
      pub number_is_safe_integer: Compat<'a>,
    
      pub number_parse_float: Compat<'a>,
    
      pub number_parse_int: Compat<'a>,
    
      pub number_to_exponential: Compat<'a>,
    
      pub number_to_fixed: Compat<'a>,
    
      pub number_to_locale_string: Compat<'a>,
    
      pub number_to_locale_string_locales_parameter: Compat<'a>,
    
      pub number_to_locale_string_options_parameter: Compat<'a>,
    
      pub number_to_precision: Compat<'a>,
    
      pub number_to_string: Compat<'a>,
    
      pub number_value_of: Compat<'a>,
    
      pub object: Compat<'a>,
    
      pub object_object: Compat<'a>,
    
      pub object_assign: Compat<'a>,
    
      pub object_constructor: Compat<'a>,
    
      pub object_create: Compat<'a>,
    
      pub object_define_getter: Compat<'a>,
    
      pub object_define_properties: Compat<'a>,
    
      pub object_define_property: Compat<'a>,
    
      pub object_define_setter: Compat<'a>,
    
      pub object_entries: Compat<'a>,
    
      pub object_freeze: Compat<'a>,
    
      pub object_from_entries: Compat<'a>,
    
      pub object_get_own_property_descriptor: Compat<'a>,
    
      pub object_get_own_property_descriptors: Compat<'a>,
    
      pub object_get_own_property_names: Compat<'a>,
    
      pub object_get_own_property_symbols: Compat<'a>,
    
      pub object_get_prototype_of: Compat<'a>,
    
      pub object_group_by: Compat<'a>,
    
      pub object_has_own: Compat<'a>,
    
      pub object_hasownproperty: Compat<'a>,
    
      pub object_is: Compat<'a>,
    
      pub object_is_extensible: Compat<'a>,
    
      pub object_is_frozen: Compat<'a>,
    
      pub object_is_prototype_of: Compat<'a>,
    
      pub object_is_sealed: Compat<'a>,
    
      pub object_keys: Compat<'a>,
    
      pub object_lookup_getter: Compat<'a>,
    
      pub object_lookup_setter: Compat<'a>,
    
      pub object_prevent_extensions: Compat<'a>,
    
      pub object_prevent_extensions_es2015_behavior: Compat<'a>,
    
      pub object_property_is_enumerable: Compat<'a>,
    
      pub object_proto: Compat<'a>,
    
      pub object_seal: Compat<'a>,
    
      pub object_set_prototype_of: Compat<'a>,
    
      pub object_to_locale_string: Compat<'a>,
    
      pub object_to_string: Compat<'a>,
    
      pub object_value_of: Compat<'a>,
    
      pub object_values: Compat<'a>,
    
      pub promise: Compat<'a>,
    
      pub promise_promise: Compat<'a>,
    
      pub promise_all: Compat<'a>,
    
      pub promise_all_settled: Compat<'a>,
    
      pub promise_any: Compat<'a>,
    
      pub promise_catch: Compat<'a>,
    
      pub promise_finally: Compat<'a>,
    
      pub promise_incumbent_settings_object_tracking: Compat<'a>,
    
      pub promise_race: Compat<'a>,
    
      pub promise_reject: Compat<'a>,
    
      pub promise_resolve: Compat<'a>,
    
      pub promise_then: Compat<'a>,
    
      pub promise_try: Compat<'a>,
    
      pub promise_with_resolvers: Compat<'a>,
    
      pub promise_species_symbol: Compat<'a>,
    
      pub proxy: Compat<'a>,
    
      pub proxy_proxy: Compat<'a>,
    
      pub proxy_handler_apply: Compat<'a>,
    
      pub proxy_handler_construct: Compat<'a>,
    
      pub proxy_handler_define_property: Compat<'a>,
    
      pub proxy_handler_delete_property: Compat<'a>,
    
      pub proxy_handler_get: Compat<'a>,
    
      pub proxy_handler_get_own_property_descriptor: Compat<'a>,
    
      pub proxy_handler_get_prototype_of: Compat<'a>,
    
      pub proxy_handler_has: Compat<'a>,
    
      pub proxy_handler_is_extensible: Compat<'a>,
    
      pub proxy_handler_own_keys: Compat<'a>,
    
      pub proxy_handler_prevent_extensions: Compat<'a>,
    
      pub proxy_handler_set: Compat<'a>,
    
      pub proxy_handler_set_prototype_of: Compat<'a>,
    
      pub proxy_revocable: Compat<'a>,
    
      pub range_error: Compat<'a>,
    
      pub range_error_range_error: Compat<'a>,
    
      pub range_error_serializable_object: Compat<'a>,
    
      pub reference_error: Compat<'a>,
    
      pub reference_error_reference_error: Compat<'a>,
    
      pub reference_error_serializable_object: Compat<'a>,
    
      pub reflect: Compat<'a>,
    
      pub reflect_apply: Compat<'a>,
    
      pub reflect_construct: Compat<'a>,
    
      pub reflect_define_property: Compat<'a>,
    
      pub reflect_delete_property: Compat<'a>,
    
      pub reflect_get: Compat<'a>,
    
      pub reflect_get_own_property_descriptor: Compat<'a>,
    
      pub reflect_get_prototype_of: Compat<'a>,
    
      pub reflect_has: Compat<'a>,
    
      pub reflect_is_extensible: Compat<'a>,
    
      pub reflect_own_keys: Compat<'a>,
    
      pub reflect_prevent_extensions: Compat<'a>,
    
      pub reflect_set: Compat<'a>,
    
      pub reflect_set_prototype_of: Compat<'a>,
    
      pub regexp: Compat<'a>,
    
      pub regexp_regexp: Compat<'a>,
    
      pub regexp_compile: Compat<'a>,
    
      pub regexp_dot_all: Compat<'a>,
    
      pub regexp_exec: Compat<'a>,
    
      pub regexp_flags: Compat<'a>,
    
      pub regexp_global: Compat<'a>,
    
      pub regexp_global_prototype_accessor: Compat<'a>,
    
      pub regexp_has_indices: Compat<'a>,
    
      pub regexp_ignore_case: Compat<'a>,
    
      pub regexp_ignore_case_prototype_accessor: Compat<'a>,
    
      pub regexp_input: Compat<'a>,
    
      pub regexp_last_index: Compat<'a>,
    
      pub regexp_last_match: Compat<'a>,
    
      pub regexp_last_paren: Compat<'a>,
    
      pub regexp_left_context: Compat<'a>,
    
      pub regexp_multiline: Compat<'a>,
    
      pub regexp_multiline_prototype_accessor: Compat<'a>,
    
      pub regexp_n: Compat<'a>,
    
      pub regexp_right_context: Compat<'a>,
    
      pub regexp_source: Compat<'a>,
    
      pub regexp_source_empty_regex_string: Compat<'a>,
    
      pub regexp_source_escaping: Compat<'a>,
    
      pub regexp_source_prototype_accessor: Compat<'a>,
    
      pub regexp_sticky: Compat<'a>,
    
      pub regexp_sticky_anchored_sticky_flag: Compat<'a>,
    
      pub regexp_sticky_prototype_accessor: Compat<'a>,
    
      pub regexp_test: Compat<'a>,
    
      pub regexp_to_string: Compat<'a>,
    
      pub regexp_to_string_escaping: Compat<'a>,
    
      pub regexp_unicode: Compat<'a>,
    
      pub regexp_unicode_sets: Compat<'a>,
    
      pub regexp_match_symbol: Compat<'a>,
    
      pub regexp_match_all_symbol: Compat<'a>,
    
      pub regexp_replace_symbol: Compat<'a>,
    
      pub regexp_search_symbol: Compat<'a>,
    
      pub regexp_species_symbol: Compat<'a>,
    
      pub regexp_split_symbol: Compat<'a>,
    
      pub set: Compat<'a>,
    
      pub set_set: Compat<'a>,
    
      pub set_set_iterable_allowed: Compat<'a>,
    
      pub set_set_null_allowed: Compat<'a>,
    
      pub set_add: Compat<'a>,
    
      pub set_clear: Compat<'a>,
    
      pub set_delete: Compat<'a>,
    
      pub set_difference: Compat<'a>,
    
      pub set_entries: Compat<'a>,
    
      pub set_foreach: Compat<'a>,
    
      pub set_has: Compat<'a>,
    
      pub set_intersection: Compat<'a>,
    
      pub set_is_disjoint_from: Compat<'a>,
    
      pub set_is_subset_of: Compat<'a>,
    
      pub set_is_superset_of: Compat<'a>,
    
      pub set_key_equality_for_zeros: Compat<'a>,
    
      pub set_keys: Compat<'a>,
    
      pub set_size: Compat<'a>,
    
      pub set_symmetric_difference: Compat<'a>,
    
      pub set_union: Compat<'a>,
    
      pub set_values: Compat<'a>,
    
      pub set_iterator_symbol: Compat<'a>,
    
      pub set_species_symbol: Compat<'a>,
    
      pub shared_array_buffer: Compat<'a>,
    
      pub shared_array_buffer_shared_array_buffer: Compat<'a>,
    
      pub shared_array_buffer_shared_array_buffer_max_byte_length_option: Compat<'a>,
    
      pub shared_array_buffer_byte_length: Compat<'a>,
    
      pub shared_array_buffer_grow: Compat<'a>,
    
      pub shared_array_buffer_growable: Compat<'a>,
    
      pub shared_array_buffer_max_byte_length: Compat<'a>,
    
      pub shared_array_buffer_slice: Compat<'a>,
    
      pub shared_array_buffer_species_symbol: Compat<'a>,
    
      pub string: Compat<'a>,
    
      pub string_string: Compat<'a>,
    
      pub string_anchor: Compat<'a>,
    
      pub string_at: Compat<'a>,
    
      pub string_big: Compat<'a>,
    
      pub string_blink: Compat<'a>,
    
      pub string_bold: Compat<'a>,
    
      pub string_charat: Compat<'a>,
    
      pub string_charcodeat: Compat<'a>,
    
      pub string_codepointat: Compat<'a>,
    
      pub string_concat: Compat<'a>,
    
      pub string_endswith: Compat<'a>,
    
      pub string_fixed: Compat<'a>,
    
      pub string_fontcolor: Compat<'a>,
    
      pub string_fontsize: Compat<'a>,
    
      pub string_fromcharcode: Compat<'a>,
    
      pub string_fromcodepoint: Compat<'a>,
    
      pub string_includes: Compat<'a>,
    
      pub string_index_of: Compat<'a>,
    
      pub string_iswellformed: Compat<'a>,
    
      pub string_italics: Compat<'a>,
    
      pub string_last_index_of: Compat<'a>,
    
      pub string_length: Compat<'a>,
    
      pub string_link: Compat<'a>,
    
      pub string_localecompare: Compat<'a>,
    
      pub string_localecompare_locales_parameter: Compat<'a>,
    
      pub string_localecompare_options_parameter: Compat<'a>,
    
      pub string_match: Compat<'a>,
    
      pub string_matchall: Compat<'a>,
    
      pub string_normalize: Compat<'a>,
    
      pub string_padend: Compat<'a>,
    
      pub string_padstart: Compat<'a>,
    
      pub string_raw: Compat<'a>,
    
      pub string_repeat: Compat<'a>,
    
      pub string_replace: Compat<'a>,
    
      pub string_replaceall: Compat<'a>,
    
      pub string_search: Compat<'a>,
    
      pub string_slice: Compat<'a>,
    
      pub string_small: Compat<'a>,
    
      pub string_split: Compat<'a>,
    
      pub string_startswith: Compat<'a>,
    
      pub string_strike: Compat<'a>,
    
      pub string_sub: Compat<'a>,
    
      pub string_substr: Compat<'a>,
    
      pub string_substring: Compat<'a>,
    
      pub string_sup: Compat<'a>,
    
      pub string_tolocalelowercase: Compat<'a>,
    
      pub string_tolocalelowercase_locale: Compat<'a>,
    
      pub string_tolocaleuppercase: Compat<'a>,
    
      pub string_tolocaleuppercase_locale: Compat<'a>,
    
      pub string_tolowercase: Compat<'a>,
    
      pub string_to_string: Compat<'a>,
    
      pub string_touppercase: Compat<'a>,
    
      pub string_towellformed: Compat<'a>,
    
      pub string_trim: Compat<'a>,
    
      pub string_trimend: Compat<'a>,
    
      pub string_trimstart: Compat<'a>,
    
      pub string_unicode_code_point_escapes: Compat<'a>,
    
      pub string_value_of: Compat<'a>,
    
      pub string_iterator_symbol: Compat<'a>,
    
      pub symbol: Compat<'a>,
    
      pub symbol_symbol: Compat<'a>,
    
      pub symbol_asynciterator: Compat<'a>,
    
      pub symbol_description: Compat<'a>,
    
      pub symbol_for: Compat<'a>,
    
      pub symbol_hasinstance: Compat<'a>,
    
      pub symbol_isconcatspreadable: Compat<'a>,
    
      pub symbol_iterator: Compat<'a>,
    
      pub symbol_keyfor: Compat<'a>,
    
      pub symbol_match: Compat<'a>,
    
      pub symbol_matchall: Compat<'a>,
    
      pub symbol_replace: Compat<'a>,
    
      pub symbol_search: Compat<'a>,
    
      pub symbol_species: Compat<'a>,
    
      pub symbol_split: Compat<'a>,
    
      pub symbol_toprimitive: Compat<'a>,
    
      pub symbol_to_string: Compat<'a>,
    
      pub symbol_tostringtag: Compat<'a>,
    
      pub symbol_tostringtag_dom_objects: Compat<'a>,
    
      pub symbol_unscopables: Compat<'a>,
    
      pub symbol_value_of: Compat<'a>,
    
      pub symbol_to_primitive_symbol: Compat<'a>,
    
      pub syntaxerror: Compat<'a>,
    
      pub syntaxerror_syntaxerror: Compat<'a>,
    
      pub syntaxerror_serializable_object: Compat<'a>,
    
      pub temporal: Compat<'a>,
    
      pub typeerror: Compat<'a>,
    
      pub typeerror_typeerror: Compat<'a>,
    
      pub typeerror_serializable_object: Compat<'a>,
    
      pub typedarray: Compat<'a>,
    
      pub typedarray_bytes_per_element: Compat<'a>,
    
      pub typedarray_at: Compat<'a>,
    
      pub typedarray_buffer: Compat<'a>,
    
      pub typedarray_byte_length: Compat<'a>,
    
      pub typedarray_byte_offset: Compat<'a>,
    
      pub typedarray_constructor_without_parameters: Compat<'a>,
    
      pub typedarray_copy_with_in: Compat<'a>,
    
      pub typedarray_entries: Compat<'a>,
    
      pub typedarray_every: Compat<'a>,
    
      pub typedarray_fill: Compat<'a>,
    
      pub typedarray_filter: Compat<'a>,
    
      pub typedarray_find: Compat<'a>,
    
      pub typedarray_find_index: Compat<'a>,
    
      pub typedarray_find_last: Compat<'a>,
    
      pub typedarray_find_last_index: Compat<'a>,
    
      pub typedarray_foreach: Compat<'a>,
    
      pub typedarray_from: Compat<'a>,
    
      pub typedarray_includes: Compat<'a>,
    
      pub typedarray_index_properties_not_consulting_prototype: Compat<'a>,
    
      pub typedarray_index_of: Compat<'a>,
    
      pub typedarray_iterable_in_constructor: Compat<'a>,
    
      pub typedarray_join: Compat<'a>,
    
      pub typedarray_keys: Compat<'a>,
    
      pub typedarray_last_index_of: Compat<'a>,
    
      pub typedarray_length: Compat<'a>,
    
      pub typedarray_map: Compat<'a>,
    
      pub typedarray_name: Compat<'a>,
    
      pub typedarray_named_properties: Compat<'a>,
    
      pub typedarray_of: Compat<'a>,
    
      pub typedarray_reduce: Compat<'a>,
    
      pub typedarray_reduce_right: Compat<'a>,
    
      pub typedarray_reverse: Compat<'a>,
    
      pub typedarray_set: Compat<'a>,
    
      pub typedarray_slice: Compat<'a>,
    
      pub typedarray_some: Compat<'a>,
    
      pub typedarray_sort: Compat<'a>,
    
      pub typedarray_subarray: Compat<'a>,
    
      pub typedarray_to_locale_string: Compat<'a>,
    
      pub typedarray_to_reversed: Compat<'a>,
    
      pub typedarray_to_sorted: Compat<'a>,
    
      pub typedarray_to_string: Compat<'a>,
    
      pub typedarray_values: Compat<'a>,
    
      pub typedarray_with: Compat<'a>,
    
      pub typedarray_iterator_symbol: Compat<'a>,
    
      pub typedarray_species_symbol: Compat<'a>,
    
      pub urierror: Compat<'a>,
    
      pub urierror_urierror: Compat<'a>,
    
      pub urierror_serializable_object: Compat<'a>,
    
      pub uint16array: Compat<'a>,
    
      pub uint16array_uint16array: Compat<'a>,
    
      pub uint16array_uint16array_constructor_without_parameters: Compat<'a>,
    
      pub uint16array_uint16array_iterable_allowed: Compat<'a>,
    
      pub uint32array: Compat<'a>,
    
      pub uint32array_uint32array: Compat<'a>,
    
      pub uint32array_uint32array_constructor_without_parameters: Compat<'a>,
    
      pub uint32array_uint32array_iterable_allowed: Compat<'a>,
    
      pub uint8array: Compat<'a>,
    
      pub uint8array_uint8array: Compat<'a>,
    
      pub uint8array_uint8array_constructor_without_parameters: Compat<'a>,
    
      pub uint8array_uint8array_iterable_allowed: Compat<'a>,
    
      pub uint8clampedarray: Compat<'a>,
    
      pub uint8clampedarray_uint8clampedarray: Compat<'a>,
    
      pub uint8clampedarray_uint8clampedarray_constructor_without_parameters: Compat<'a>,
    
      pub uint8clampedarray_uint8clampedarray_iterable_allowed: Compat<'a>,
    
      pub weakmap: Compat<'a>,
    
      pub weakmap_weakmap: Compat<'a>,
    
      pub weakmap_weakmap_iterable_allowed: Compat<'a>,
    
      pub weakmap_weakmap_null_allowed: Compat<'a>,
    
      pub weakmap_delete: Compat<'a>,
    
      pub weakmap_get: Compat<'a>,
    
      pub weakmap_has: Compat<'a>,
    
      pub weakmap_set: Compat<'a>,
    
      pub weakmap_symbol_as_keys: Compat<'a>,
    
      pub weakref: Compat<'a>,
    
      pub weakref_weakref: Compat<'a>,
    
      pub weakref_deref: Compat<'a>,
    
      pub weakref_symbol_as_target: Compat<'a>,
    
      pub weakset: Compat<'a>,
    
      pub weakset_weakset: Compat<'a>,
    
      pub weakset_weakset_iterable_allowed: Compat<'a>,
    
      pub weakset_weakset_null_allowed: Compat<'a>,
    
      pub weakset_add: Compat<'a>,
    
      pub weakset_delete: Compat<'a>,
    
      pub weakset_has: Compat<'a>,
    
      pub weakset_symbol_as_keys: Compat<'a>,
    
      pub infinity: Compat<'a>,
    
      pub nan: Compat<'a>,
    
      pub decodeuri: Compat<'a>,
    
      pub decodeuricomponent: Compat<'a>,
    
      pub encodeuri: Compat<'a>,
    
      pub encodeuricomponent: Compat<'a>,
    
      pub escape: Compat<'a>,
    
      pub eval: Compat<'a>,
    
      pub globalthis: Compat<'a>,
    
      pub is_finite: Compat<'a>,
    
      pub is_nan: Compat<'a>,
    
      pub parse_float: Compat<'a>,
    
      pub parse_int: Compat<'a>,
    
      pub parse_int_leading_zero_strings_as_decimal: Compat<'a>,
    
      pub undefined: Compat<'a>,
    
      pub unescape: Compat<'a>,
    
}

pub const BUILTINS: Builtins = Builtins { 
    
  aggregate_error: Compat {
    name: "aggregate_error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AggregateError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-aggregate-error-objects",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  aggregate_error_aggregate_error: Compat {
    name: "AggregateError",
    description: r##"<code>AggregateError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/AggregateError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-aggregate-error-constructor",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  aggregate_error_errors: Compat {
    name: "errors",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/errors",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-aggregate-error",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  aggregate_error_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>AggregateError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  array: Compat {
    name: "array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  array_array: Compat {
    name: "Array",
    description: r##"<code>Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  array_at: Compat {
    name: "at",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/at",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.at",
    support: Support {
      chrome: "92",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  array_concat: Compat {
    name: "concat",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/concat",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.concat",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_copy_with_in: Compat {
    name: "copyWithin",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.copywithin",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "32",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  array_entries: Compat {
    name: "entries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/entries",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.entries",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "28",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  array_every: Compat {
    name: "every",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/every",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.every",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_fill: Compat {
    name: "fill",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/fill",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.fill",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  array_filter: Compat {
    name: "filter",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/filter",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.filter",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_find: Compat {
    name: "find",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/find",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.find",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  array_find_index: Compat {
    name: "findIndex",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.findindex",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  array_find_last: Compat {
    name: "findLast",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.findlast",
    support: Support {
      chrome: "97",
      edge: "No",
      firefox: "104",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  array_find_last_index: Compat {
    name: "findLastIndex",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.findlastindex",
    support: Support {
      chrome: "97",
      edge: "No",
      firefox: "104",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  array_flat: Compat {
    name: "flat",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/flat",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.flat",
    support: Support {
      chrome: "69",
      edge: "No",
      firefox: "62",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  array_flat_map: Compat {
    name: "flatMap",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.flatmap",
    support: Support {
      chrome: "69",
      edge: "No",
      firefox: "62",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  array_foreach: Compat {
    name: "forEach",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.foreach",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_from: Compat {
    name: "from",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/from",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.from",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "32",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  array_from_async: Compat {
    name: "fromAsync",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/fromAsync",
    spec_url: "https://tc39.es/proposal-array-from-async/#sec-array.fromAsync",
    support: Support {
      chrome: "121",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  array_includes: Compat {
    name: "includes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/includes",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.includes",
    support: Support {
      chrome: "47",
      edge: "14",
      firefox: "43",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  array_index_of: Compat {
    name: "indexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.indexof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_is_array: Compat {
    name: "isArray",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.isarray",
    support: Support {
      chrome: "4",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  array_join: Compat {
    name: "join",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/join",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.join",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_keys: Compat {
    name: "keys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/keys",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.keys",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "28",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  array_last_index_of: Compat {
    name: "lastIndexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.lastindexof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_length: Compat {
    name: "length",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/length",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-properties-of-array-instances-length",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  array_map: Compat {
    name: "map",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/map",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.map",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_of: Compat {
    name: "of",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/of",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.of",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  array_pop: Compat {
    name: "pop",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/pop",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.pop",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_push: Compat {
    name: "push",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/push",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.push",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_reduce: Compat {
    name: "reduce",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.reduce",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "3",
      ie: "9",
      node: "No",
      safari: "4",
    },
  },

  array_reduce_right: Compat {
    name: "reduceRight",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.reduceright",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "3",
      ie: "9",
      node: "No",
      safari: "4",
    },
  },

  array_reverse: Compat {
    name: "reverse",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.reverse",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_shift: Compat {
    name: "shift",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/shift",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.shift",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_slice: Compat {
    name: "slice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/slice",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.slice",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  array_some: Compat {
    name: "some",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/some",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.some",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  array_sort: Compat {
    name: "sort",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/sort",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.sort",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_sort_stable_sorting: Compat {
    name: "stable_sorting",
    description: r##"Stable sorting"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "70",
      edge: "No",
      firefox: "3",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  array_splice: Compat {
    name: "splice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/splice",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.splice",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.tolocalestring,https://tc39.es/ecma402/#sup-array.prototype.tolocalestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_to_locale_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "No",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  array_to_locale_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "No",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  array_to_reversed: Compat {
    name: "toReversed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/toReversed",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.toreversed",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  array_to_sorted: Compat {
    name: "toSorted",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.tosorted",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  array_to_spliced: Compat {
    name: "toSpliced",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.tospliced",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  array_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/toString",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  array_un_shift: Compat {
    name: "unshift",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.unshift",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  array_values: Compat {
    name: "values",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/values",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.values",
    support: Support {
      chrome: "66",
      edge: "14",
      firefox: "60",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  array_with: Compat {
    name: "with",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/with",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.with",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  array_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype-%symbol.iterator%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  array_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-array-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "No",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  array_unscopables_symbol: Compat {
    name: "@@unscopables",
    description: r##"[Symbol.unscopables]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.unscopables",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype-%symbol.unscopables%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  array_buffer: Compat {
    name: "array_buffer",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer-objects",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  array_buffer_array_buffer: Compat {
    name: "ArrayBuffer",
    description: r##"<code>ArrayBuffer()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer-constructor",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  array_buffer_array_buffer_max_byte_length_option: Compat {
    name: "maxByteLength_option",
    description: r##"<code>maxByteLength</code> option"##,
    mdn_url: "undefined",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer-constructor",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  array_buffer_byte_length: Compat {
    name: "byteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/byteLength",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-arraybuffer.prototype.bytelength",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  array_buffer_detached: Compat {
    name: "detached",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/detached",
    spec_url: "https://tc39.es/proposal-arraybuffer-transfer/#sec-get-arraybuffer.prototype.detached",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "122",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  array_buffer_is_view: Compat {
    name: "isView",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/isView",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer.isview",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "7",
    },
  },

  array_buffer_max_byte_length: Compat {
    name: "maxByteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/maxByteLength",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-arraybuffer.prototype.maxbytelength",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  array_buffer_resizable: Compat {
    name: "resizable",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resizable",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-arraybuffer.prototype.resizable",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  array_buffer_resize: Compat {
    name: "resize",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resize",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer.prototype.resize",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  array_buffer_slice: Compat {
    name: "slice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-arraybuffer.prototype.slice",
    support: Support {
      chrome: "17",
      edge: "12",
      firefox: "12",
      ie: "11",
      node: "No",
      safari: "5.1",
    },
  },

  array_buffer_transfer: Compat {
    name: "transfer",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer",
    spec_url: "https://tc39.es/proposal-arraybuffer-transfer/#sec-arraybuffer.prototype.transfer",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "122",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  array_buffer_transfer_to_fixed_length: Compat {
    name: "transferToFixedLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transferToFixedLength",
    spec_url: "https://tc39.es/proposal-arraybuffer-transfer/#sec-arraybuffer.prototype.transfertofixedlength",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "122",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  array_buffer_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-arraybuffer-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  async_function: Compat {
    name: "async_function",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-async-function-objects",
    support: Support {
      chrome: "55",
      edge: "15",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  async_function_async_function: Compat {
    name: "AsyncFunction",
    description: r##"<code>AsyncFunction()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncFunction/AsyncFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-async-function-constructor",
    support: Support {
      chrome: "55",
      edge: "15",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  async_generator: Compat {
    name: "async_generator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgenerator-objects",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_generator_next: Compat {
    name: "next",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/next",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgenerator-prototype-next",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_generator_return: Compat {
    name: "return",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/return",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgenerator-prototype-return",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_generator_throw: Compat {
    name: "throw",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/throw",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgenerator-prototype-throw",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_generator_function: Compat {
    name: "async_generator_function",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGeneratorFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgeneratorfunction-objects",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_generator_function_async_generator_function: Compat {
    name: "AsyncGeneratorFunction",
    description: r##"<code>AsyncGeneratorFunction()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncGeneratorFunction/AsyncGeneratorFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asyncgeneratorfunction-constructor",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  async_iterator: Compat {
    name: "async_iterator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncIterator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asynciteratorprototype",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "57",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  async_iterator_async_iterator_symbol: Compat {
    name: "@@asyncIterator",
    description: r##"[Symbol.asyncIterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/AsyncIterator/Symbol.asyncIterator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-asynciteratorprototype-asynciterator",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "57",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  atomics: Compat {
    name: "atomics",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics-object",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_atomic_operations_on_non_shared_buffers: Compat {
    name: "Atomic_operations_on_non_shared_buffers",
    description: r##"Atomic operations on non-shared <code>ArrayBuffer</code> objects"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  atomics_add: Compat {
    name: "add",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/add",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.add",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_and: Compat {
    name: "and",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/and",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.and",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_compare_exchange: Compat {
    name: "compareExchange",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/compareExchange",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.compareexchange",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_exchange: Compat {
    name: "exchange",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/exchange",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.exchange",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_is_lock_free: Compat {
    name: "isLockFree",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/isLockFree",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.islockfree",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_load: Compat {
    name: "load",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/load",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.load",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_notify: Compat {
    name: "notify",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/notify",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.notify",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_or: Compat {
    name: "or",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/or",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.or",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_store: Compat {
    name: "store",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/store",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.store",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_sub: Compat {
    name: "sub",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/sub",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.sub",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_wait: Compat {
    name: "wait",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.wait",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  atomics_wait_async: Compat {
    name: "waitAsync",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.waitasync",
    support: Support {
      chrome: "87",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  atomics_xor: Compat {
    name: "xor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics/xor",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-atomics.xor",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  bigint: Compat {
    name: "bigint",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint-objects",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_bigint: Compat {
    name: "BigInt",
    description: r##"<code>BigInt()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint-constructor",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_as_int_n: Compat {
    name: "asIntN",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/asIntN",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint.asintn",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_as_uint_n: Compat {
    name: "asUintN",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/asUintN",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint.asuintn",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toLocaleString",
    spec_url: "https://tc39.es/ecma402/#sup-bigint.prototype.tolocalestring",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_to_locale_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "76",
      edge: "No",
      firefox: "70",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_to_locale_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "76",
      edge: "No",
      firefox: "70",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint.prototype.tostring",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  bigint_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-bigint.prototype.valueof",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  big_int_64_array: Compat {
    name: "big_int_64_array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-objects",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  big_int_64_array_big_int_64_array: Compat {
    name: "BigInt64Array",
    description: r##"<code>BigInt64Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt64Array/BigInt64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  big_uint_64_array: Compat {
    name: "big_uint_64_array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigUint64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-objects",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  big_uint_64_array_big_uint_64_array: Compat {
    name: "BigUint64Array",
    description: r##"<code>BigUint64Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigUint64Array/BigUint64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  boolean: Compat {
    name: "boolean",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Boolean",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-boolean-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  boolean_boolean: Compat {
    name: "Boolean",
    description: r##"<code>Boolean()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Boolean/Boolean",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-boolean-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  boolean_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Boolean/toString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-boolean.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  boolean_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Boolean/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-boolean.prototype.valueof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  data_view: Compat {
    name: "data_view",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview-objects",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_data_view: Compat {
    name: "DataView",
    description: r##"<code>DataView()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/DataView",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview-constructor",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_data_view_shared_array_buffer_support: Compat {
    name: "sharedarraybuffer_support",
    description: r##"<code>SharedArrayBuffer</code> accepted as buffer"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  data_view_buffer: Compat {
    name: "buffer",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/buffer",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-dataview.prototype.buffer",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_byte_length: Compat {
    name: "byteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteLength",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-dataview.prototype.bytelength",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_byte_offset: Compat {
    name: "byteOffset",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteOffset",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-dataview.prototype.byteoffset",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_big_int_64: Compat {
    name: "getBigInt64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getbigint64",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  data_view_get_big_uint_64: Compat {
    name: "getBigUint64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigUint64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getbiguint64",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  data_view_get_float_16: Compat {
    name: "getFloat16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat16",
    spec_url: "https://tc39.es/proposal-float16array/#sec-dataview.prototype.getfloat16",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  data_view_get_float_32: Compat {
    name: "getFloat32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getfloat32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_float_64: Compat {
    name: "getFloat64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getfloat64",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_int_16: Compat {
    name: "getInt16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getint16",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_int_32: Compat {
    name: "getInt32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getint32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_int_8: Compat {
    name: "getInt8",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getint8",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_uint_16: Compat {
    name: "getUint16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getuint16",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_uint_32: Compat {
    name: "getUint32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getuint32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_get_uint_8: Compat {
    name: "getUint8",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint8",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.getuint8",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_big_int_64: Compat {
    name: "setBigInt64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigInt64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setbigint64",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  data_view_set_big_uint_64: Compat {
    name: "setBigUint64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigUint64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setbiguint64",
    support: Support {
      chrome: "67",
      edge: "No",
      firefox: "68",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  data_view_set_float_16: Compat {
    name: "setFloat16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat16",
    spec_url: "https://tc39.es/proposal-float16array/#sec-dataview.prototype.setfloat16",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  data_view_set_float_32: Compat {
    name: "setFloat32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setfloat32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_float_64: Compat {
    name: "setFloat64",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat64",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setfloat64",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_int_16: Compat {
    name: "setInt16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt16",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setint16",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_int_32: Compat {
    name: "setInt32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setint32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_int_8: Compat {
    name: "setInt8",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt8",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setint8",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_uint_16: Compat {
    name: "setUint16",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint16",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setuint16",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_uint_32: Compat {
    name: "setUint32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint32",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setuint32",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  data_view_set_uint_8: Compat {
    name: "setUint8",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint8",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-dataview.prototype.setuint8",
    support: Support {
      chrome: "9",
      edge: "12",
      firefox: "15",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  date: Compat {
    name: "date",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_date: Compat {
    name: "Date",
    description: r##"<code>Date()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/Date",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_utc: Compat {
    name: "UTC",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.utc",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_utc_optional_month_index: Compat {
    name: "optional_monthIndex",
    description: r##"<code>monthIndex</code> defaults to 0"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "57",
      edge: "No",
      firefox: "54",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  date_get_date: Compat {
    name: "getDate",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getdate",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_get_day: Compat {
    name: "getDay",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getday",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_get_full_year: Compat {
    name: "getFullYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getfullyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_hours: Compat {
    name: "getHours",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.gethours",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_get_milliseconds: Compat {
    name: "getMilliseconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getmilliseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_minutes: Compat {
    name: "getMinutes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getminutes",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_month: Compat {
    name: "getMonth",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getmonth",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_seconds: Compat {
    name: "getSeconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_time: Compat {
    name: "getTime",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.gettime",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_timezone_offset: Compat {
    name: "getTimezoneOffset",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.gettimezoneoffset",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_date: Compat {
    name: "getUTCDate",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcdate",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_day: Compat {
    name: "getUTCDay",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcday",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_full_year: Compat {
    name: "getUTCFullYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcfullyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_hours: Compat {
    name: "getUTCHours",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutchours",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_milliseconds: Compat {
    name: "getUTCMilliseconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcmilliseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_minutes: Compat {
    name: "getUTCMinutes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcminutes",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_month: Compat {
    name: "getUTCMonth",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcmonth",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_utc_seconds: Compat {
    name: "getUTCSeconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.getutcseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_get_year: Compat {
    name: "getYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getYear",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-date.prototype.getyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_now: Compat {
    name: "now",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/now",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.now",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "9",
      node: "No",
      safari: "4",
    },
  },

  date_parse: Compat {
    name: "parse",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/parse",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.parse",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_parse_iso_8601: Compat {
    name: "iso_8601",
    description: r##"ISO 8601 format"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  date_set_date: Compat {
    name: "setDate",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setdate",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_full_year: Compat {
    name: "setFullYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setfullyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_hours: Compat {
    name: "setHours",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.sethours",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_milliseconds: Compat {
    name: "setMilliseconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setmilliseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_minutes: Compat {
    name: "setMinutes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setminutes",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_month: Compat {
    name: "setMonth",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setmonth",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_seconds: Compat {
    name: "setSeconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_time: Compat {
    name: "setTime",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.settime",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_date: Compat {
    name: "setUTCDate",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcdate",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_full_year: Compat {
    name: "setUTCFullYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcfullyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_hours: Compat {
    name: "setUTCHours",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutchours",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_milliseconds: Compat {
    name: "setUTCMilliseconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcmilliseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_minutes: Compat {
    name: "setUTCMinutes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcminutes",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_month: Compat {
    name: "setUTCMonth",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcmonth",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_utc_seconds: Compat {
    name: "setUTCSeconds",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.setutcseconds",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_set_year: Compat {
    name: "setYear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/setYear",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-date.prototype.setyear",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_to_date_string: Compat {
    name: "toDateString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.todatestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  date_to_gmt_string: Compat {
    name: "toGMTString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-date.prototype.togmtstring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_to_iso_string: Compat {
    name: "toISOString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.toisostring",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "1",
      ie: "9",
      node: "No",
      safari: "4",
    },
  },

  date_to_json: Compat {
    name: "toJSON",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.tojson",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "1",
      ie: "8",
      node: "No",
      safari: "4",
    },
  },

  date_to_locale_date_string: Compat {
    name: "toLocaleDateString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.tolocaledatestring,https://tc39.es/ecma402/#sup-date.prototype.tolocaledatestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  date_to_locale_date_string_iana_time_zone_names: Compat {
    name: "iana_time_zone_names",
    description: r##"IANA time zone names in <code>timeZone</code> option"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  date_to_locale_date_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_locale_date_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.tolocalestring,https://tc39.es/ecma402/#sup-date.prototype.tolocalestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_to_locale_string_iana_time_zone_names: Compat {
    name: "iana_time_zone_names",
    description: r##"IANA time zone names in <code>timeZone</code> option"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  date_to_locale_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_locale_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_locale_time_string: Compat {
    name: "toLocaleTimeString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.tolocaletimestring,https://tc39.es/ecma402/#sup-date.prototype.tolocaletimestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  date_to_locale_time_string_iana_time_zone_names: Compat {
    name: "iana_time_zone_names",
    description: r##"IANA time zone names in <code>timeZone</code> option"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  date_to_locale_time_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_locale_time_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  date_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  date_to_time_string: Compat {
    name: "toTimeString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.totimestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  date_to_utc_string: Compat {
    name: "toUTCString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.toutcstring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype.valueof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  date_to_primitive_symbol: Compat {
    name: "@@toPrimitive",
    description: r##"[Symbol.toPrimitive]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/Symbol.toPrimitive",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-date.prototype-%symbol.toprimitive%",
    support: Support {
      chrome: "47",
      edge: "15",
      firefox: "44",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  error: Compat {
    name: "error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  error_error: Compat {
    name: "Error",
    description: r##"<code>Error()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/Error",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  error_error_filename_parameter: Compat {
    name: "fileName_parameter",
    description: r##"<code>fileName</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_error_linenumber_parameter: Compat {
    name: "lineNumber_parameter",
    description: r##"<code>lineNumber</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_error_options_cause_parameter: Compat {
    name: "options_cause_parameter",
    description: r##"<code>options.cause</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-installerrorcause",
    support: Support {
      chrome: "93",
      edge: "No",
      firefox: "91",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  error_cause: Compat {
    name: "cause",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/cause",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-installerrorcause",
    support: Support {
      chrome: "93",
      edge: "No",
      firefox: "91",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  error_cause_displayed_in_console: Compat {
    name: "displayed_in_console",
    description: r##"Cause is displayed in console"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "91",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_column_number: Compat {
    name: "columnNumber",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/columnNumber",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_filename: Compat {
    name: "fileName",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/fileName",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_linenumber: Compat {
    name: "lineNumber",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/lineNumber",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_message: Compat {
    name: "message",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/message",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error.prototype.message",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  error_name: Compat {
    name: "name",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/name",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error.prototype.name",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  error_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>Error</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  error_stack: Compat {
    name: "stack",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/stack",
    spec_url: "undefined",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "1",
      ie: "10",
      node: "No",
      safari: "6",
    },
  },

  error_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Error/toString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  eval_error: Compat {
    name: "eval_error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/EvalError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-evalerror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  eval_error_eval_error: Compat {
    name: "EvalError",
    description: r##"<code>EvalError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/EvalError/EvalError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  eval_error_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>EvalError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  finalization_registry: Compat {
    name: "finalization_registry",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-finalization-registry-objects",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  finalization_registry_finalization_registry: Compat {
    name: "FinalizationRegistry",
    description: r##"<code>FinalizationRegistry()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/FinalizationRegistry",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-finalization-registry-constructor",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  finalization_registry_register: Compat {
    name: "register",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/register",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-finalization-registry.prototype.register",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  finalization_registry_symbol_as_target: Compat {
    name: "symbol_as_target",
    description: r##"Non-registered symbol as target"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "109",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  finalization_registry_unregister: Compat {
    name: "unregister",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/unregister",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-finalization-registry.prototype.unregister",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  float16array: Compat {
    name: "float16array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float16Array",
    spec_url: "https://tc39.es/proposal-float16array/#sec-float16array",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  float16array_float16array: Compat {
    name: "Float16Array",
    description: r##"<code>Float16Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float16Array/Float16Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  float32array: Compat {
    name: "float32array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float32array_float32array: Compat {
    name: "Float32Array",
    description: r##"<code>Float32Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float32Array/Float32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float32array_float32array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float32array_float32array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Float32Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  float64array: Compat {
    name: "float64array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float64array_float64array: Compat {
    name: "Float64Array",
    description: r##"<code>Float64Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Float64Array/Float64Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float64array_float64array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  float64array_float64array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Float64Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  function: Compat {
    name: "function",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  function_function: Compat {
    name: "Function",
    description: r##"<code>Function()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/Function",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  function_apply: Compat {
    name: "apply",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/apply",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function.prototype.apply",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  function_apply_generic_arrays_as_arguments: Compat {
    name: "generic_arrays_as_arguments",
    description: r##"ES 5.1: generic array-like object as <code>arguments</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "17",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "6",
    },
  },

  function_arguments: Compat {
    name: "arguments",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/arguments",
    spec_url: "undefined",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  function_bind: Compat {
    name: "bind",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/bind",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function.prototype.bind",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  function_call: Compat {
    name: "call",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/call",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function.prototype.call",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  function_caller: Compat {
    name: "caller",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/caller",
    spec_url: "undefined",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "8",
      node: "No",
      safari: "3",
    },
  },

  function_display_name: Compat {
    name: "displayName",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/displayName",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "13",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  function_length: Compat {
    name: "length",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/length",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function-instances-length",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  function_length_configurable_true: Compat {
    name: "configurable_true",
    description: r##"Configurable: true"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "43",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  function_name: Compat {
    name: "name",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/name",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function-instances-name",
    support: Support {
      chrome: "15",
      edge: "14",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "6",
    },
  },

  function_name_configurable_true: Compat {
    name: "configurable_true",
    description: r##"Configurable: true"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "43",
      edge: "14",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  function_name_inferred_names: Compat {
    name: "inferred_names",
    description: r##"Inferred names on anonymous functions"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "51",
      edge: "79",
      firefox: "53",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  function_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/toString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5",
      node: "No",
      safari: "1",
    },
  },

  function_to_string_to_string_revision: Compat {
    name: "toString_revision",
    description: r##"Implements <code>Function.prototype.toString</code> revision"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "66",
      edge: "No",
      firefox: "54",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  function_has_instance_symbol: Compat {
    name: "@@hasInstance",
    description: r##"[Symbol.hasInstance]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/Symbol.hasInstance",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-function.prototype-%symbol.hasinstance%",
    support: Support {
      chrome: "50",
      edge: "15",
      firefox: "50",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator: Compat {
    name: "generator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Generator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generator-objects",
    support: Support {
      chrome: "39",
      edge: "13",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_next: Compat {
    name: "next",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Generator/next",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generator.prototype.next",
    support: Support {
      chrome: "39",
      edge: "13",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_return: Compat {
    name: "return",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Generator/return",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generator.prototype.return",
    support: Support {
      chrome: "50",
      edge: "13",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_throw: Compat {
    name: "throw",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Generator/throw",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generator.prototype.throw",
    support: Support {
      chrome: "39",
      edge: "13",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_function: Compat {
    name: "generator_function",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/GeneratorFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generatorfunction-objects",
    support: Support {
      chrome: "39",
      edge: "13",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_function_generator_function: Compat {
    name: "GeneratorFunction",
    description: r##"<code>GeneratorFunction()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/GeneratorFunction/GeneratorFunction",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-generatorfunction-constructor",
    support: Support {
      chrome: "39",
      edge: "13",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  int16array: Compat {
    name: "int16array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int16Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int16array_int16array: Compat {
    name: "Int16Array",
    description: r##"<code>Int16Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int16Array/Int16Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int16array_int16array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int16array_int16array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Int16Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  int32array: Compat {
    name: "int32array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int32array_int32array: Compat {
    name: "Int32Array",
    description: r##"<code>Int32Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int32Array/Int32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int32array_int32array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int32array_int32array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Int32Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  int8array: Compat {
    name: "int8array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int8Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int8array_int8array: Compat {
    name: "Int8Array",
    description: r##"<code>Int8Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Int8Array/Int8Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int8array_int8array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  int8array_int8array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Int8Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  internal_error: Compat {
    name: "internal_error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/InternalError",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  internal_error_internal_error: Compat {
    name: "InternalError",
    description: r##"<code>InternalError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/InternalError/InternalError",
    spec_url: "undefined",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  intl: Compat {
    name: "intl",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Intl",
    spec_url: "https://tc39.es/ecma402/#intl-object",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  intl_get_canonical_locales: Compat {
    name: "getCanonicalLocales",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Intl/getCanonicalLocales",
    spec_url: "https://tc39.es/ecma402/#sec-intl.getcanonicallocales",
    support: Support {
      chrome: "54",
      edge: "16",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  intl_supported_values_of: Compat {
    name: "supportedValuesOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Intl/supportedValuesOf",
    spec_url: "https://tc39.es/ecma402/#sec-intl.supportedvaluesof",
    support: Support {
      chrome: "99",
      edge: "No",
      firefox: "93",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  iterator: Compat {
    name: "iterator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-%iteratorprototype%-object",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "17",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  iterator_iterator: Compat {
    name: "Iterator",
    description: r##"<code>Iterator()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/Iterator",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iterator-constructor",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_drop: Compat {
    name: "drop",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/drop",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.drop",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_every: Compat {
    name: "every",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/every",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.every",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_filter: Compat {
    name: "filter",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/filter",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.filter",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_find: Compat {
    name: "find",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/find",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.find",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_flat_map: Compat {
    name: "flatMap",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/flatMap",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.flatmap",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_foreach: Compat {
    name: "forEach",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/forEach",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.foreach",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_from: Compat {
    name: "from",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/from",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iterator.from",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_map: Compat {
    name: "map",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/map",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.map",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_reduce: Compat {
    name: "reduce",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/reduce",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.reduce",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_some: Compat {
    name: "some",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/some",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.some",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_take: Compat {
    name: "take",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/take",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.take",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_to_array: Compat {
    name: "toArray",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/toArray",
    spec_url: "https://tc39.es/proposal-iterator-helpers/#sec-iteratorprototype.toarray",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  iterator_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Iterator/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-%iteratorprototype%-%symbol.iterator%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  json: Compat {
    name: "json",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-json-object",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "3.5",
      ie: "8",
      node: "No",
      safari: "4",
    },
  },

  json_is_raw_json: Compat {
    name: "isRawJSON",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON/isRawJSON",
    spec_url: "https://tc39.es/proposal-json-parse-with-source/#sec-json.israwjson",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  json_json_superset: Compat {
    name: "json_superset",
    description: r##"JavaScript is a superset of JSON"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON#JavaScript_and_JSON_differences",
    spec_url: "undefined",
    support: Support {
      chrome: "66",
      edge: "No",
      firefox: "62",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  json_parse: Compat {
    name: "parse",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-json.parse",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "3.5",
      ie: "8",
      node: "No",
      safari: "4",
    },
  },

  json_parse_reviver_parameter_context_argument: Compat {
    name: "reviver_parameter_context_argument",
    description: r##"Reviver has <code>context</code> parameter"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse#the_reviver_parameter",
    spec_url: "https://tc39.es/proposal-json-parse-with-source/#sec-internalizejsonproperty",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  json_raw_json: Compat {
    name: "rawJSON",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON/rawJSON",
    spec_url: "https://tc39.es/proposal-json-parse-with-source/#sec-json.rawjson",
    support: Support {
      chrome: "114",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  json_stringify: Compat {
    name: "stringify",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-json.stringify",
    support: Support {
      chrome: "3",
      edge: "12",
      firefox: "3.5",
      ie: "8",
      node: "No",
      safari: "4",
    },
  },

  json_stringify_well_formed_stringify: Compat {
    name: "well_formed_stringify",
    description: r##"Strings are escaped to well-formed UTF-8"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "72",
      edge: "No",
      firefox: "64",
      ie: "false",
      node: "No",
      safari: "12.1",
    },
  },

  map: Compat {
    name: "map",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map-objects",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_map: Compat {
    name: "Map",
    description: r##"<code>Map()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/Map",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map-constructor",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_map_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Map(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  map_map_null_allowed: Compat {
    name: "null_allowed",
    description: r##"<code>new Map(null)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "37",
      ie: "11",
      node: "No",
      safari: "9",
    },
  },

  map_clear: Compat {
    name: "clear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/clear",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.clear",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "19",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_delete: Compat {
    name: "delete",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/delete",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.delete",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_entries: Compat {
    name: "entries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/entries",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.entries",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "20",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  map_foreach: Compat {
    name: "forEach",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.foreach",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_get: Compat {
    name: "get",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/get",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.get",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_group_by: Compat {
    name: "groupBy",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/groupBy",
    spec_url: "https://tc39.es/proposal-array-grouping/#sec-map.groupby",
    support: Support {
      chrome: "117",
      edge: "No",
      firefox: "119",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  map_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/has",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.has",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_key_equality_for_zeros: Compat {
    name: "key_equality_for_zeros",
    description: r##"Key equality for -0 and 0"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  map_keys: Compat {
    name: "keys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/keys",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.keys",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "20",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  map_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/set",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.set",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_size: Compat {
    name: "size",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/size",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-get-map.prototype.size",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "19",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  map_values: Compat {
    name: "values",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/values",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype.values",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "20",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  map_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-map.prototype-%symbol.iterator%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  map_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-get-map-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  math: Compat {
    name: "math",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math-object",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_e: Compat {
    name: "E",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/E",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.e",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_ln10: Compat {
    name: "LN10",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/LN10",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.ln10",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_ln2: Compat {
    name: "LN2",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/LN2",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.ln2",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_log10e: Compat {
    name: "LOG10E",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/LOG10E",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log10e",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_log2e: Compat {
    name: "LOG2E",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/LOG2E",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log2e",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_pi: Compat {
    name: "PI",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/PI",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.pi",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_sqrt1_2: Compat {
    name: "SQRT1_2",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/SQRT1_2",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sqrt1_2",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_sqrt2: Compat {
    name: "SQRT2",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/SQRT2",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sqrt2",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_abs: Compat {
    name: "abs",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/abs",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.abs",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_acos: Compat {
    name: "acos",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/acos",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.acos",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_acosh: Compat {
    name: "acosh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/acosh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.acosh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_asin: Compat {
    name: "asin",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/asin",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.asin",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_asinh: Compat {
    name: "asinh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/asinh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.asinh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_atan: Compat {
    name: "atan",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/atan",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.atan",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_atan2: Compat {
    name: "atan2",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.atan2",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  math_atanh: Compat {
    name: "atanh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/atanh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.atanh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_cbrt: Compat {
    name: "cbrt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/cbrt",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.cbrt",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_ceil: Compat {
    name: "ceil",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/ceil",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.ceil",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_clz32: Compat {
    name: "clz32",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/clz32",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.clz32",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  math_cos: Compat {
    name: "cos",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/cos",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.cos",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_cosh: Compat {
    name: "cosh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/cosh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.cosh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_exp: Compat {
    name: "exp",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/exp",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.exp",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_expm1: Compat {
    name: "expm1",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/expm1",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.expm1",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_f16round: Compat {
    name: "f16round",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/f16round",
    spec_url: "https://tc39.es/proposal-float16array/#sec-math.f16round",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  math_floor: Compat {
    name: "floor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/floor",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.floor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_fround: Compat {
    name: "fround",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/fround",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.fround",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_hypot: Compat {
    name: "hypot",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/hypot",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.hypot",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "27",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_imul: Compat {
    name: "imul",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/imul",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.imul",
    support: Support {
      chrome: "28",
      edge: "12",
      firefox: "20",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  math_log: Compat {
    name: "log",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/log",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_log10: Compat {
    name: "log10",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/log10",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log10",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_log1p: Compat {
    name: "log1p",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/log1p",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log1p",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_log2: Compat {
    name: "log2",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/log2",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.log2",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_max: Compat {
    name: "max",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/max",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.max",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_min: Compat {
    name: "min",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/min",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.min",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_pow: Compat {
    name: "pow",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/pow",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.pow",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_random: Compat {
    name: "random",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/random",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.random",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_round: Compat {
    name: "round",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/round",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.round",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_sign: Compat {
    name: "sign",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/sign",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sign",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  math_sin: Compat {
    name: "sin",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/sin",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sin",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_sinh: Compat {
    name: "sinh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/sinh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sinh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_sqrt: Compat {
    name: "sqrt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/sqrt",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.sqrt",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_tan: Compat {
    name: "tan",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/tan",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.tan",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  math_tanh: Compat {
    name: "tanh",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/tanh",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.tanh",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  math_trunc: Compat {
    name: "trunc",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-math.trunc",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  number: Compat {
    name: "number",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  number_epsilon: Compat {
    name: "EPSILON",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/EPSILON",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.epsilon",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_max_safe_integer: Compat {
    name: "MAX_SAFE_INTEGER",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.max_safe_integer",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_max_value: Compat {
    name: "MAX_VALUE",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_VALUE",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.max_value",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  number_min_safe_integer: Compat {
    name: "MIN_SAFE_INTEGER",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/MIN_SAFE_INTEGER",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.min_safe_integer",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_min_value: Compat {
    name: "MIN_VALUE",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/MIN_VALUE",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.min_value",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  number_nan: Compat {
    name: "NaN",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/NaN",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.nan",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  number_negative_infinity: Compat {
    name: "NEGATIVE_INFINITY",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/NEGATIVE_INFINITY",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.negative_infinity",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  number_number: Compat {
    name: "Number",
    description: r##"<code>Number()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/Number",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  number_positive_infinity: Compat {
    name: "POSITIVE_INFINITY",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/POSITIVE_INFINITY",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.positive_infinity",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  number_is_finite: Compat {
    name: "isFinite",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/isFinite",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.isfinite",
    support: Support {
      chrome: "19",
      edge: "12",
      firefox: "16",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_is_integer: Compat {
    name: "isInteger",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/isInteger",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.isinteger",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "16",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_is_nan: Compat {
    name: "isNaN",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/isNaN",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.isnan",
    support: Support {
      chrome: "25",
      edge: "12",
      firefox: "15",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_is_safe_integer: Compat {
    name: "isSafeInteger",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/isSafeInteger",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.issafeinteger",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "32",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_parse_float: Compat {
    name: "parseFloat",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/parseFloat",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.parsefloat",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_parse_int: Compat {
    name: "parseInt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/parseInt",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.parseint",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "25",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  number_to_exponential: Compat {
    name: "toExponential",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/toExponential",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.toexponential",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "2",
    },
  },

  number_to_fixed: Compat {
    name: "toFixed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.tofixed",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "2",
    },
  },

  number_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.tolocalestring,https://tc39.es/ecma402/#sup-number.prototype.tolocalestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5",
      node: "No",
      safari: "1",
    },
  },

  number_to_locale_string_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  number_to_locale_string_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  number_to_precision: Compat {
    name: "toPrecision",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.toprecision",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "2",
    },
  },

  number_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/toString",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  number_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Number/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.valueof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  object: Compat {
    name: "object",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  object_object: Compat {
    name: "Object",
    description: r##"<code>Object()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/Object",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  object_assign: Compat {
    name: "assign",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/assign",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.assign",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  object_constructor: Compat {
    name: "constructor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "8",
      node: "No",
      safari: "1",
    },
  },

  object_create: Compat {
    name: "create",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/create",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.create",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_define_getter: Compat {
    name: "defineGetter",
    description: r##"<code>__defineGetter__</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineGetter__",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.__defineGetter__",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "11",
      node: "No",
      safari: "3",
    },
  },

  object_define_properties: Compat {
    name: "defineProperties",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperties",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.defineproperties",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_define_property: Compat {
    name: "defineProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.defineproperty",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_define_setter: Compat {
    name: "defineSetter",
    description: r##"<code>__defineSetter__</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineSetter__",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.__defineSetter__",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "11",
      node: "No",
      safari: "3",
    },
  },

  object_entries: Compat {
    name: "entries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/entries",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.entries",
    support: Support {
      chrome: "54",
      edge: "14",
      firefox: "47",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  object_freeze: Compat {
    name: "freeze",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/freeze",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.freeze",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_from_entries: Compat {
    name: "fromEntries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/fromEntries",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.fromentries",
    support: Support {
      chrome: "73",
      edge: "No",
      firefox: "63",
      ie: "false",
      node: "No",
      safari: "12.1",
    },
  },

  object_get_own_property_descriptor: Compat {
    name: "getOwnPropertyDescriptor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptor",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.getownpropertydescriptor",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_get_own_property_descriptors: Compat {
    name: "getOwnPropertyDescriptors",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptors",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.getownpropertydescriptors",
    support: Support {
      chrome: "54",
      edge: "15",
      firefox: "50",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  object_get_own_property_names: Compat {
    name: "getOwnPropertyNames",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyNames",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.getownpropertynames",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_get_own_property_symbols: Compat {
    name: "getOwnPropertySymbols",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertySymbols",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.getownpropertysymbols",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  object_get_prototype_of: Compat {
    name: "getPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/getPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.getprototypeof",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "3.5",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_group_by: Compat {
    name: "groupBy",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/groupBy",
    spec_url: "https://tc39.es/proposal-array-grouping/#sec-object.groupby",
    support: Support {
      chrome: "117",
      edge: "No",
      firefox: "119",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  object_has_own: Compat {
    name: "hasOwn",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.hasown",
    support: Support {
      chrome: "93",
      edge: "No",
      firefox: "92",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  object_hasownproperty: Compat {
    name: "hasOwnProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.hasownproperty",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  object_is: Compat {
    name: "is",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/is",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.is",
    support: Support {
      chrome: "19",
      edge: "12",
      firefox: "22",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  object_is_extensible: Compat {
    name: "isExtensible",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/isExtensible",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.isextensible",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_is_frozen: Compat {
    name: "isFrozen",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/isFrozen",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.isfrozen",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_is_prototype_of: Compat {
    name: "isPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.isprototypeof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  object_is_sealed: Compat {
    name: "isSealed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/isSealed",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.issealed",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_keys: Compat {
    name: "keys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/keys",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.keys",
    support: Support {
      chrome: "5",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5",
    },
  },

  object_lookup_getter: Compat {
    name: "lookupGetter",
    description: r##"<code>__lookupGetter__</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupGetter__",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.__lookupGetter__",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "11",
      node: "No",
      safari: "3",
    },
  },

  object_lookup_setter: Compat {
    name: "lookupSetter",
    description: r##"<code>__lookupSetter__</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupSetter__",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.__lookupSetter__",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "11",
      node: "No",
      safari: "3",
    },
  },

  object_prevent_extensions: Compat {
    name: "preventExtensions",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/preventExtensions",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.preventextensions",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_prevent_extensions_es2015_behavior: Compat {
    name: "ES2015_behavior",
    description: r##"ES2015 behavior for non-object argument"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "44",
      edge: "12",
      firefox: "35",
      ie: "11",
      node: "No",
      safari: "9",
    },
  },

  object_property_is_enumerable: Compat {
    name: "propertyIsEnumerable",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.propertyisenumerable",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  object_proto: Compat {
    name: "proto",
    description: r##"<code>__proto__</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/proto",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.__proto__",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "11",
      node: "No",
      safari: "3",
    },
  },

  object_seal: Compat {
    name: "seal",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/seal",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.seal",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "4",
      ie: "9",
      node: "No",
      safari: "5.1",
    },
  },

  object_set_prototype_of: Compat {
    name: "setPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.setprototypeof",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "31",
      ie: "11",
      node: "No",
      safari: "9",
    },
  },

  object_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.tolocalestring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  object_to_string: Compat {
    name: "toString",
    description: r##"<code>toString()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/toString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  object_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.prototype.valueof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  object_values: Compat {
    name: "values",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Object/values",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-object.values",
    support: Support {
      chrome: "54",
      edge: "14",
      firefox: "47",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  promise: Compat {
    name: "promise",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise-objects",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_promise: Compat {
    name: "Promise",
    description: r##"<code>Promise()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/Promise",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise-constructor",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_all: Compat {
    name: "all",
    description: r##"<code>all()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/all",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.all",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_all_settled: Compat {
    name: "allSettled",
    description: r##"<code>allSettled()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/allSettled",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.allsettled",
    support: Support {
      chrome: "76",
      edge: "No",
      firefox: "71",
      ie: "false",
      node: "No",
      safari: "13",
    },
  },

  promise_any: Compat {
    name: "any",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/any",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.any",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  promise_catch: Compat {
    name: "catch",
    description: r##"<code>catch()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/catch",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.prototype.catch",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_finally: Compat {
    name: "finally",
    description: r##"<code>finally()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/finally",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.prototype.finally",
    support: Support {
      chrome: "63",
      edge: "18",
      firefox: "58",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  promise_incumbent_settings_object_tracking: Compat {
    name: "incumbent_settings_object_tracking",
    description: r##"Incumbent settings object tracking"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise#Incumbent_settings_object_tracking",
    spec_url: "https://html.spec.whatwg.org/multipage/webappapis.html#incumbent-settings-object-tracking-in-promises",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "50",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  promise_race: Compat {
    name: "race",
    description: r##"<code>race()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/race",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.race",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_reject: Compat {
    name: "reject",
    description: r##"<code>reject()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/reject",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.reject",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_resolve: Compat {
    name: "resolve",
    description: r##"<code>resolve()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.resolve",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_then: Compat {
    name: "then",
    description: r##"<code>then()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/then",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-promise.prototype.then",
    support: Support {
      chrome: "32",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  promise_try: Compat {
    name: "try",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/try",
    spec_url: "https://tc39.es/proposal-promise-try/#sec-promise.try",
    support: Support {
      chrome: "128",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  promise_with_resolvers: Compat {
    name: "withResolvers",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers",
    spec_url: "https://tc39.es/proposal-promise-with-resolvers/#sec-promise.withResolvers",
    support: Support {
      chrome: "119",
      edge: "No",
      firefox: "121",
      ie: "false",
      node: "No",
      safari: "17.4",
    },
  },

  promise_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/control-abstraction-objects.html#sec-get-promise-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "No",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy: Compat {
    name: "proxy",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-proxy-objects",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_proxy: Compat {
    name: "Proxy",
    description: r##"<code>Proxy()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-proxy-constructor",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_apply: Compat {
    name: "apply",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/apply",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-call-thisargument-argumentslist",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_construct: Compat {
    name: "construct",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/construct",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-construct-argumentslist-newtarget",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_define_property: Compat {
    name: "defineProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/defineProperty",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-defineownproperty-p-desc",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_delete_property: Compat {
    name: "deleteProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/deleteProperty",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-delete-p",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_get: Compat {
    name: "get",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/get",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-get-p-receiver",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_get_own_property_descriptor: Compat {
    name: "getOwnPropertyDescriptor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/getOwnPropertyDescriptor",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-getownproperty-p",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_get_prototype_of: Compat {
    name: "getPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/getPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-getprototypeof",
    support: Support {
      chrome: "49",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/has",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-hasproperty-p",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_is_extensible: Compat {
    name: "isExtensible",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/isExtensible",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-isextensible",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_own_keys: Compat {
    name: "ownKeys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/ownKeys",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-ownpropertykeys",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_prevent_extensions: Compat {
    name: "preventExtensions",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/preventExtensions",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-preventextensions",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "22",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/set",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-set-p-v-receiver",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "18",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_handler_set_prototype_of: Compat {
    name: "setPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy/setPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-proxy-object-internal-methods-and-internal-slots-setprototypeof-v",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  proxy_revocable: Compat {
    name: "revocable",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-proxy.revocable",
    support: Support {
      chrome: "63",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  range_error: Compat {
    name: "range_error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RangeError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-rangeerror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  range_error_range_error: Compat {
    name: "RangeError",
    description: r##"<code>RangeError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RangeError/RangeError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  range_error_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>RangeError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  reference_error: Compat {
    name: "reference_error",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-referenceerror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  reference_error_reference_error: Compat {
    name: "ReferenceError",
    description: r##"<code>ReferenceError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError/ReferenceError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  reference_error_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>ReferenceError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  reflect: Compat {
    name: "reflect",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect-object",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_apply: Compat {
    name: "apply",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.apply",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_construct: Compat {
    name: "construct",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.construct",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_define_property: Compat {
    name: "defineProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.defineproperty",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_delete_property: Compat {
    name: "deleteProperty",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.deleteproperty",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_get: Compat {
    name: "get",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.get",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_get_own_property_descriptor: Compat {
    name: "getOwnPropertyDescriptor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.getownpropertydescriptor",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_get_prototype_of: Compat {
    name: "getPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.getprototypeof",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.has",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_is_extensible: Compat {
    name: "isExtensible",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.isextensible",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_own_keys: Compat {
    name: "ownKeys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.ownkeys",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_prevent_extensions: Compat {
    name: "preventExtensions",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.preventextensions",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.set",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  reflect_set_prototype_of: Compat {
    name: "setPrototypeOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf",
    spec_url: "https://tc39.es/ecma262/multipage/reflection.html#sec-reflect.setprototypeof",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "42",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp: Compat {
    name: "regexp",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp-regular-expression-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_regexp: Compat {
    name: "RegExp",
    description: r##"<code>RegExp()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/RegExp",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_compile: Compat {
    name: "compile",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/compile",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-regexp.prototype.compile",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "3.1",
    },
  },

  regexp_dot_all: Compat {
    name: "dotAll",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/dotAll",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.dotAll",
    support: Support {
      chrome: "62",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  regexp_exec: Compat {
    name: "exec",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype.exec",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_flags: Compat {
    name: "flags",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/flags",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.flags",
    support: Support {
      chrome: "49",
      edge: "No",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  regexp_global: Compat {
    name: "global",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/global",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.global",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  regexp_global_prototype_accessor: Compat {
    name: "prototype_accessor",
    description: r##"Prototype accessor property (ES2015)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "48",
      edge: "12",
      firefox: "38",
      ie: "5.5",
      node: "No",
      safari: "1.3",
    },
  },

  regexp_has_indices: Compat {
    name: "hasIndices",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/hasIndices",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.hasIndices",
    support: Support {
      chrome: "90",
      edge: "No",
      firefox: "88",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  regexp_ignore_case: Compat {
    name: "ignoreCase",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/ignoreCase",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.ignorecase",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  regexp_ignore_case_prototype_accessor: Compat {
    name: "prototype_accessor",
    description: r##"Prototype accessor property (ES2015)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "48",
      edge: "12",
      firefox: "38",
      ie: "5.5",
      node: "No",
      safari: "1.3",
    },
  },

  regexp_input: Compat {
    name: "input",
    description: r##"<code>RegExp.input</code> (<code>$_</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/input",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  regexp_last_index: Compat {
    name: "lastIndex",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-properties-of-regexp-instances",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  regexp_last_match: Compat {
    name: "lastMatch",
    description: r##"<code>RegExp.lastMatch</code> (<code>$&amp;</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastMatch",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  regexp_last_paren: Compat {
    name: "lastParen",
    description: r##"<code>RegExp.lastParen</code> (<code>$+</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastParen",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  regexp_left_context: Compat {
    name: "leftContext",
    description: r##"<code>RegExp.leftContext</code> (<code>$`</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/leftContext",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  regexp_multiline: Compat {
    name: "multiline",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/multiline",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.multiline",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  regexp_multiline_prototype_accessor: Compat {
    name: "prototype_accessor",
    description: r##"Prototype accessor property (ES2015)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "48",
      edge: "12",
      firefox: "38",
      ie: "5.5",
      node: "No",
      safari: "1.3",
    },
  },

  regexp_n: Compat {
    name: "n",
    description: r##"<code>RegExp.$1-$9</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/n",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_right_context: Compat {
    name: "rightContext",
    description: r##"<code>RegExp.rightContext</code> (<code>$'</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/rightContext",
    spec_url: "https://github.com/tc39/proposal-regexp-legacy-features/#additional-properties-of-the-regexp-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  regexp_source: Compat {
    name: "source",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/source",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.source",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_source_empty_regex_string: Compat {
    name: "empty_regex_string",
    description: r##""(?:)" for empty regexps"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "6",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "5",
    },
  },

  regexp_source_escaping: Compat {
    name: "escaping",
    description: r##"Escaping"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "73",
      edge: "12",
      firefox: "38",
      ie: "10",
      node: "No",
      safari: "6",
    },
  },

  regexp_source_prototype_accessor: Compat {
    name: "prototype_accessor",
    description: r##"Prototype accessor property (ES2015)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "48",
      edge: "12",
      firefox: "41",
      ie: "4",
      node: "No",
      safari: "1.3",
    },
  },

  regexp_sticky: Compat {
    name: "sticky",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/sticky",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.sticky",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "3",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_sticky_anchored_sticky_flag: Compat {
    name: "anchored_sticky_flag",
    description: r##"Anchored sticky flag behavior per ES2015"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "44",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_sticky_prototype_accessor: Compat {
    name: "prototype_accessor",
    description: r##"Prototype accessor property (ES2015)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_test: Compat {
    name: "test",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/test",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype.test",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/toString",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  regexp_to_string_escaping: Compat {
    name: "escaping",
    description: r##"Escaping"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "73",
      edge: "12",
      firefox: "38",
      ie: "9",
      node: "No",
      safari: "6",
    },
  },

  regexp_unicode: Compat {
    name: "unicode",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicode",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.unicode",
    support: Support {
      chrome: "50",
      edge: "12",
      firefox: "46",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_unicode_sets: Compat {
    name: "unicodeSets",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicodeSets",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp.prototype.unicodesets",
    support: Support {
      chrome: "112",
      edge: "No",
      firefox: "116",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  regexp_match_symbol: Compat {
    name: "@@match",
    description: r##"[Symbol.match]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.match",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype-%symbol.match%",
    support: Support {
      chrome: "50",
      edge: "13",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_match_all_symbol: Compat {
    name: "@@matchAll",
    description: r##"[Symbol.matchAll]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.matchAll",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp-prototype-%symbol.matchall%",
    support: Support {
      chrome: "73",
      edge: "No",
      firefox: "67",
      ie: "false",
      node: "No",
      safari: "13",
    },
  },

  regexp_replace_symbol: Compat {
    name: "@@replace",
    description: r##"[Symbol.replace]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.replace",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype-%symbol.replace%",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_search_symbol: Compat {
    name: "@@search",
    description: r##"[Symbol.search]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.search",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype-%symbol.search%",
    support: Support {
      chrome: "50",
      edge: "13",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-get-regexp-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  regexp_split_symbol: Compat {
    name: "@@split",
    description: r##"[Symbol.split]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.split",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-regexp.prototype-%symbol.split%",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set-objects",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_set: Compat {
    name: "Set",
    description: r##"<code>Set()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/Set",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set-constructor",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_set_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Set(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  set_set_null_allowed: Compat {
    name: "null_allowed",
    description: r##"<code>new Set(null)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "37",
      ie: "11",
      node: "No",
      safari: "9",
    },
  },

  set_add: Compat {
    name: "add",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/add",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.add",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_clear: Compat {
    name: "clear",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/clear",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.clear",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "19",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_delete: Compat {
    name: "delete",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/delete",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.delete",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_difference: Compat {
    name: "difference",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/difference",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.difference",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_entries: Compat {
    name: "entries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/entries",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.entries",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "24",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  set_foreach: Compat {
    name: "forEach",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/forEach",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.foreach",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "25",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/has",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.has",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "13",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_intersection: Compat {
    name: "intersection",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/intersection",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.intersection",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_is_disjoint_from: Compat {
    name: "isDisjointFrom",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/isDisjointFrom",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.isdisjointfrom",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_is_subset_of: Compat {
    name: "isSubsetOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/isSubsetOf",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.issubsetof",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_is_superset_of: Compat {
    name: "isSupersetOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/isSupersetOf",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.issupersetof",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_key_equality_for_zeros: Compat {
    name: "key_equality_for_zeros",
    description: r##"Key equality for -0 and 0"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  set_keys: Compat {
    name: "keys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/keys",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.keys",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "24",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  set_size: Compat {
    name: "size",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/size",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-get-set.prototype.size",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "19",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  set_symmetric_difference: Compat {
    name: "symmetricDifference",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/symmetricDifference",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.symmetricdifference",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_union: Compat {
    name: "union",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/union",
    spec_url: "https://tc39.es/proposal-set-methods/#sec-set.prototype.union",
    support: Support {
      chrome: "122",
      edge: "No",
      firefox: "127",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },

  set_values: Compat {
    name: "values",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/values",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype.values",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "24",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  set_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-set.prototype-%symbol.iterator%",
    support: Support {
      chrome: "43",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  set_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-get-set-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  shared_array_buffer: Compat {
    name: "shared_array_buffer",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer-objects",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  shared_array_buffer_shared_array_buffer: Compat {
    name: "SharedArrayBuffer",
    description: r##"<code>SharedArrayBuffer()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/SharedArrayBuffer",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer-constructor",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  shared_array_buffer_shared_array_buffer_max_byte_length_option: Compat {
    name: "maxByteLength_option",
    description: r##"<code>maxByteLength</code> option"##,
    mdn_url: "undefined",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer-constructor",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  shared_array_buffer_byte_length: Compat {
    name: "byteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/byteLength",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-sharedarraybuffer.prototype.bytelength",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  shared_array_buffer_grow: Compat {
    name: "grow",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/grow",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer.prototype.grow",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  shared_array_buffer_growable: Compat {
    name: "growable",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/growable",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-sharedarraybuffer.prototype.growable",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  shared_array_buffer_max_byte_length: Compat {
    name: "maxByteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/maxByteLength",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-get-sharedarraybuffer.prototype.maxbytelength",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "128",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  shared_array_buffer_slice: Compat {
    name: "slice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/slice",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer.prototype.slice",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  shared_array_buffer_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-sharedarraybuffer-%symbol.species%",
    support: Support {
      chrome: "68",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "15.2",
    },
  },

  string: Compat {
    name: "string",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_string: Compat {
    name: "String",
    description: r##"<code>String()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/String",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string-constructor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_anchor: Compat {
    name: "anchor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/anchor",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.anchor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "false",
      node: "No",
      safari: "1",
    },
  },

  string_at: Compat {
    name: "at",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/at",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.at",
    support: Support {
      chrome: "92",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  string_big: Compat {
    name: "big",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/big",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.big",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_blink: Compat {
    name: "blink",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/blink",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.blink",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_bold: Compat {
    name: "bold",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/bold",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.bold",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_charat: Compat {
    name: "charAt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/charAt",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.charat",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_charcodeat: Compat {
    name: "charCodeAt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/charCodeAt",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.charcodeat",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_codepointat: Compat {
    name: "codePointAt",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.codepointat",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_concat: Compat {
    name: "concat",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/concat",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.concat",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_endswith: Compat {
    name: "endsWith",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/endsWith",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.endswith",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "17",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_fixed: Compat {
    name: "fixed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/fixed",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.fixed",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_fontcolor: Compat {
    name: "fontcolor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/fontcolor",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.fontcolor",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_fontsize: Compat {
    name: "fontsize",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/fontsize",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.fontsize",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_fromcharcode: Compat {
    name: "fromCharCode",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/fromCharCode",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.fromcharcode",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_fromcodepoint: Compat {
    name: "fromCodePoint",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/fromCodePoint",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.fromcodepoint",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "29",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_includes: Compat {
    name: "includes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/includes",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.includes",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "40",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_index_of: Compat {
    name: "indexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.indexof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_iswellformed: Compat {
    name: "isWellFormed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/isWellFormed",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.iswellformed",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "119",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  string_italics: Compat {
    name: "italics",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/italics",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.italics",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_last_index_of: Compat {
    name: "lastIndexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.lastindexof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  string_length: Compat {
    name: "length",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/length",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-properties-of-string-instances-length",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_link: Compat {
    name: "link",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/link",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.link",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_localecompare: Compat {
    name: "localeCompare",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/localeCompare",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.localecompare,https://tc39.es/ecma402/#sup-String.prototype.localeCompare",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "3",
    },
  },

  string_localecompare_locales_parameter: Compat {
    name: "locales_parameter",
    description: r##"<code>locales</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  string_localecompare_options_parameter: Compat {
    name: "options_parameter",
    description: r##"<code>options</code> parameter"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "24",
      edge: "12",
      firefox: "29",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  string_match: Compat {
    name: "match",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/match",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.match",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_matchall: Compat {
    name: "matchAll",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.matchall",
    support: Support {
      chrome: "73",
      edge: "No",
      firefox: "67",
      ie: "false",
      node: "No",
      safari: "13",
    },
  },

  string_normalize: Compat {
    name: "normalize",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/normalize",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.normalize",
    support: Support {
      chrome: "34",
      edge: "12",
      firefox: "31",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  string_padend: Compat {
    name: "padEnd",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/padEnd",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.padend",
    support: Support {
      chrome: "57",
      edge: "15",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  string_padstart: Compat {
    name: "padStart",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/padStart",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.padstart",
    support: Support {
      chrome: "57",
      edge: "15",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  string_raw: Compat {
    name: "raw",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/raw",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.raw",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_repeat: Compat {
    name: "repeat",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/repeat",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.repeat",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "24",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_replace: Compat {
    name: "replace",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/replace",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.replace",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  string_replaceall: Compat {
    name: "replaceAll",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.replaceall",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "77",
      ie: "false",
      node: "No",
      safari: "13.1",
    },
  },

  string_search: Compat {
    name: "search",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/search",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.search",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_slice: Compat {
    name: "slice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/slice",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.slice",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_small: Compat {
    name: "small",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/small",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.small",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_split: Compat {
    name: "split",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/split",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.split",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_startswith: Compat {
    name: "startsWith",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.startswith",
    support: Support {
      chrome: "41",
      edge: "12",
      firefox: "17",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  string_strike: Compat {
    name: "strike",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/strike",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.strike",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_sub: Compat {
    name: "sub",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/sub",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.sub",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_substr: Compat {
    name: "substr",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/substr",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.substr",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_substring: Compat {
    name: "substring",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/substring",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.substring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_sup: Compat {
    name: "sup",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/sup",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-string.prototype.sup",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_tolocalelowercase: Compat {
    name: "toLocaleLowerCase",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleLowerCase",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.tolocalelowercase,https://tc39.es/ecma402/#sup-string.prototype.tolocalelowercase",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.3",
    },
  },

  string_tolocalelowercase_locale: Compat {
    name: "locale",
    description: r##"undefined"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "58",
      edge: "12",
      firefox: "55",
      ie: "6",
      node: "No",
      safari: "10",
    },
  },

  string_tolocaleuppercase: Compat {
    name: "toLocaleUpperCase",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleUpperCase",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.tolocaleuppercase,https://tc39.es/ecma402/#sup-string.prototype.tolocaleuppercase",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.3",
    },
  },

  string_tolocaleuppercase_locale: Compat {
    name: "locale",
    description: r##"undefined"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "58",
      edge: "12",
      firefox: "55",
      ie: "6",
      node: "No",
      safari: "10",
    },
  },

  string_tolowercase: Compat {
    name: "toLowerCase",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.tolowercase",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toString",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.tostring",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_touppercase: Compat {
    name: "toUpperCase",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.touppercase",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  string_towellformed: Compat {
    name: "toWellFormed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toWellFormed",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.towellformed",
    support: Support {
      chrome: "111",
      edge: "No",
      firefox: "119",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  string_trim: Compat {
    name: "trim",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trim",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.trim",
    support: Support {
      chrome: "4",
      edge: "12",
      firefox: "3.5",
      ie: "10",
      node: "No",
      safari: "5",
    },
  },

  string_trimend: Compat {
    name: "trimEnd",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.trimend",
    support: Support {
      chrome: "66",
      edge: "79",
      firefox: "61",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  string_trimstart: Compat {
    name: "trimStart",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.trimstart",
    support: Support {
      chrome: "66",
      edge: "79",
      firefox: "61",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  string_unicode_code_point_escapes: Compat {
    name: "unicode_code_point_escapes",
    description: r##"Unicode code point escapes \u{xxxxxx}"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "40",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype.valueof",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  string_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#sec-string.prototype-%symbol.iterator%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol: Compat {
    name: "symbol",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol-objects",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_symbol: Compat {
    name: "Symbol",
    description: r##"<code>Symbol()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol-constructor",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_asynciterator: Compat {
    name: "asyncIterator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/asyncIterator",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.asynciterator",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "57",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  symbol_description: Compat {
    name: "description",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/description",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.prototype.description",
    support: Support {
      chrome: "70",
      edge: "No",
      firefox: "63",
      ie: "false",
      node: "No",
      safari: "12.1",
    },
  },

  symbol_for: Compat {
    name: "for",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/for",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.for",
    support: Support {
      chrome: "40",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_hasinstance: Compat {
    name: "hasInstance",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/hasInstance",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.hasinstance",
    support: Support {
      chrome: "50",
      edge: "15",
      firefox: "50",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_isconcatspreadable: Compat {
    name: "isConcatSpreadable",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/isConcatSpreadable",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.isconcatspreadable",
    support: Support {
      chrome: "48",
      edge: "15",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_iterator: Compat {
    name: "iterator",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.iterator",
    support: Support {
      chrome: "43",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_keyfor: Compat {
    name: "keyFor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/keyFor",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.keyfor",
    support: Support {
      chrome: "40",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_match: Compat {
    name: "match",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/match",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.match",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "40",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_matchall: Compat {
    name: "matchAll",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/matchAll",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.matchall",
    support: Support {
      chrome: "73",
      edge: "No",
      firefox: "67",
      ie: "false",
      node: "No",
      safari: "13",
    },
  },

  symbol_replace: Compat {
    name: "replace",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/replace",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.replace",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_search: Compat {
    name: "search",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/search",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.search",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_species: Compat {
    name: "species",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/species",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.species",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_split: Compat {
    name: "split",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/split",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.split",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "49",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_toprimitive: Compat {
    name: "toPrimitive",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.toprimitive",
    support: Support {
      chrome: "47",
      edge: "15",
      firefox: "44",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toString",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.prototype.tostring",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_tostringtag: Compat {
    name: "toStringTag",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.tostringtag",
    support: Support {
      chrome: "49",
      edge: "15",
      firefox: "51",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  symbol_tostringtag_dom_objects: Compat {
    name: "dom_objects",
    description: r##"<code>toStringTag</code> available on all DOM prototype objects"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag#toStringTag_available_on_all_DOM_prototype_objects",
    spec_url: "undefined",
    support: Support {
      chrome: "50",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  symbol_unscopables: Compat {
    name: "unscopables",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/unscopables",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.unscopables",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_value_of: Compat {
    name: "valueOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/valueOf",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.prototype.valueof",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  symbol_to_primitive_symbol: Compat {
    name: "@@toPrimitive",
    description: r##"[Symbol.toPrimitive]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol.toPrimitive",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.prototype-%symbol.toprimitive%",
    support: Support {
      chrome: "47",
      edge: "15",
      firefox: "44",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  syntaxerror: Compat {
    name: "syntaxerror",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SyntaxError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-syntaxerror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  syntaxerror_syntaxerror: Compat {
    name: "SyntaxError",
    description: r##"<code>SyntaxError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/SyntaxError/SyntaxError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  syntaxerror_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>SyntaxError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  temporal: Compat {
    name: "temporal",
    description: r##"Temporal API"##,
    mdn_url: "undefined",
    spec_url: "https://tc39.es/proposal-temporal/#sec-temporal-intro",
    support: Support {
      chrome: "false",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "preview",
    },
  },

  typeerror: Compat {
    name: "typeerror",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypeError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-typeerror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  typeerror_typeerror: Compat {
    name: "TypeError",
    description: r##"<code>TypeError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypeError/TypeError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  typeerror_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>TypeError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  typedarray: Compat {
    name: "typedarray",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-objects",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_bytes_per_element: Compat {
    name: "BYTES_PER_ELEMENT",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/BYTES_PER_ELEMENT",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray.bytes_per_element",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_at: Compat {
    name: "at",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/at",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.at",
    support: Support {
      chrome: "92",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  typedarray_buffer: Compat {
    name: "buffer",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/buffer",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-%typedarray%.prototype.buffer",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_byte_length: Compat {
    name: "byteLength",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteLength",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-%typedarray%.prototype.bytelength",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_byte_offset: Compat {
    name: "byteOffset",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteOffset",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-%typedarray%.prototype.byteoffset",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_copy_with_in: Compat {
    name: "copyWithin",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/copyWithin",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.copywithin",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_entries: Compat {
    name: "entries",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/entries",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.entries",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_every: Compat {
    name: "every",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/every",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.every",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_fill: Compat {
    name: "fill",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.fill",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_filter: Compat {
    name: "filter",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/filter",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.filter",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_find: Compat {
    name: "find",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/find",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.find",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_find_index: Compat {
    name: "findIndex",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findIndex",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.findindex",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_find_last: Compat {
    name: "findLast",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLast",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.findlast",
    support: Support {
      chrome: "97",
      edge: "No",
      firefox: "104",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  typedarray_find_last_index: Compat {
    name: "findLastIndex",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLastIndex",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.findlastindex",
    support: Support {
      chrome: "97",
      edge: "No",
      firefox: "104",
      ie: "false",
      node: "No",
      safari: "15.4",
    },
  },

  typedarray_foreach: Compat {
    name: "forEach",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/forEach",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.foreach",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_from: Compat {
    name: "from",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/from",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.from",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_includes: Compat {
    name: "includes",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/includes",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.includes",
    support: Support {
      chrome: "47",
      edge: "14",
      firefox: "43",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_index_properties_not_consulting_prototype: Compat {
    name: "index_properties_not_consulting_prototype",
    description: r##"Indexed properties not consulting prototype"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "25",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_index_of: Compat {
    name: "indexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/indexOf",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.indexof",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_iterable_in_constructor: Compat {
    name: "iterable_in_constructor",
    description: r##"Iterable in constructor"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_join: Compat {
    name: "join",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/join",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.join",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_keys: Compat {
    name: "keys",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/keys",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.keys",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_last_index_of: Compat {
    name: "lastIndexOf",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/lastIndexOf",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.lastindexof",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_length: Compat {
    name: "length",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/length",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-%typedarray%.prototype.length",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_map: Compat {
    name: "map",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/map",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.map",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_name: Compat {
    name: "name",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Function/name",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-properties-of-the-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_named_properties: Compat {
    name: "named_properties",
    description: r##"Named properties"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "30",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_of: Compat {
    name: "of",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/of",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.of",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_reduce: Compat {
    name: "reduce",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduce",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.reduce",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_reduce_right: Compat {
    name: "reduceRight",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduceRight",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.reduceright",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_reverse: Compat {
    name: "reverse",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reverse",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.reverse",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/set",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.set",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_slice: Compat {
    name: "slice",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.slice",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "38",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_some: Compat {
    name: "some",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/some",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.some",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_sort: Compat {
    name: "sort",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/sort",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.sort",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "46",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_subarray: Compat {
    name: "subarray",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.subarray",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_to_locale_string: Compat {
    name: "toLocaleString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toLocaleString",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.tolocalestring",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "51",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_to_reversed: Compat {
    name: "toReversed",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toReversed",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.toreversed",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  typedarray_to_sorted: Compat {
    name: "toSorted",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toSorted",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.tosorted",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  typedarray_to_string: Compat {
    name: "toString",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toString",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.tostring",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "51",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  typedarray_values: Compat {
    name: "values",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/values",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.values",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_with: Compat {
    name: "with",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/with",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype.with",
    support: Support {
      chrome: "110",
      edge: "No",
      firefox: "115",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  typedarray_iterator_symbol: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-%typedarray%.prototype-%symbol.iterator%",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  typedarray_species_symbol: Compat {
    name: "@@species",
    description: r##"[Symbol.species]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/Symbol.species",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-get-%typedarray%-%symbol.species%",
    support: Support {
      chrome: "51",
      edge: "13",
      firefox: "48",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  urierror: Compat {
    name: "urierror",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/URIError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-urierror",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  urierror_urierror: Compat {
    name: "URIError",
    description: r##"<code>URIError()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/URIError/URIError",
    spec_url: "https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-nativeerror-constructors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  urierror_serializable_object: Compat {
    name: "serializable_object",
    description: r##"<code>URIError</code> is serializable"##,
    mdn_url: "https://developer.mozilla.org/docs/Glossary/Serializable_object",
    spec_url: "https://html.spec.whatwg.org/multipage/structured-data.html#serializable-objects",
    support: Support {
      chrome: "77",
      edge: "No",
      firefox: "103",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },

  uint16array: Compat {
    name: "uint16array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint16Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint16array_uint16array: Compat {
    name: "Uint16Array",
    description: r##"<code>Uint16Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint16Array/Uint16Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint16array_uint16array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint16array_uint16array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Uint16Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  uint32array: Compat {
    name: "uint32array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint32array_uint32array: Compat {
    name: "Uint32Array",
    description: r##"<code>Uint32Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint32Array/Uint32Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint32array_uint32array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint32array_uint32array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Uint32Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  uint8array: Compat {
    name: "uint8array",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8array_uint8array: Compat {
    name: "Uint8Array",
    description: r##"<code>Uint8Array()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/Uint8Array",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8array_uint8array_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8array_uint8array_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Uint8Array(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  uint8clampedarray: Compat {
    name: "uint8clampedarray",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#table-49",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8clampedarray_uint8clampedarray: Compat {
    name: "Uint8ClampedArray",
    description: r##"<code>Uint8ClampedArray()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray/Uint8ClampedArray",
    spec_url: "https://tc39.es/ecma262/multipage/indexed-collections.html#sec-typedarray-constructors",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "4",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8clampedarray_uint8clampedarray_constructor_without_parameters: Compat {
    name: "constructor_without_parameters",
    description: r##"Constructor without parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "7",
      edge: "12",
      firefox: "55",
      ie: "10",
      node: "No",
      safari: "5.1",
    },
  },

  uint8clampedarray_uint8clampedarray_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new Uint8ClampedArray(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "39",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  weakmap: Compat {
    name: "weakmap",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap-objects",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_weakmap: Compat {
    name: "WeakMap",
    description: r##"<code>WeakMap()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/WeakMap",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap-constructor",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_weakmap_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new WeakMap(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "36",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakmap_weakmap_null_allowed: Compat {
    name: "null_allowed",
    description: r##"<code>new WeakMap(null)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "37",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_delete: Compat {
    name: "delete",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/delete",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap.prototype.delete",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_get: Compat {
    name: "get",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/get",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap.prototype.get",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/has",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap.prototype.has",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/set",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakmap.prototype.set",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "6",
      ie: "11",
      node: "No",
      safari: "8",
    },
  },

  weakmap_symbol_as_keys: Compat {
    name: "symbol_as_keys",
    description: r##"Non-registered symbols as keys"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "109",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  weakref: Compat {
    name: "weakref",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakRef",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-weak-ref-objects",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  weakref_weakref: Compat {
    name: "WeakRef",
    description: r##"<code>WeakRef()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakRef/WeakRef",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-weak-ref-constructor",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  weakref_deref: Compat {
    name: "deref",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakRef/deref",
    spec_url: "https://tc39.es/ecma262/multipage/managing-memory.html#sec-weak-ref.prototype.deref",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  weakref_symbol_as_target: Compat {
    name: "symbol_as_target",
    description: r##"Non-registered symbol as target"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "109",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  weakset: Compat {
    name: "weakset",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakSet",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakset-objects",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_weakset: Compat {
    name: "WeakSet",
    description: r##"<code>WeakSet()</code> constructor"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/WeakSet",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakset-constructor",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_weakset_iterable_allowed: Compat {
    name: "iterable_allowed",
    description: r##"<code>new WeakSet(iterable)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "38",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_weakset_null_allowed: Compat {
    name: "null_allowed",
    description: r##"<code>new WeakSet(null)</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "37",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_add: Compat {
    name: "add",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/add",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakset.prototype.add",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_delete: Compat {
    name: "delete",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/delete",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakset.prototype.delete",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_has: Compat {
    name: "has",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/has",
    spec_url: "https://tc39.es/ecma262/multipage/keyed-collections.html#sec-weakset.prototype.has",
    support: Support {
      chrome: "36",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  weakset_symbol_as_keys: Compat {
    name: "symbol_as_keys",
    description: r##"Non-registered symbols as keys"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "109",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  infinity: Compat {
    name: "infinity",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Infinity",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-value-properties-of-the-global-object-infinity",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  nan: Compat {
    name: "nan",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/NaN",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-value-properties-of-the-global-object-nan",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  decodeuri: Compat {
    name: "decodeuri",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/decodeURI",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-decodeuri-encodeduri",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.1",
    },
  },

  decodeuricomponent: Compat {
    name: "decodeuricomponent",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/decodeURIComponent",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-decodeuricomponent-encodeduricomponent",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.1",
    },
  },

  encodeuri: Compat {
    name: "encodeuri",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/encodeURI",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-encodeuri-uri",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.1",
    },
  },

  encodeuricomponent: Compat {
    name: "encodeuricomponent",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-encodeuricomponent-uricomponent",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1.1",
    },
  },

  escape: Compat {
    name: "escape",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/escape",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-escape-string",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  eval: Compat {
    name: "eval",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/eval",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-eval-x",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  globalthis: Compat {
    name: "globalthis",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/globalThis",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-globalthis",
    support: Support {
      chrome: "71",
      edge: "No",
      firefox: "65",
      ie: "false",
      node: "No",
      safari: "12.1",
    },
  },

  is_finite: Compat {
    name: "is_finite",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/isFinite",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-isfinite-number",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  is_nan: Compat {
    name: "is_nan",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/isNaN",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-isnan-number",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  parse_float: Compat {
    name: "parse_float",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/parseFloat",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-parsefloat-string",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  parse_int: Compat {
    name: "parse_int",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/parseInt",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-parseint-string-radix",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  parse_int_leading_zero_strings_as_decimal: Compat {
    name: "leading_zero_strings_as_decimal",
    description: r##"Parses leading-zero strings as decimal, not octal"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "23",
      edge: "12",
      firefox: "21",
      ie: "9",
      node: "No",
      safari: "6",
    },
  },

  undefined: Compat {
    name: "undefined",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/undefined",
    spec_url: "https://tc39.es/ecma262/multipage/global-object.html#sec-undefined",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  unescape: Compat {
    name: "unescape",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/unescape",
    spec_url: "https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-unescape-string",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },
};
    