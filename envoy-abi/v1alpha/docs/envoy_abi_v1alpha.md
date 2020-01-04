# Types
## `wasmresult`
Error codes returned by ABI functions.

Enum represented by `u32`

### Variants:
#### `ok`
No error occurred. ABI call completed successfully.

#### `notfound`
The result could not be found, e.g. a provided key did not appear in a table.

#### `badargument`
An argument was bad, e.g. did not conform to the required range.

#### `serializationfailure`
A protobuf could not be serialized.

#### `parsefailure`
A protobuf could not be parsed.

#### `badexpression`
A provided expression (e.g. "foo.bar") was illegal or unrecognized.

#### `invalidmemoryaccess`
A provided memory range was not legal.

#### `empty`
Data was requested from an empty container.

#### `casmismatch`
The provided CAS did not match that of the stored data.

#### `resultmismatch`
Returned result was unexpected, e.g. of the incorrect size.

#### `internalfailure`
Internal failure: trying check logs of the surrounding system.

#### `brokenconnection`
The connection/stream/pipe was broken/closed unexpectedly.


## `filterheadersstatus`
Return codes for encode/decode headers filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.

#### `stopiteration`
Do not iterate to any of the remaining filters in the chain. Returning
FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.


## `filterdatastatus`
Return codes for encode/decode data filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration. If headers have not yet been sent to the next filter, they
will be sent first via decodeHeaders()/encodeHeaders(). If data has previously been buffered,
the data in this callback will be added to the buffer before the entirety is sent to the next
filter.

#### `stopiterationandbuffer`
Do not iterate to any of the remaining filters in the chain, and buffer body data for later
dispatching. Returning FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.

This should be called by filters which must parse a larger block of the incoming data before
continuing processing and so can not push back on streaming data via watermarks.

If buffering the request causes buffered data to exceed the configured buffer limit, a 413 will
be sent to the user. On the response path exceeding buffer limits will result in a 500.

#### `stopiterationandwatermark`
Do not iterate to any of the remaining filters in the chain, and buffer body data for later
dispatching. Returning FilterDataStatus::Continue from decodeData()/encodeData() or calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.

This will cause the flow of incoming data to cease until one of the continue.*() functions is
called.

This should be returned by filters which can nominally stream data but have a transient back-up
such as the configured delay of the fault filter, or if the router filter is still fetching an
upstream connection.

#### `stopiterationnobuffer`
Do not iterate to any of the remaining filters in the chain, but do not buffer any of the
body data for later dispatching. Returning FilterDataStatus::Continue from
decodeData()/encodeData() or calling continueDecoding()/continueEncoding() MUST be called if
continued filter iteration is desired.


## `filtertrailersstatus`
Return codes for encode/decode trailers filter invocations. The connection manager bases further
filter invocations on the return code of the previous filter.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.

#### `stopiteration`
Do not iterate to any of the remaining filters in the chain. Calling
continueDecoding()/continueEncoding() MUST be called if continued filter iteration is desired.


## `filtermetadatastatus`
Return codes for encode metadata filter invocations. Metadata currently can not stop filter
iteration.

Enum represented by `u32`

### Variants:
#### `continue`
Continue filter chain iteration.


## `headermaptype`
Type of a header map.

Enum represented by `u32`

### Variants:
#### `requestheaders`
Request headers of a proxied HTTP request.

#### `requesttrailers`
Request trailers of a proxied HTTP request.

#### `responseheaders`
Response headers of a proxied HTTP request.

#### `responsetrailers`
Response trailers of a proxied HTTP request.

#### `grpccreateinitialmetadata`
Client's initial metadata of an outgoing gRPC call.

#### `grpcreceiveinitialmetadata`
Server's initial metadata of an outgoing gRPC call.

#### `grpcreceivetrailingmetadata`
Server's trailing metadata of an outgoing gRPC call.

#### `httpcallresponseheaders`
Response headers of an outgoing HTTP request.

#### `httpcallresponsetrailers`
Response trailers of an outgoing HTTP request.


## `buffertype`
Type of payload in a buffer.

