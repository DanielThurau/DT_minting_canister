use ic_cdk_macros::{init, query, pre_upgrade, post_upgrade};
use ic_canisters_http_types::{HttpRequest, HttpResponse, HttpResponseBuilder};

use ic_nervous_system_common::{
    serve_logs_v2, serve_logs,
};
use logs::{INFO, ERROR};

mod types;
mod logs;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[init]
fn canister_init(init_payload: types::InitPayload) {
    c_log!(INFO, "Executing the `canister_init` canister entry point with payload: ({:?})", init_payload);
}

#[pre_upgrade]
fn canister_pre_upgrade() {
    c_log!(INFO, "Executing the `canister_pre_upgrade` canister entry point.") ;
}

#[post_upgrade]
fn canister_post_upgrade() {
    c_log!(INFO, "Executing the `canister_post_upgrade` canister entry point.");
}

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    match request.path() {
        "/metrics" => HttpResponseBuilder::not_found().build(),
        "/logs" => serve_logs_v2(request, &INFO, &ERROR),

        // These are obsolete.
        "/log/info" => serve_logs(&INFO),
        "/log/error" => serve_logs(&ERROR),

        _ => HttpResponseBuilder::not_found().build(),
    }
}


