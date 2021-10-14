# simple-sns

## Development

### Prepare token

```bash
# Clone the extendable-token repository as a sibling of the simple-sns repository
git clone git@github.com:Toniq-Labs/extendable-token.git

# Find the principal of the identity you would like to send the initial supply of the token to
# In this case, I have chosen l3x6q-t2lwb-aryfe-q5guh-5a2ry-vs6ie-mb6ic-phtci-ltcnz-ulj25-jae
dfx deploy token --argument="(\"Token\", \"TOK\", 3, 1000000:nat, principal \"l3x6q-t2lwb-aryfe-q5guh-5a2ry-vs6ie-mb6ic-phtci-ltcnz-ulj25-jae\")"
```

### Prepare governance canister

```bash
dfx deploy governance

# Find the canister id of the previously-created token canister
# In this case, the token canister id is: renrk-eyaaa-aaaaa-aaada-cai
# The parameters are as follows: num_tokens_to_create_proposal, target_canister_principal, threshold_to_adopt, token_principal
# Look at the configure function in the canister to understand these parameters better
dfx canister call governance configure '(1, null, .5, "renrk-eyaaa-aaaaa-aaada-cai")'
```

### Proposals

```bash
# Creates a proposal to create an empty canister
dfx canister call governance create_proposal '(variant { CANISTER_CREATE }, "Create a canister", "https://google.com", blob "")'
```

### Votes

```bash
# This casts an adopt vote for proposal with id 7ktvvp-sjrui-i4xf3-j7nxy-z4x6s-zsulr-nluy5-wcd5t-qnqc
dfx canister call governance cast_vote '("7ktvvp-sjrui-i4xf3-j7nxy-z4x6s-zsulr-nluy5-wcd5t-qnqc", true)'
```