Enum represented by `u32`

### Variants:
#### `httprequestbody`
Request body of a proxied HTTP request.

#### `httpresponsebody`
Response body of a proxied HTTP request.

#### `networkdownstreamdata`
Request payload of a proxied L4 connection.

#### `networkupstreamdata`
Response payload of a proxied L4 connection.

#### `httpcallresponsebody`
Response body of an outgoing HTTP request.

#### `grpcreceivebuffer`
Response payload of an outgoing gRPC call.


## `bufferflags`
Buffer flags.

Flags represented by `u32`

### Flags:
#### `endofstream`
End of stream has been reached.


## `memorysize`
Size of data in a linear memory.

Builtin type u32
## `mapsize`
Size of a header map, e.g. request headers, response trailers, etc.

Builtin type u32
## `contextid`
Unique identifier of a root/connection/request context.

Builtin type u32
## `token`
Opaque identifier of an outgoing HTTP request, gRPC call, etc.

Builtin type u32
## `metric`
Metric handle.

Builtin type u32
## `metrictype`
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
## `grpcstatus`
Well-known gRPC statuses.

Enum represented by `u32`

### Variants:
#### `ok`
The RPC completed successfully.

#### `canceled`
The RPC was canceled.

#### `unknown`
Some unknown error occurred.

#### `invalidargument`
An argument to the RPC was invalid.

#### `deadlineexceeded`
The deadline for the RPC expired before the RPC completed.

#### `notfound`
Some resource for the RPC was not found.

#### `alreadyexists`
A resource the RPC attempted to create already exists.

#### `permissiondenied`
Permission was denied for the RPC.

#### `resourceexhausted`
Some resource is exhausted, resulting in RPC failure.

#### `failedprecondition`
Some precondition for the RPC failed.

#### `aborted`
The RPC was aborted.

#### `outofrange`
Some operation was requested outside of a legal range.

#### `unimplemented`
The RPC requested was not implemented.

#### `internal`
Some internal error occurred.

#### `unavailable`
The RPC endpoint is current unavailable.

#### `dataloss`
There was some data loss resulting in RPC failure.

#### `unauthenticated`
The RPC does not have required credentials for the RPC to succeed.

#### `invalidcode`
This is a non-GRPC error code, indicating the status code in gRPC headers
was invalid.


## `loglevel`
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
`root_context_id` has type `contextid`
TODO(docs)

##### `configuration_size`
`configuration_size` has type `memorysize`
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
`root_context_id` has type `contextid`
TODO(docs)

##### `configuration_size`
`configuration_size` has type `memorysize`
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
`root_context_id` has type `contextid`
TODO(docs)

##### `configuration_size`
`configuration_size` has type `memorysize`
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
`root_context_id` has type `contextid`
TODO(docs)
#### Results:

### proxy_on_queue_ready
extern "C" void proxy_on_queue_ready(uint32_t root_context_id, uint32_t token);
TODO(docs)

#### Parameters:
##### `root_context_id`
`root_context_id` has type `contextid`
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
`context_id` has type `contextid`
TODO(docs)

##### `root_context_id`
`root_context_id` has type `contextid`
TODO(docs)
#### Results:

