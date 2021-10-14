// TODO there are tools that can generate all of these types from a GraphQL schema

export type Proposal = Readonly<{
    id: string;
    author_principal: string;
    num_tokens_adopted: number;
    num_tokens_rejected: number;
    proposal_type: 'CANISTER_CREATE' | 'CANISTER_UPGRADE' | 'CONFIG_UPDATE';
    title: string;
    url: string;
    votes: ReadonlyArray<Vote>;
    wasm_module: Uint8Array;
}>;

export type Vote = Readonly<{
    id: string;
    adopt: boolean;
    num_tokens: number;
    proposal: Proposal;
    voter_principal: string;
}>;