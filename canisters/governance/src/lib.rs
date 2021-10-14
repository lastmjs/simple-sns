// TODO get rid of all unwraps, use results and ?
// TODO dealing with the number types needs to be more robust, I am doing horrific string parsing and casting conversions
// TODO simplify the graphql types and such with some generic functions
// TODO I do not have to grab all fields every time, since I did figure out the default problem with the generated Rust types
// TODO split all graphql queries and mutations into their own functions

sudograph::graphql_database!("canisters/governance/src/schema.graphql");

#[derive(candid::CandidType, serde::Deserialize)]
struct CreateCanisterRecord {
    canister_id: candid::Principal
}

#[derive(candid::CandidType)]
enum User {
    address(String),
    principal(candid::Principal)
}

#[derive(candid::CandidType)]
struct BalanceRequest {
    user: User,
    token: String
}

#[derive(candid::CandidType, serde::Deserialize, Debug)]
enum CommonError {
    InvalidToken(String),
    Other(String)
}

#[derive(candid::CandidType, serde::Deserialize, Debug)]
enum BalanceResponse {
    ok(candid::Nat),
    err(CommonError)
}

#[derive(candid::CandidType, Deserialize)]
enum UpdateCanisterMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade
}

#[derive(candid::CandidType, Deserialize)]
struct UpdateCanisterArgs {
    mode: UpdateCanisterMode,
    canister_id: candid::Principal,
    wasm_module: Vec<u8>,
    arg: Vec<u8>,
    compute_allocation: Option<u64>,
    memory_allocation: Option<u64>
}

#[derive(serde::Deserialize)]
struct GQLResult<T> {
    data: T,
    errors: Option<Vec<String>>
}

#[ic_cdk_macros::update]
async fn configure(
    num_tokens_to_create_proposal: candid::Int,
    target_canister_id: Option<String>,
    threshold_percentage_to_adopt: f64,
    token_principal: String
) -> String {
    // TODO somehow permission this I think...right now anyone can call the initial configuration
    // TODO perhaps this should just be done when the canister is first deployed, with arguments of some kind

    if let Some(_) = get_config().await {
        return gql_error("You can only call configure once. All subsequent changes to the config must be done through proposals.");
    }

    let result = graphql_mutation(
        "
            mutation (
                $num_tokens_to_create_proposal: Int!
                $target_canister_id: String
                $threshold_percentage_to_adopt: Int!
                $token_principal: String!
            ) {
                createConfig(input: {
                    num_tokens_to_create_proposal: $num_tokens_to_create_proposal
                    target_canister_id: $target_canister_id
                    threshold_percentage_to_adopt: $threshold_percentage_to_adopt
                    token_principal: $token_principal
                }) {
                    id
                }
            }
        ".to_string(),
        serde_json::json!({
            "num_tokens_to_create_proposal": num_tokens_to_create_proposal.0.to_string().parse::<i32>().unwrap(), // TODO this is horrendous
            "target_canister_id": if let Some(target_canister_id) = target_canister_id { serde_json::Value::String(target_canister_id) } else { serde_json::Value::Null },
            "threshold_percentage_to_adopt": threshold_percentage_to_adopt,
            "token_principal": token_principal
        }).to_string()
    ).await;

    return result;
}

#[ic_cdk_macros::update]
async fn create_proposal(
    proposal_type: ProposalType,
    title: String,
    url: String,
    wasm_module: Vec<u8>
) -> String {
    if let Some(config) = get_config().await {
        let caller_principal = ic_cdk::caller();

        let balance = get_balance(
            candid::Principal::from_text(config.token_principal).unwrap(),
            caller_principal
        ).await;

        if balance < config.num_tokens_to_create_proposal {
            return gql_error(&format!(
                "You do not have enough tokens to create a proposal. {num_tokens_to_create_proposal} token(s) required.",
                num_tokens_to_create_proposal = config.num_tokens_to_create_proposal
            ));
        }

        let result = graphql_mutation(
            "
                mutation (
                    $author_principal: String!
                    $proposal_type: ProposalType!
                    $title: String!
                    $url: String!
                    $wasm_module: Blob!
                ) {
                    createProposal(input: {
                        author_principal: $author_principal
                        num_tokens_adopted: 0
                        num_tokens_rejected: 0
                        proposal_type: $proposal_type
                        title: $title
                        url: $url
                        wasm_module: $wasm_module
                    }) {
                        id
                    }
                }
            ".to_string(),
            serde_json::json!({
                "author_principal": caller_principal.to_string(),
                "proposal_type": proposal_type,
                "title": title,
                "url": url,
                "wasm_module": wasm_module
            }).to_string()
        ).await;
    
        return result;
    }
    else {
        return gql_error("config has not been set");
    }
}