### proxy_on_request_headers
extern "C" FilterHeadersStatus proxy_on_request_headers(uint32_t context_id, uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `headers`
`headers` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filterheadersstatus`
TODO(docs)

### proxy_on_request_body
extern "C" FilterDataStatus proxy_on_request_body(uint32_t context_id, uint32_t body_buffer_length,
                                                  uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `body_buffer_length`
`body_buffer_length` has type `memorysize`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filterdatastatus`
TODO(docs)

### proxy_on_request_trailers
extern "C" FilterTrailersStatus proxy_on_request_trailers(uint32_t context_id, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `trailers`
`trailers` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filtertrailersstatus`
TODO(docs)

### proxy_on_request_metadata
extern "C" FilterMetadataStatus proxy_on_request_metadata(uint32_t context_id, uint32_t nelements);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `nelements`
`nelements` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filtermetadatastatus`
TODO(docs)

### proxy_on_response_headers
extern "C" FilterHeadersStatus proxy_on_response_headers(uint32_t context_id, uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `headers`
`headers` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filterheadersstatus`
TODO(docs)

### proxy_on_response_body
extern "C" FilterDataStatus proxy_on_response_body(uint32_t context_id, uint32_t body_buffer_length,
                                                   uint32_t end_of_stream);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `body_buffer_length`
`body_buffer_length` has type `memorysize`
TODO(docs)

##### `end_of_stream`
`end_of_stream` has type `boolean`
TODO(docs)
#### Results:
##### `status`
`status` has type `filterdatastatus`
TODO(docs)

### proxy_on_response_trailers
extern "C" FilterTrailersStatus proxy_on_response_trailers(uint32_t context_id, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `trailers`
`trailers` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filtertrailersstatus`
TODO(docs)

### proxy_on_response_metadata
extern "C" FilterMetadataStatus proxy_on_response_metadata(uint32_t context_id, uint32_t nelements);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `nelements`
`nelements` has type `mapsize`
TODO(docs)
#### Results:
##### `status`
`status` has type `filtermetadatastatus`
TODO(docs)

### proxy_on_done
extern "C" uint32_t proxy_on_done(uint32_t context_id);
The stream/vm has completed.

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
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
`context_id` has type `contextid`
TODO(docs)
#### Results:

### proxy_on_delete
extern "C" void proxy_on_delete(uint32_t context_id);
The Context in the proxy has been destroyed and no further calls will be coming.

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)
#### Results:

### proxy_on_http_call_response
HTTP.
extern "C" void proxy_on_http_call_response(uint32_t context_id, uint32_t token, uint32_t headers,
                                            uint32_t body_size, uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers`
`headers` has type `mapsize`
TODO(docs)

##### `body_size`
`body_size` has type `memorysize`
TODO(docs)

##### `trailers`
`trailers` has type `mapsize`
TODO(docs)
#### Results:

### proxy_on_grpc_create_initial_metadata
gRPC.
extern "C" void proxy_on_grpc_create_initial_metadata(uint32_t context_id, uint32_t token,
                                                      uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers`
`headers` has type `mapsize`
TODO(docs)
#### Results:

### proxy_on_grpc_receive_initial_metadata
extern "C" void proxy_on_grpc_receive_initial_metadata(uint32_t context_id, uint32_t token,
                                                       uint32_t headers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `headers`
`headers` has type `mapsize`
TODO(docs)
#### Results:

### proxy_on_grpc_trailing_metadata
extern "C" void proxy_on_grpc_trailing_metadata(uint32_t context_id, uint32_t token,
                                                uint32_t trailers);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `trailers`
`trailers` has type `mapsize`
TODO(docs)
#### Results:

### proxy_on_grpc_receive
extern "C" void proxy_on_grpc_receive(uint32_t context_id, uint32_t token, uint32_t response_size);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `response_size`
`response_size` has type `memorysize`
TODO(docs)
#### Results:

### proxy_on_grpc_close
extern "C" void proxy_on_grpc_close(uint32_t context_id, uint32_t token, uint32_t status_code);
TODO(docs)

#### Parameters:
##### `context_id`
`context_id` has type `contextid`
TODO(docs)

##### `token`
`token` has type `token`
TODO(docs)

##### `status_code`
`status_code` has type `grpcstatus`
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
##### `configuration_ptr`
`configuration_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `configuration_size`
`configuration_size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_status
extern "C" WasmResult proxy_get_status(uint32_t* status_code_ptr, const char** message_ptr,
                                       size_t* message_size);
Results status details for any previous ABI call and onGrpcClose.

#### Parameters:
##### `status_code_ptr`
`status_code_ptr` has type `Pointer<wasmresult>`
TODO(docs)

##### `message_ptr`
`message_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `message_size`
`message_size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_log
Logging
extern "C" WasmResult proxy_log(LogLevel level, const char* logMessage, size_t messageSize);
TODO(docs)

#### Parameters:
##### `level`
`level` has type `loglevel`
TODO(docs)

##### `logMessage`
`logMessage` has type `Pointer<char8>`
TODO(docs)

##### `messageSize`
`messageSize` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_set_tick_period_milliseconds
Timer
extern "C" WasmResult proxy_set_tick_period_milliseconds(uint32_t millisecond);
Timer (must be called from a root context, e.g. onStart, onTick).

#### Parameters:
##### `millisecond`
`millisecond` has type `u32`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_current_time_nanoseconds
Time
extern "C" WasmResult proxy_get_current_time_nanoseconds(uint64_t* nanoseconds);
TODO(docs)

#### Parameters:
##### `nanoseconds`
`nanoseconds` has type `Pointer<timestamp>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_property
State accessors
extern "C" WasmResult proxy_get_property(const char* path_ptr, size_t path_size,
                                         const char** value_ptr_ptr, size_t* value_size_ptr);
TODO(docs)

#### Parameters:
##### `path_ptr`
`path_ptr` has type `Pointer<char8>`
TODO(docs)

##### `path_size`
`path_size` has type `memorysize`
TODO(docs)

##### `value_ptr_ptr`
`value_ptr_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_size_ptr`
`value_size_ptr` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_set_property
extern "C" WasmResult proxy_set_property(const char* path_ptr, size_t path_size,
                                         const char* value_ptr, size_t value_size);
TODO(docs)

#### Parameters:
##### `path_ptr`
`path_ptr` has type `Pointer<char8>`
TODO(docs)

##### `path_size`
`path_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<char8>`
TODO(docs)

##### `value_size`
`value_size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_set_effective_context
System
extern "C" WasmResult proxy_set_effective_context(uint32_t effective_context_id);
TODO(docs)

#### Parameters:
##### `effective_context_id`
`effective_context_id` has type `contextid`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_done
extern "C" WasmResult proxy_done();
TODO(docs)

#### Parameters:
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_define_metric
Metrics
extern "C" WasmResult proxy_define_metric(MetricType type, const char* name_ptr, size_t name_size,
                                          uint32_t* metric_id);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `metrictype`
TODO(docs)

##### `name_ptr`
`name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `name_size`
`name_size` has type `memorysize`
TODO(docs)

##### `metric_id`
`metric_id` has type `Pointer<metric>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_increment_metric
extern "C" WasmResult proxy_increment_metric(uint32_t metric_id, int64_t offset);
TODO(docs)

#### Parameters:
##### `metric_id`
`metric_id` has type `metric`
TODO(docs)

##### `offset`
`offset` has type `s64`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_record_metric
extern "C" WasmResult proxy_record_metric(uint32_t metric_id, uint64_t value);
TODO(docs)

#### Parameters:
##### `metric_id`
`metric_id` has type `metric`
TODO(docs)

##### `value`
`value` has type `u64`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_metric
extern "C" WasmResult proxy_get_metric(uint32_t metric_id, uint64_t* result);
TODO(docs)

#### Parameters:
##### `metric_id`
`metric_id` has type `metric`
TODO(docs)

##### `value`
`value` has type `Pointer<u64>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_continue_request
Continue/Reply/Route
extern "C" WasmResult proxy_continue_request();
TODO(docs)

#### Parameters:
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_continue_response
extern "C" WasmResult proxy_continue_response();
TODO(docs)

#### Parameters:
#### Results:
##### `result`
`result` has type `wasmresult`
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

##### `response_code_details_ptr`
`response_code_details_ptr` has type `Pointer<char8>`
TODO(docs)

##### `response_code_details_size`
`response_code_details_size` has type `memorysize`
TODO(docs)

##### `body_ptr`
`body_ptr` has type `Pointer<char8>`
TODO(docs)

##### `body_size`
`body_size` has type `memorysize`
TODO(docs)

##### `additional_response_header_pairs_ptr`
`additional_response_header_pairs_ptr` has type `Pointer<char8>`
TODO(docs)

##### `additional_response_header_pairs_size`
`additional_response_header_pairs_size` has type `memorysize`
TODO(docs)

##### `grpc_status`
`grpc_status` has type `grpcstatus`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_clear_route_cache
extern "C" WasmResult proxy_clear_route_cache();
TODO(docs)

#### Parameters:
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_shared_data
SharedData
extern "C" WasmResult proxy_get_shared_data(const char* key_ptr, size_t key_size,
                                            const char** value_ptr, size_t* value_size,
                                            uint32_t* cas);
Returns: Ok, NotFound
TODO(docs)

#### Parameters:
##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_size`
`value_size` has type `Pointer<memorysize>`
TODO(docs)

##### `cas`
`cas` has type `Pointer<u32>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_set_shared_data
extern "C" WasmResult proxy_set_shared_data(const char* key_ptr, size_t key_size,
                                            const char* value_ptr, size_t value_size, uint32_t cas);
//  If cas != 0 and cas != the current cas for 'key' return false, otherwise set the value and
//  return true.
// Returns: Ok, CasMismatch
TODO(docs)

#### Parameters:
##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<char8>`
TODO(docs)

##### `value_size`
`value_size` has type `memorysize`
TODO(docs)

##### `cas`
`cas` has type `u32`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
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
##### `queue_name_ptr`
`queue_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `queue_name_size`
`queue_name_size` has type `memorysize`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_resolve_shared_queue
extern "C" WasmResult proxy_resolve_shared_queue(const char* vm_id, size_t vm_id_size,
                                                 const char* queue_name_ptr, size_t queue_name_size,
                                                 uint32_t* token);
// Returns: Ok, NotFound
TODO(docs)

#### Parameters:
##### `vm_id`
`vm_id` has type `Pointer<char8>`
TODO(docs)

##### `vm_id_size`
`vm_id_size` has type `memorysize`
TODO(docs)

##### `queue_name_ptr`
`queue_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `queue_name_size`
`queue_name_size` has type `memorysize`
TODO(docs)

##### `token`
`token` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
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

##### `data_ptr`
`data_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `data_size`
`data_size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
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

##### `data_ptr`
`data_ptr` has type `Pointer<char8>`
TODO(docs)

##### `data_size`
`data_size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_add_header_map_value
Headers/Trailers/Metadata Maps
extern "C" WasmResult proxy_add_header_map_value(HeaderMapType type, const char* key_ptr,
                                                 size_t key_size, const char* value_ptr,
                                                 size_t value_size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<char8>`
TODO(docs)

##### `value_size`
`value_size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_header_map_value
extern "C" WasmResult proxy_get_header_map_value(HeaderMapType type, const char* key_ptr,
                                                 size_t key_size, const char** value_ptr,
                                                 size_t* value_size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `value_size`
`value_size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_header_map_pairs
extern "C" WasmResult proxy_get_header_map_pairs(HeaderMapType type, const char** ptr,
                                                 size_t* size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `ptr`
`ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `size`
`size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_set_header_map_pairs
extern "C" WasmResult proxy_set_header_map_pairs(HeaderMapType type, const char* ptr, size_t size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `ptr`
`ptr` has type `Pointer<char8>`
TODO(docs)

##### `size`
`size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_replace_header_map_value
extern "C" WasmResult proxy_replace_header_map_value(HeaderMapType type, const char* key_ptr,
                                                     size_t key_size, const char* value_ptr,
                                                     size_t value_size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)

##### `value_ptr`
`value_ptr` has type `Pointer<char8>`
TODO(docs)

##### `value_size`
`value_size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_remove_header_map_value
extern "C" WasmResult proxy_remove_header_map_value(HeaderMapType type, const char* key_ptr,
                                                    size_t key_size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `key_ptr`
`key_ptr` has type `Pointer<char8>`
TODO(docs)

##### `key_size`
`key_size` has type `memorysize`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_header_map_size
extern "C" WasmResult proxy_get_header_map_size(HeaderMapType type, size_t* size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `headermaptype`
TODO(docs)

##### `size`
`size` has type `Pointer<mapsize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_buffer_bytes
Buffer
extern "C" WasmResult proxy_get_buffer_bytes(BufferType type, uint32_t start, uint32_t length,
                                             const char** ptr, size_t* size);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `buffertype`
TODO(docs)

##### `start`
`start` has type `u32`
TODO(docs)

##### `length`
`length` has type `u32`
TODO(docs)

##### `ptr`
`ptr` has type `Pointer<Pointer<char8>>`
TODO(docs)

##### `size`
`size` has type `Pointer<memorysize>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_get_buffer_status
extern "C" WasmResult proxy_get_buffer_status(BufferType type, size_t* length_ptr,
                                              uint32_t* flags_ptr);
TODO(docs)

#### Parameters:
##### `type`
`type` has type `buffertype`
TODO(docs)

##### `length_ptr`
`length_ptr` has type `Pointer<u32>`
TODO(docs)

##### `flags_ptr`
`flags_ptr` has type `Pointer<bufferflags>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
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
##### `uri_ptr`
`uri_ptr` has type `Pointer<char8>`
TODO(docs)

##### `uri_size`
`uri_size` has type `memorysize`
TODO(docs)

##### `header_pairs_ptr`
`header_pairs_ptr` has type `Pointer<char8>`
TODO(docs)

##### `header_pairs_size`
`header_pairs_size` has type `memorysize`
TODO(docs)

##### `body_ptr`
`body_ptr` has type `Pointer<char8>`
TODO(docs)

##### `body_size`
`body_size` has type `memorysize`
TODO(docs)

##### `trailer_pairs_ptr`
`trailer_pairs_ptr` has type `Pointer<char8>`
TODO(docs)

##### `trailer_pairs_size`
`trailer_pairs_size` has type `memorysize`
TODO(docs)

##### `timeout_milliseconds`
`timeout_milliseconds` has type `u32`
TODO(docs)

##### `token_ptr`
`token_ptr` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
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
##### `service_ptr`
`service_ptr` has type `Pointer<char8>`
TODO(docs)

##### `service_size`
`service_size` has type `memorysize`
TODO(docs)

##### `service_name_ptr`
`service_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `service_name_size`
`service_name_size` has type `memorysize`
TODO(docs)

##### `method_name_ptr`
`method_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `method_name_size`
`method_name_size` has type `memorysize`
TODO(docs)

##### `request_ptr`
`request_ptr` has type `Pointer<char8>`
TODO(docs)

##### `request_size`
`request_size` has type `memorysize`
TODO(docs)

##### `timeout_milliseconds`
`timeout_milliseconds` has type `u32`
TODO(docs)

##### `token_ptr`
`token_ptr` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_grpc_stream
extern "C" WasmResult proxy_grpc_stream(const char* service_ptr, size_t service_size,
                                        const char* service_name_ptr, size_t service_name_size,
                                        const char* method_name_ptr, size_t method_name_size,
                                        uint32_t* token_ptr);
TODO(docs)

#### Parameters:
##### `service_ptr`
`service_ptr` has type `Pointer<char8>`
TODO(docs)

##### `service_size`
`service_size` has type `memorysize`
TODO(docs)

##### `service_name_ptr`
`service_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `service_name_size`
`service_name_size` has type `memorysize`
TODO(docs)

##### `method_name_ptr`
`method_name_ptr` has type `Pointer<char8>`
TODO(docs)

##### `method_name_size`
`method_name_size` has type `memorysize`
TODO(docs)

##### `token_ptr`
`token_ptr` has type `Pointer<token>`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_grpc_cancel
extern "C" WasmResult proxy_grpc_cancel(uint32_t token);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_grpc_close
extern "C" WasmResult proxy_grpc_close(uint32_t token);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

### proxy_grpc_send
extern "C" WasmResult proxy_grpc_send(uint32_t token, const char* message_ptr, size_t message_size,
                                      uint32_t end_stream);
TODO(docs)

#### Parameters:
##### `token`
`token` has type `token`
TODO(docs)

##### `message_ptr`
`message_ptr` has type `Pointer<char8>`
TODO(docs)

##### `message_size`
`message_size` has type `memorysize`
TODO(docs)

##### `end_stream`
`end_stream` has type `boolean`
TODO(docs)
#### Results:
##### `result`
`result` has type `wasmresult`
TODO(docs)

