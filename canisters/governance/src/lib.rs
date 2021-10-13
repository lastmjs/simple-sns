#[derive(candid::CandidType, serde::Deserialize)]
struct CreateCanisterRecord {
    canister_id: candid::Principal
}

#[ic_cdk_macros::update]
async fn create_canister() -> String {
    let create_canister_result: ic_cdk::api::call::CallResult<(CreateCanisterRecord,)> =
        ic_cdk::call(
            candid::Principal::management_canister(),
            "create_canister",
            ()
        ).await;
    
    let (create_canister_record,) = create_canister_result.unwrap();

    ic_cdk::println!("canister_id: {}", create_canister_record.canister_id);

    return create_canister_record.canister_id.to_string();
}