#[ic_cdk_macros::update]
async fn cast_vote(
    proposal_id: String,
    adopt: bool
) -> String {
    let vote_already_cast = was_vote_already_cast(
        &proposal_id,
        &ic_cdk::caller().to_string()
    ).await;

    if vote_already_cast == true {
        return gql_error("You can only vote once per proposal.");
    }

    if let Some(config) = get_config().await {
        let caller_principal = ic_cdk::caller();

        let balance = get_balance(
            candid::Principal::from_text(&config.token_principal).unwrap(),
            caller_principal
        ).await;

        if balance == 0 {
            return gql_error("You must have more than 0 tokens to vote.");
        }

        let proposal = get_proposal(&proposal_id).await;

        let num_tokens_adopted = if adopt == true { proposal.num_tokens_adopted + balance } else { proposal.num_tokens_adopted };
        let num_tokens_rejected = if adopt == true { proposal.num_tokens_rejected } else { proposal.num_tokens_rejected - balance };

        let create_vote_string = graphql_mutation(
            "
                mutation (
                    $adopt: Boolean!
                    $num_tokens: Int!
                    $num_tokens_adopted: Int!
                    $num_tokens_rejected: Int!
                    $proposal_id: ID!
                    $voter_principal: String!
                ) {
                    createVote(input: {
                        adopt: $adopt
                        num_tokens: $num_tokens
                        proposal: {
                            connect: $proposal_id
                        }
                        voter_principal: $voter_principal
                    }) {
                        id
                    }

                    updateProposal(input: {
                        id: $proposal_id
                        num_tokens_adopted: $num_tokens_adopted
                        num_tokens_rejected: $num_tokens_rejected
                    }) {
                        id
                    }
                }
            ".to_string(),
            serde_json::json!({
                "adopt": adopt,
                "num_tokens": balance,
                "num_tokens_adopted": num_tokens_adopted,
                "num_tokens_rejected": num_tokens_rejected,
                "proposal_id": proposal_id,
                "voter_principal": caller_principal
            }).to_string()
        ).await;

        let token_supply = get_supply(
            candid::Principal::from_text(&config.token_principal).unwrap()
        ).await;

        // TODO this math is probably very imprecise, figure out robust floating point operations
        if num_tokens_adopted as f32 >= token_supply as f32 * config.threshold_percentage_to_adopt {
            // TODO I don't think I'll need it again
            // TODO probably get just what I need
            // let proposal = get_proposal(&proposal_id).await;

            adopt_proposal(
                &config,
                proposal
            ).await;
        }
    
        return create_vote_string;
    }
    else {
        return gql_error("config has not been set");
    }
}

async fn adopt_proposal(
    config: &Config,
    proposal: Proposal
) {
    ic_cdk::println!("adopt_proposal: {:#?}", proposal);

    match proposal.proposal_type {
        ProposalType::CANISTER_CREATE => {
            let canister_id = create_canister().await;
            
            update_canister(
                UpdateCanisterMode::Install,
                &canister_id,
                proposal.wasm_module.0
            ).await;

            update_target_canister_id(
                &config.id.to_string(),
                &canister_id
            ).await;

            ic_cdk::println!("created canister_id: {}", canister_id);
        },
        ProposalType::CANISTER_UPGRADE => {
            if let Some(canister_id) = &config.target_canister_id {
                update_canister(
                    UpdateCanisterMode::Install,
                    canister_id,
                    proposal.wasm_module.0
                ).await;
            }
            else {
                panic!("config.target_canister_id is not set");
            }
        },
        ProposalType::CONFIG_UPDATE => {
            // TODO allow proposals to have all of the things that can be changed in the config
            // TODO then simply update the config with what is in the proposal
        }
    };
}

async fn create_canister() -> String {
    let create_canister_result: ic_cdk::api::call::CallResult<(CreateCanisterRecord,)> =
        ic_cdk::call(
            candid::Principal::management_canister(),
            "create_canister",
            ()
        ).await;
    
    let (create_canister_record,) = create_canister_result.unwrap();

    return create_canister_record.canister_id.to_string();
}

async fn update_canister(
    update_canister_mode: UpdateCanisterMode,
    canister_id: &str,
    wasm_module: Vec<u8>
) {
    let update_canister_args = UpdateCanisterArgs {
        mode: update_canister_mode,
        canister_id: candid::Principal::from_text(canister_id).unwrap(),
        wasm_module,
        arg: Vec::new(),
        compute_allocation: None,
        memory_allocation: None
    };

    // TODO check to make sure this result is correct
    let update_canister_result: ic_cdk::api::call::CallResult<()> =
        ic_cdk::call(
            candid::Principal::management_canister(),
            "install_code",
            (update_canister_args,)
        ).await;

    ic_cdk::println!("update_canister_result: {:#?}", update_canister_result);
}

async fn update_target_canister_id(
    config_id: &str,
    target_canister_id: &str
) {
    // TODO check the result for errors
    let result_string = graphql_mutation(
        "
            mutation (
                $config_id: ID!
                $target_canister_id: String!
            ) {
                updateConfig(input: {
                    id: $config_id
                    target_canister_id: $target_canister_id
                }) {
                    id
                }
            }
        ".to_string(),
        serde_json::json!({
            "config_id": config_id,
            "target_canister_id": target_canister_id
        }).to_string()
    ).await;

    // TODO we should check that the mutation worked appropriately, probably return a result here
    ic_cdk::println!("update_target_canister_id result_string: {}", result_string);
}

