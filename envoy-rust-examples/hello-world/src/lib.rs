use envoy_rust_abi as envoy;

#[no_mangle]
pub extern fn malloc(_size: u32) -> u32 {
   panic!("not implemented yet")
}

#[no_mangle]
pub extern fn free(_size: u32) {
   panic!("not implemented yet")
}

#[no_mangle]
pub extern fn proxy_on_start(context: envoy::ContextId, config_len: envoy::Size) -> envoy::Boolean {
    unsafe {
        match envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_start( context={}, config_len={} )", context, config_len) ) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern fn proxy_validate_configuration(context: envoy::ContextId, config_len: envoy::Size) -> envoy::Boolean {
    unsafe {
        match envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_validate_configuration( context={}, config_len={} )", context, config_len) ) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern fn proxy_on_configure(context: envoy::ContextId, config_len: envoy::Size) -> envoy::Boolean {
    unsafe {
        match envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_configure( context={}, config_len={} )", context, config_len) ) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern fn proxy_on_create(context: envoy::ContextId, root_context: envoy::ContextId) {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_create( context={}, root_context={} )", context, root_context) ).unwrap();
    }
}

#[no_mangle]
pub extern fn proxy_on_request_headers(context: envoy::ContextId, headers_count: envoy::MapSize) -> envoy::FilterHeadersStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_request_headers( context={}, headers_count={} )", context, headers_count) ).unwrap();
    }
    envoy::FILTER_HEADERS_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_request_body(context: envoy::ContextId, body_buffer_length: envoy::Size, end_of_stream: envoy::Boolean) -> envoy::FilterDataStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_request_body( context={}, body_buffer_length={}, end_of_stream={} )", context, body_buffer_length, end_of_stream) ).unwrap();
    }
    envoy::FILTER_DATA_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_request_trailers(context: envoy::ContextId, trailers_count: envoy::MapSize) -> envoy::FilterTrailersStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_request_trailers( context={}, trailers_count={} )", context, trailers_count) ).unwrap();
    }
    envoy::FILTER_TRAILERS_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_request_metadata(context: envoy::ContextId, elements_count: envoy::MapSize) -> envoy::FilterMetadataStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_request_metadata( context={}, elements_count={} )", context, elements_count) ).unwrap();
    }
    envoy::FILTER_METADATA_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_response_headers(context: envoy::ContextId, headers_count: envoy::MapSize) -> envoy::FilterHeadersStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_response_headers( context={}, headers_count={} )", context, headers_count) ).unwrap();
    }
    envoy::FILTER_HEADERS_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_response_body(context: envoy::ContextId, body_buffer_length: envoy::Size, end_of_stream: envoy::Boolean) -> envoy::FilterDataStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_response_body( context={}, body_buffer_length={}, end_of_stream={} )", context, body_buffer_length, end_of_stream) ).unwrap();
    }
    envoy::FILTER_DATA_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_response_trailers(context: envoy::ContextId, trailers_count: envoy::MapSize) -> envoy::FilterTrailersStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_response_trailers( context={}, trailers_count={} )", context, trailers_count) ).unwrap();
    }
    envoy::FILTER_TRAILERS_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_response_metadata(context: envoy::ContextId, elements_count: envoy::MapSize) -> envoy::FilterMetadataStatus {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_response_metadata( context={}, elements_count={} )", context, elements_count) ).unwrap();
    }
    envoy::FILTER_METADATA_STATUS_CONTINUE
}

#[no_mangle]
pub extern fn proxy_on_done(context: envoy::ContextId) -> envoy::Boolean {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_done( context={} )", context) ).unwrap();
    }
    1
}

#[no_mangle]
pub extern fn proxy_on_log(context: envoy::ContextId) {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_log( context={} )", context) ).unwrap();
    }
}

#[no_mangle]
pub extern fn proxy_on_delete(context: envoy::ContextId) {
    unsafe {
        envoy::proxy_log(envoy::LOG_LEVEL_INFO, &format!("proxy_on_delete( context={} )", context) ).unwrap();
    }
}
