# Table of contents

* [Types](#types)
    * [`wasm_result`](#wasm_result)
    * [`peer_type`](#peer_type)
    * [`filter_status`](#filter_status)
    * [`filter_headers_status`](#filter_headers_status)
    * [`filter_data_status`](#filter_data_status)
    * [`filter_trailers_status`](#filter_trailers_status)
    * [`filter_metadata_status`](#filter_metadata_status)
    * [`headers_type`](#headers_type)
    * [`buffer_type`](#buffer_type)
    * [`buffer_flags`](#buffer_flags)
    * [`size`](#size)
    * [`map_size`](#map_size)
    * [`context_id`](#context_id)
    * [`token`](#token)
    * [`metric_id`](#metric_id)
    * [`metric_type`](#metric_type)
    * [`boolean`](#boolean)
    * [`timestamp`](#timestamp)
    * [`grpc_status`](#grpc_status)
    * [`log_level`](#log_level)
* [Modules](#modules)
  * [`envoy_extension`](#envoy_extension) - `ABI` implemented by `Envoy Wasm extensions`
    * [`proxy_on_start`](#proxy_on_start)
    * [`proxy_validate_configuration`](#proxy_validate_configuration)
    * [`proxy_on_configure`](#proxy_on_configure)
    * [`proxy_on_tick`](#proxy_on_tick)
    * [`proxy_on_queue_ready`](#proxy_on_queue_ready)
    * [`proxy_on_create`](#proxy_on_create)
    * [`proxy_on_request_headers`](#proxy_on_request_headers)
    * [`proxy_on_request_body`](#proxy_on_request_body)
    * [`proxy_on_request_trailers`](#proxy_on_request_trailers)
    * [`proxy_on_request_metadata`](#proxy_on_request_metadata)
    * [`proxy_on_response_headers`](#proxy_on_response_headers)
    * [`proxy_on_response_body`](#proxy_on_response_body)
    * [`proxy_on_response_trailers`](#proxy_on_response_trailers)
    * [`proxy_on_response_metadata`](#proxy_on_response_metadata)
    * [`proxy_on_done`](#proxy_on_done)
    * [`proxy_on_log`](#proxy_on_log)
    * [`proxy_on_delete`](#proxy_on_delete)
    * [`proxy_on_http_call_response`](#proxy_on_http_call_response)
    * [`proxy_on_grpc_create_initial_metadata`](#proxy_on_grpc_create_initial_metadata)
    * [`proxy_on_grpc_receive_initial_metadata`](#proxy_on_grpc_receive_initial_metadata)
    * [`proxy_on_grpc_trailing_metadata`](#proxy_on_grpc_trailing_metadata)
    * [`proxy_on_grpc_receive`](#proxy_on_grpc_receive)
    * [`proxy_on_grpc_close`](#proxy_on_grpc_close)
  * [`envoy`](#envoy) - `ABI` implemented by `Envoy` itself (as a host of Wasm extensions)
    * [`proxy_get_configuration`](#proxy_get_configuration)
    * [`proxy_get_status`](#proxy_get_status)
    * [`proxy_log`](#proxy_log)
    * [`proxy_set_tick_period_milliseconds`](#proxy_set_tick_period_milliseconds)
    * [`proxy_get_current_time_nanoseconds`](#proxy_get_current_time_nanoseconds)
    * [`proxy_get_property`](#proxy_get_property)
    * [`proxy_set_property`](#proxy_set_property)
    * [`proxy_set_effective_context`](#proxy_set_effective_context)
    * [`proxy_done`](#proxy_done)
    * [`proxy_define_metric`](#proxy_define_metric)
    * [`proxy_increment_metric`](#proxy_increment_metric)
    * [`proxy_record_metric`](#proxy_record_metric)
    * [`proxy_get_metric`](#proxy_get_metric)
    * [`proxy_continue_request`](#proxy_continue_request)
    * [`proxy_continue_response`](#proxy_continue_response)
    * [`proxy_send_local_response`](#proxy_send_local_response)
    * [`proxy_clear_route_cache`](#proxy_clear_route_cache)
    * [`proxy_get_shared_data`](#proxy_get_shared_data)
    * [`proxy_set_shared_data`](#proxy_set_shared_data)
    * [`proxy_register_shared_queue`](#proxy_register_shared_queue)
    * [`proxy_resolve_shared_queue`](#proxy_resolve_shared_queue)
    * [`proxy_dequeue_shared_queue`](#proxy_dequeue_shared_queue)
    * [`proxy_enqueue_shared_queue`](#proxy_enqueue_shared_queue)
    * [`proxy_add_header_map_value`](#proxy_add_header_map_value)
    * [`proxy_get_header_map_value`](#proxy_get_header_map_value)
    * [`proxy_get_header_map_pairs`](#proxy_get_header_map_pairs)
    * [`proxy_set_header_map_pairs`](#proxy_set_header_map_pairs)
    * [`proxy_replace_header_map_value`](#proxy_replace_header_map_value)
    * [`proxy_remove_header_map_value`](#proxy_remove_header_map_value)
    * [`proxy_get_header_map_size`](#proxy_get_header_map_size)
    * [`proxy_get_buffer_bytes`](#proxy_get_buffer_bytes)
    * [`proxy_get_buffer_status`](#proxy_get_buffer_status)
    * [`proxy_http_call`](#proxy_http_call)
    * [`proxy_grpc_call`](#proxy_grpc_call)
    * [`proxy_grpc_stream`](#proxy_grpc_stream)
    * [`proxy_grpc_cancel`](#proxy_grpc_cancel)
    * [`proxy_grpc_close`](#proxy_grpc_close)
    * [`proxy_grpc_send`](#proxy_grpc_send)

# Types
## `wasm_result`
Error codes returned by ABI functions.

Enum represented by `u32`

### Variants:
#### `ok`
No error occurred. ABI call completed successfully.

#### `not_found`
The result could not be found, e.g. a provided key did not appear in a table.

#### `bad_argument`
An argument was bad, e.g. did not conform to the required range.

#### `serialization_failure`
A protobuf could not be serialized.

#### `parse_failure`
A protobuf could not be parsed.

#### `bad_expression`
A provided expression (e.g. "foo.bar") was illegal or unrecognized.

#### `invalid_memory_access`
A provided memory range was not legal.

#### `empty`
Data was requested from an empty container.

#### `cas_mismatch`
The provided CAS did not match that of the stored data.

#### `result_mismatch`
Returned result was unexpected, e.g. of the incorrect size.

#### `internal_failure`
Internal failure: trying check logs of the surrounding system.

#### `broken_connection`
The connection/stream/pipe was broken/closed unexpectedly.


## `peer_type`
Peer type.

Enum represented by `u32`

### Variants:
#### `unknown`
Unknown.

#### `local`
Local.

#### `remote`
Remote.


## `filter_status`
Status codes returned by filters that can cause future filters to not get iterated to.

Enum represented by `u32`

### Variants:
#### `continue`
Continue to further filters.

#### `stop_iteration`
Stop executing further filters.


## `filter_headers_status`
Return codes for encode/decode headers filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.

#### `stop_iteration`
Do not iterate to any of the remaining filters in the chain. Returning
FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.


## `filter_data_status`
Return codes for encode/decode data filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration. If headers have not yet been sent to the next filter, they
will be sent first via decodeHeaders()/encodeHeaders(). If data has previously been buffered,
the data in this callback will be added to the buffer before the entirety is sent to the next
filter.

#### `stop_iteration_and_buffer`
Do not iterate to any of the remaining filters in the chain, and buffer body data for later
dispatching. Returning FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.

This should be called by filters which must parse a larger block of the incoming data before
continuing processing and so can not push back on streaming data via watermarks.

If buffering the request causes buffered data to exceed the configured buffer limit, a 413 will
be sent to the user. On the response path exceeding buffer limits will result in a 500.

#### `stop_iteration_and_watermark`
Do not iterate to any of the remaining filters in the chain, and buffer body data for later
dispatching. Returning FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.

This will cause the flow of incoming data to cease until one of the continue.*() functions is
called.

This should be returned by filters which can nominally stream data but have a transient back-up
such as the configured delay of the fault filter, or if the router filter is still fetching an
upstream connection.

#### `stop_iteration_no_buffer`
Do not iterate to any of the remaining filters in the chain, but do not buffer any of the
body data for later dispatching. Returning FilterDataStatus::Continue from
decodeData()/encodeData() or calling continueDecoding()/continueEncoding() MUST be called if
continued filter iteration is desired.


## `filter_trailers_status`
Return codes for encode/decode trailers filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.

#### `stop_iteration`
Do not iterate to any of the remaining filters in the chain. Calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.


## `filter_metadata_status`
Return codes for encode metadata filter invocations. Metadata currently can not stop filter
iteration.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.


## `headers_type`
Type of a header map.

Enum represented by `u32`

### Variants:
#### `request_headers`
Request headers of a proxied HTTP request.

#### `request_trailers`
Request trailers of a proxied HTTP request.

#### `response_headers`
Response headers of a proxied HTTP request.

#### `response_trailers`
Response trailers of a proxied HTTP request.

#### `grpc_create_initial_metadata`
Client's initial metadata of an outgoing gRPC call.

#### `grpc_receive_initial_metadata`
Server's initial metadata of an outgoing gRPC call.

#### `grpc_receive_trailing_metadata`
Server's trailing metadata of an outgoing gRPC call.

#### `http_call_response_headers`
Response headers of an outgoing HTTP request.

#### `http_call_response_trailers`
Response trailers of an outgoing HTTP request.


## `buffer_type`
Type of payload in a buffer.

Enum represented by `u32`

### Variants:
#### `http_request_body`
Request body of a proxied HTTP request.

#### `http_response_body`
Response body of a proxied HTTP request.

#### `network_downstream_data`
Request payload of a proxied L4 connection.

#### `network_upstream_data`
Response payload of a proxied L4 connection.

#### `http_call_response_body`
Response body of an outgoing HTTP request.

#### `grpc_receive_buffer`
Response payload of an outgoing gRPC call.


## `buffer_flags`
Buffer flags.

Flags represented by `u32`

### Flags:
#### `end_of_stream`
End of stream has been reached.


## `size`
Size of data in a linear memory.

Builtin type u32
## `map_size`
Size of a header map, e.g. request headers, response trailers, etc.

Builtin type u32
## `context_id`
Unique identifier of a root/connection/request context.

### Handle supertypes:


## `token`
Opaque identifier of an outgoing HTTP request, gRPC call, etc.

### Handle supertypes:


## `metric_id`
Metric handle.

### Handle supertypes:


## `metric_type`
Metric type.

Enum represented by `u32`

### Variants:
#### `counter`
Counter.

#### `gauge`
Gauge.

#### `histogram`
Histogram.


## `boolean`
Boolean true or false.

Builtin type u32
## `timestamp`
Timestamp in nanoseconds.

Builtin type u64
## `grpc_status`
Well-known gRPC statuses.

Enum represented by `u32`

### Variants:
#### `ok`
The RPC completed successfully.

#### `canceled`
The RPC was canceled.

#### `unknown`
Some unknown error occurred.

#### `invalid_argument`
An argument to the RPC was invalid.

#### `deadline_exceeded`
The deadline for the RPC expired before the RPC completed.

#### `not_found`
Some resource for the RPC was not found.

#### `already_exists`
A resource the RPC attempted to create already exists.

#### `permission_denied`
Permission was denied for the RPC.

#### `resource_exhausted`
Some resource is exhausted, resulting in RPC failure.

#### `failed_precondition`
Some precondition for the RPC failed.

#### `aborted`
The RPC was aborted.

#### `out_of_range`
Some operation was requested outside of a legal range.

#### `unimplemented`
The RPC requested was not implemented.

#### `internal`
Some internal error occurred.

#### `unavailable`
The RPC endpoint is current unavailable.

#### `data_loss`
There was some data loss resulting in RPC failure.

#### `unauthenticated`
The RPC does not have required credentials for the RPC to succeed.

#### `invalid_code`
This is a non-GRPC error code, indicating the status code in gRPC headers
was invalid.


## `log_level`
Log level.

Enum represented by `u32`

### Variants:
#### `trace`
Trace.

#### `debug`
Debug.

#### `info`
Info.

#### `warn`
Warn.

#### `error`
Error.

#### `critical`
Critical.



# Modules
## `envoy_extension`
### Imports
* memory: Memory
### Functions
### proxy_on_start
Calls in.
extern "C" uint32_t proxy_on_start(uint32_t root_context_id, uint32_t configuration_size);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)

##### `configuration_len`
`configuration_len` has type `size`
TODO(docs)
#### Results:
##### `result`
`result` has type `u32`
TODO(docs)

### proxy_validate_configuration
extern "C" uint32_t proxy_validate_configuration(uint32_t root_context_id,
                                                 uint32_t configuration_size);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)

##### `configuration_len`
`configuration_len` has type `size`
TODO(docs)
#### Results:
##### `result`
`result` has type `u32`
TODO(docs)

### proxy_on_configure
extern "C" uint32_t proxy_on_configure(uint32_t root_context_id, uint32_t configuration_size);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)

##### `configuration_len`
`configuration_len` has type `size`
TODO(docs)
#### Results:
##### `result`
`result` has type `u32`
TODO(docs)

### proxy_on_tick
extern "C" void proxy_on_tick(uint32_t root_context_id);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)
#### Results:

### proxy_on_queue_ready
extern "C" void proxy_on_queue_ready(uint32_t root_context_id, uint32_t token);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)
#### Results:

### proxy_on_create
Stream calls.
extern "C" void proxy_on_create(uint32_t context_id, uint32_t root_context_id);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `root_context_id`
`root_context_id` has type `context_id`
TODO(docs)
#### Results:

### proxy_on_new_connection
Network calls.
extern "C" FilterStatus proxy_on_new_connection(uint32_t context_id);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_status`
TODO(docs)

### proxy_on_downstream_data
extern "C" FilterStatus proxy_on_downstream_data(uint32_t context_id,
                                                 uint32_t data_length,
                                                 uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `data_len`
`data_len` has type `size`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_status`
TODO(docs)

### proxy_on_upstream_data
extern "C" FilterStatus proxy_on_upstream_data(uint32_t context_id,
                                               uint32_t data_length,
                                               uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `data_len`
`data_len` has type `size`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_status`
TODO(docs)

### proxy_on_downstream_connection_close
extern "C" void proxy_on_downstream_connection_close(uint32_t context_id,
                                                     uint32_t peer_type);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `peer`
`peer` has type `peer_type`
TODO(docs)
#### Results:

### proxy_on_upstream_connection_close
extern "C" void proxy_on_upstream_connection_close(uint32_t context_id,
                                                   uint32_t peer_type);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `peer`
`peer` has type `peer_type`
TODO(docs)
#### Results:

### proxy_on_request_headers
HTTP calls.
extern "C" FilterHeadersStatus proxy_on_request_headers(uint32_t context_id, uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `headers_count`
`headers_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_headers_status`
TODO(docs)

### proxy_on_request_body
extern "C" FilterDataStatus proxy_on_request_body(uint32_t context_id, uint32_t body_buffer_length,
                                                  uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `body_buffer_length`
`body_buffer_length` has type `size`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_data_status`
TODO(docs)

### proxy_on_request_trailers
extern "C" FilterTrailersStatus proxy_on_request_trailers(uint32_t context_id, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `trailers_count`
`trailers_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_trailers_status`
TODO(docs)

### proxy_on_request_metadata
extern "C" FilterMetadataStatus proxy_on_request_metadata(uint32_t context_id, uint32_t nelements);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `elements_count`
`elements_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_metadata_status`
TODO(docs)

### proxy_on_response_headers
extern "C" FilterHeadersStatus proxy_on_response_headers(uint32_t context_id, uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `headers_count`
`headers_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_headers_status`
TODO(docs)

### proxy_on_response_body
extern "C" FilterDataStatus proxy_on_response_body(uint32_t context_id, uint32_t body_buffer_length,
                                                   uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `body_buffer_length`
`body_buffer_length` has type `size`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_data_status`
TODO(docs)

### proxy_on_response_trailers
extern "C" FilterTrailersStatus proxy_on_response_trailers(uint32_t context_id, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `trailers_count`
`trailers_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_trailers_status`
TODO(docs)

### proxy_on_response_metadata
extern "C" FilterMetadataStatus proxy_on_response_metadata(uint32_t context_id, uint32_t nelements);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `elements_count`
`elements_count` has type `map_size`
TODO(docs)
#### Results:
##### `status`
`status` has type `filter_metadata_status`
TODO(docs)

### proxy_on_done
extern "C" uint32_t proxy_on_done(uint32_t context_id);
The stream/vm has completed.

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)
#### Results:
##### `result`
`result` has type `u32`
TODO(docs)

### proxy_on_log
extern "C" void proxy_on_log(uint32_t context_id);
proxy_on_log occurs after proxy_on_done.

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)
#### Results:

### proxy_on_delete
extern "C" void proxy_on_delete(uint32_t context_id);
The Context in the proxy has been destroyed and no further calls will be coming.

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)
#### Results:

### proxy_on_http_call_response
HTTP.
extern "C" void proxy_on_http_call_response(uint32_t context_id, uint32_t token, uint32_t headers,
                                            uint32_t body_size, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers_count`
`headers_count` has type `map_size`
TODO(docs)

##### `body_size`
`body_size` has type `size`
TODO(docs)

##### `trailers_count`
`trailers_count` has type `map_size`
TODO(docs)
#### Results:

### proxy_on_grpc_create_initial_metadata
gRPC.
extern "C" void proxy_on_grpc_create_initial_metadata(uint32_t context_id, uint32_t token,
                                                      uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers_count`
`headers_count` has type `map_size`
TODO(docs)
#### Results:

### proxy_on_grpc_receive_initial_metadata
extern "C" void proxy_on_grpc_receive_initial_metadata(uint32_t context_id, uint32_t token,
                                                       uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers_count`
`headers_count` has type `map_size`
TODO(docs)
#### Results:

### proxy_on_grpc_trailing_metadata
extern "C" void proxy_on_grpc_trailing_metadata(uint32_t context_id, uint32_t token,
                                                uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `trailers_count`
`trailers_count` has type `map_size`
TODO(docs)
#### Results:

### proxy_on_grpc_receive
extern "C" void proxy_on_grpc_receive(uint32_t context_id, uint32_t token, uint32_t response_size);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `response_size`
`response_size` has type `size`
TODO(docs)
#### Results:

### proxy_on_grpc_close
extern "C" void proxy_on_grpc_close(uint32_t context_id, uint32_t token, uint32_t status_code);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `context_id`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `status_code`
`status_code` has type `grpc_status`
TODO(docs)
#### Results:

## `envoy`
### Imports
* memory: Memory
### Functions
### proxy_get_configuration
Configuration and Status.
extern "C" WasmResult proxy_get_configuration(const char** configuration_ptr,
                                              size_t* configuration_size);
TODO(docs)

#### Parameters:
##### `config`
`config` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `config_len`
`config_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_status
extern "C" WasmResult proxy_get_status(uint32_t* status_code_ptr, const char** message_ptr,
                                       size_t* message_size);
Results status details for any previous ABI call and onGrpcClose.

#### Parameters:
##### `code`
`code` has type `Pointer<wasm_result>`
TODO(docs)

##### `message`
`message` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `message_len`
`message_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_log
Logging
extern "C" WasmResult proxy_log(LogLevel level, const char* logMessage, size_t messageSize);
TODO(docs)

#### Parameters:
##### `level`
`level` has type `log_level`
TODO(docs)

##### `message`
`message` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_set_tick_period_milliseconds
Timer
extern "C" WasmResult proxy_set_tick_period_milliseconds(uint32_t millisecond);
Timer (must be called from a root context, e.g. onStart, onTick).

#### Parameters:
##### `period_millis`
`period_millis` has type `u32`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_current_time_nanoseconds
Time
extern "C" WasmResult proxy_get_current_time_nanoseconds(uint64_t* nanoseconds);
TODO(docs)

#### Parameters:
##### `time`
`time` has type `Pointer<timestamp>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_property
State accessors
extern "C" WasmResult proxy_get_property(const char* path_ptr, size_t path_size,
                                         const char** value_ptr_ptr, size_t* value_size_ptr);
TODO(docs)

#### Parameters:
##### `path`
`path` has type `string`
TODO(docs)

##### `value`
`value` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_len`
`value_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_set_property
extern "C" WasmResult proxy_set_property(const char* path_ptr, size_t path_size,
                                         const char* value_ptr, size_t value_size);
TODO(docs)

#### Parameters:
##### `path`
`path` has type `string`
TODO(docs)

##### `value`
`value` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_set_effective_context
System
extern "C" WasmResult proxy_set_effective_context(uint32_t effective_context_id);
TODO(docs)

#### Parameters:
##### `id`
`id` has type `context_id`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_done
extern "C" WasmResult proxy_done();
TODO(docs)

#### Parameters:
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_define_metric
Metrics
extern "C" WasmResult proxy_define_metric(MetricType type, const char* name_ptr, size_t name_size,
                                          uint32_t* metric_id);
TODO(docs)

#### Parameters:
##### `metric_type`
`metric_type` has type `metric_type`
TODO(docs)

##### `name`
`name` has type `string`
TODO(docs)

##### `id`
`id` has type `Pointer<metric_id>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_increment_metric
extern "C" WasmResult proxy_increment_metric(uint32_t metric_id, int64_t offset);
TODO(docs)

#### Parameters:
##### `id`
`id` has type `metric_id`
TODO(docs)

##### `offset`
`offset` has type `s64`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_record_metric
extern "C" WasmResult proxy_record_metric(uint32_t metric_id, uint64_t value);
TODO(docs)

#### Parameters:
##### `id`
`id` has type `metric_id`
TODO(docs)

##### `value`
`value` has type `u64`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_metric
extern "C" WasmResult proxy_get_metric(uint32_t metric_id, uint64_t* result);
TODO(docs)

#### Parameters:
##### `id`
`id` has type `metric_id`
TODO(docs)

##### `value`
`value` has type `Pointer<u64>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_continue_request
Continue/Reply/Route
extern "C" WasmResult proxy_continue_request();
TODO(docs)

#### Parameters:
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_continue_response
extern "C" WasmResult proxy_continue_response();
TODO(docs)

#### Parameters:
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_send_local_response
extern "C" WasmResult
proxy_send_local_response(uint32_t response_code, const char* response_code_details_ptr,
                          size_t response_code_details_size, const char* body_ptr, size_t body_size,
                          const char* additional_response_header_pairs_ptr,
                          size_t additional_response_header_pairs_size, uint32_t grpc_status);
TODO(docs)

#### Parameters:
##### `response_code`
`response_code` has type `u32`
TODO(docs)

##### `response_code_details`
`response_code_details` has type `string`
TODO(docs)

##### `body`
`body` has type `string`
TODO(docs)

##### `additional_response_header_pairs`
`additional_response_header_pairs` has type `string`
TODO(docs)

##### `grpc_status`
`grpc_status` has type `grpc_status`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_clear_route_cache
extern "C" WasmResult proxy_clear_route_cache();
TODO(docs)

#### Parameters:
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_shared_data
SharedData
extern "C" WasmResult proxy_get_shared_data(const char* key_ptr, size_t key_size,
                                            const char** value_ptr, size_t* value_size,
                                            uint32_t* cas);
Returns: Ok, NotFound
TODO(docs)

#### Parameters:
##### `key`
`key` has type `string`
TODO(docs)

##### `value`
`value` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_len`
`value_len` has type `Pointer<size>`
TODO(docs)

##### `cas`
`cas` has type `Pointer<u32>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_set_shared_data
extern "C" WasmResult proxy_set_shared_data(const char* key_ptr, size_t key_size,
                                            const char* value_ptr, size_t value_size, uint32_t cas);
//  If cas != 0 and cas != the current cas for 'key' return false, otherwise set the value and
//  return true.
// Returns: Ok, CasMismatch
TODO(docs)

#### Parameters:
##### `key`
`key` has type `string`
TODO(docs)

##### `value`
`value` has type `string`
TODO(docs)

##### `cas`
`cas` has type `u32`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_register_shared_queue
SharedQueue
extern "C" WasmResult proxy_register_shared_queue(const char* queue_name_ptr,
                                                  size_t queue_name_size, uint32_t* token);
// Note: Registering the same queue_name will overwrite the old registration while preseving any
// pending data. Consequently it should typically be followed by a call to
// proxy_dequeue_shared_queue. Returns: Ok
TODO(docs)

#### Parameters:
##### `queue_name`
`queue_name` has type `string`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_resolve_shared_queue
extern "C" WasmResult proxy_resolve_shared_queue(const char* vm_id, size_t vm_id_size,
                                                 const char* queue_name_ptr, size_t queue_name_size,
                                                 uint32_t* token);
// Returns: Ok, NotFound
TODO(docs)

#### Parameters:
##### `vm_id`
`vm_id` has type `string`
TODO(docs)

##### `queue_name`
`queue_name` has type `string`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_dequeue_shared_queue
extern "C" WasmResult proxy_dequeue_shared_queue(uint32_t token, const char** data_ptr,
                                                 size_t* data_size);
// Returns Ok, Empty, NotFound (token not registered).
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)

##### `data`
`data` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `data_len`
`data_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_enqueue_shared_queue
extern "C" WasmResult proxy_enqueue_shared_queue(uint32_t token, const char* data_ptr,
                                                 size_t data_size);
// Returns false if the queue was not found and the data was not enqueued.
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)

##### `data`
`data` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_add_header_map_value
Headers/Trailers/Metadata Maps
extern "C" WasmResult proxy_add_header_map_value(HeaderMapType type, const char* key_ptr,
                                                 size_t key_size, const char* value_ptr,
                                                 size_t value_size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `key`
`key` has type `string`
TODO(docs)

##### `value`
`value` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_header_map_value
extern "C" WasmResult proxy_get_header_map_value(HeaderMapType type, const char* key_ptr,
                                                 size_t key_size, const char** value_ptr,
                                                 size_t* value_size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `key`
`key` has type `string`
TODO(docs)

##### `value`
`value` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_len`
`value_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_header_map_pairs
extern "C" WasmResult proxy_get_header_map_pairs(HeaderMapType type, const char** ptr,
                                                 size_t* size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `buf`
`buf` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `buf_len`
`buf_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_set_header_map_pairs
extern "C" WasmResult proxy_set_header_map_pairs(HeaderMapType type, const char* ptr, size_t size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `buf`
`buf` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_replace_header_map_value
extern "C" WasmResult proxy_replace_header_map_value(HeaderMapType type, const char* key_ptr,
                                                     size_t key_size, const char* value_ptr,
                                                     size_t value_size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `key`
`key` has type `string`
TODO(docs)

##### `value`
`value` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_remove_header_map_value
extern "C" WasmResult proxy_remove_header_map_value(HeaderMapType type, const char* key_ptr,
                                                    size_t key_size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `key`
`key` has type `string`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_header_map_size
extern "C" WasmResult proxy_get_header_map_size(HeaderMapType type, size_t* size);
TODO(docs)

#### Parameters:
##### `headers_type`
`headers_type` has type `headers_type`
TODO(docs)

##### `len`
`len` has type `Pointer<map_size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_buffer_bytes
Buffer
extern "C" WasmResult proxy_get_buffer_bytes(BufferType type, uint32_t start, uint32_t length,
                                             const char** ptr, size_t* size);
TODO(docs)

#### Parameters:
##### `buffer_type`
`buffer_type` has type `buffer_type`
TODO(docs)

##### `start`
`start` has type `u32`
TODO(docs)

##### `length`
`length` has type `u32`
TODO(docs)

##### `buf`
`buf` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `buf_len`
`buf_len` has type `Pointer<size>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_get_buffer_status
extern "C" WasmResult proxy_get_buffer_status(BufferType type, size_t* length_ptr,
                                              uint32_t* flags_ptr);
TODO(docs)

#### Parameters:
##### `buffer_type`
`buffer_type` has type `buffer_type`
TODO(docs)

##### `len`
`len` has type `Pointer<u32>`
TODO(docs)

##### `flags`
`flags` has type `Pointer<buffer_flags>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_http_call
HTTP
extern "C" WasmResult proxy_http_call(const char* uri_ptr, size_t uri_size, void* header_pairs_ptr,
                                      size_t header_pairs_size, const char* body_ptr,
                                      size_t body_size, void* trailer_pairs_ptr,
                                      size_t trailer_pairs_size, uint32_t timeout_milliseconds,
                                      uint32_t* token_ptr);
TODO(docs)

#### Parameters:
##### `uri`
`uri` has type `string`
TODO(docs)

##### `header_pairs`
`header_pairs` has type `string`
TODO(docs)

##### `body`
`body` has type `string`
TODO(docs)

##### `trailer_pairs`
`trailer_pairs` has type `string`
TODO(docs)

##### `timeout_ms`
`timeout_ms` has type `u32`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_grpc_call
gRPC
extern "C" WasmResult proxy_grpc_call(const char* service_ptr, size_t service_size,
                                      const char* service_name_ptr, size_t service_name_size,
                                      const char* method_name_ptr, size_t method_name_size,
                                      const char* request_ptr, size_t request_size,
                                      uint32_t timeout_milliseconds, uint32_t* token_ptr);
TODO(docs)

#### Parameters:
##### `service`
`service` has type `string`
TODO(docs)

##### `service_name`
`service_name` has type `string`
TODO(docs)

##### `method_name`
`method_name` has type `string`
TODO(docs)

##### `request`
`request` has type `string`
TODO(docs)

##### `timeout_ms`
`timeout_ms` has type `u32`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_grpc_stream
extern "C" WasmResult proxy_grpc_stream(const char* service_ptr, size_t service_size,
                                        const char* service_name_ptr, size_t service_name_size,
                                        const char* method_name_ptr, size_t method_name_size,
                                        uint32_t* token_ptr);
TODO(docs)

#### Parameters:
##### `service`
`service` has type `string`
TODO(docs)

##### `service_name`
`service_name` has type `string`
TODO(docs)

##### `method_name`
`method_name` has type `string`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_grpc_cancel
extern "C" WasmResult proxy_grpc_cancel(uint32_t token);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_grpc_close
extern "C" WasmResult proxy_grpc_close(uint32_t token);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)

### proxy_grpc_send
extern "C" WasmResult proxy_grpc_send(uint32_t token, const char* message_ptr, size_t message_size,
                                      uint32_t end_stream);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)

##### `message`
`message` has type `string`
TODO(docs)

##### `end_stream`
`end_stream` has type `boolean`
TODO(docs)
#### Results:
##### `error`
`error` has type `wasm_result`
TODO(docs)