async fn get_config() -> Option<Config> {
    let configs_string = graphql_query(
        "
            query {
                readConfig {
                    id
                    num_tokens_to_create_proposal
                    target_canister_id
                    threshold_percentage_to_adopt
                    token_principal
                }
            }
        ".to_string(),
        serde_json::json!({}).to_string()
    ).await;

    #[derive(serde::Deserialize)]
    struct Result {
        readConfig: Vec<Config>
    }

    let configs_result: GQLResult<Result> = serde_json::from_str(&configs_string).unwrap();

    let config_option = configs_result.data.readConfig.get(0);

    match config_option {
        Some(config) => Some(config.clone()),
        None => None
    }
}

async fn get_proposal(id: &str) -> Proposal {
    // TODO grabbing everything like this all of the time will be
    // TODO very innefficient...figure out the deserialization so that
    // TODO we can leave out fields...might need defaults
    let proposals_string = graphql_query(
        "
            query ($id: ID!) {
                readProposal(search: {
                    id: {
                        eq: $id
                    }
                }) {
                    num_tokens_adopted
                    num_tokens_rejected
                    proposal_type
                    wasm_module
                }
            }
        ".to_string(),
        serde_json::json!({
            "id": id
        }).to_string()
    ).await;

    ic_cdk::println!("proposals_string {}", proposals_string);

    #[derive(serde::Deserialize)]
    struct Result {
        readProposal: Vec<Proposal>
    }

    let proposals_result: GQLResult<Result> = serde_json::from_str(&proposals_string).unwrap();

    let proposal = proposals_result.data.readProposal.get(0).unwrap().clone();

    return proposal;
}

async fn was_vote_already_cast(
    proposal_id: &str,
    voter_principal: &str
) -> bool {
    let votes_string = graphql_query(
        "
            query (
                $proposal_id: String!
                $voter_principal: String!
            ) {
                readVote(search: {
                    proposal: {
                        id: {
                            eq: $proposal_id
                        }
                    }
                    voter_principal: {
                        eq: $voter_principal
                    }
                }) {
                    id
                }
            }
        ".to_string(),
        serde_json::json!({
            "proposal_id": proposal_id,
            "voter_principal": voter_principal
        }).to_string()
    ).await;

    #[derive(serde::Deserialize)]
    struct Result {
        readVote: Vec<Vote>
    }

    let votes_result: GQLResult<Result> = serde_json::from_str(&votes_string).unwrap();

    let vote_option = votes_result.data.readVote.get(0);

    match vote_option {
        Some(_) => true,
        None => false
    }
}

// #[ic_cdk_macros::query]
async fn read_proposals() -> Vec<Proposal> {
    let proposals_string = graphql_query(
        "
            query {
                readProposal {
                    id
                    title
                }
            }
        ".to_string(),
        "{}".to_string()
    ).await;

    #[derive(serde::Deserialize)]
    struct ProposalGQLResult {
        readProposal: Vec<Proposal>
    }

    let proposals_result: GQLResult<ProposalGQLResult> = serde_json::from_str(&proposals_string).unwrap();

    let proposals = proposals_result.data.readProposal;

    ic_cdk::println!("proposals: {:#?}", proposals);

    return proposals;
}

async fn get_balance(
    token_principal: candid::Principal,
    user_principal: candid::Principal
) -> i32 {
    let balance_response_result: ic_cdk::api::call::CallResult<(BalanceResponse,)> =
        ic_cdk::call(
            token_principal,
            "balance",
            (
                BalanceRequest {
                    user: User::principal(user_principal),
                    token: "0".to_string()
                },
            )
        ).await;

    let (balance_response,) = balance_response_result.unwrap(); // TODO fix this unwrap

    match balance_response {
        BalanceResponse::ok(balance) => balance.0.to_string().parse::<i32>().unwrap(), // TODO horrendous conversion
        BalanceResponse::err(_) => panic!("error") // TODO fix this panic
    }
}

async fn get_supply(token_principal: candid::Principal) -> i32 {
    let balance_response_result: ic_cdk::api::call::CallResult<(BalanceResponse,)> =
        ic_cdk::call(
            token_principal,
            "supply",
            ("0".to_string(),)
        ).await;

    let (balance_response,) = balance_response_result.unwrap(); // TODO fix this unwrap

    match balance_response {
        BalanceResponse::ok(balance) => balance.0.to_string().parse::<i32>().unwrap(), // TODO horrendous conversion
        BalanceResponse::err(_) => panic!("error") // TODO fix this panic
    }
}

fn gql_error(message: &str) -> String {
    return serde_json::json!({
        "data": null,
        "errors": [
            {
                "message": message
            }
        ]
    }).to_string();
}