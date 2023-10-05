set -e

# Create identities
dfx identity new minter --disable-encryption || true
dfx identity new reciever --disable-encryption || true
dfx identity new testing --disable-encryption || true

dfx identity use default

MINTER=$(dfx --identity minter identity get-principal)
DEFAULT=$(dfx --identity default identity get-principal)
RECIEVER=$(dfx --identity reciever identity get-principal)
TOKEN_SYMBOL=TOK
TOKEN_NAME="fancy_token_name"
TRANSFER_FEE=100
PRE_MINTED_TOKENS=100000000000

dfx deploy ledger --argument "(variant {Init = 
record {
     token_symbol = \"${TOKEN_SYMBOL}\";
     token_name = \"${TOKEN_NAME}\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = ${TRANSFER_FEE};
     metadata = vec {};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; ${PRE_MINTED_TOKENS}; }; };
     archive_options = record {
         num_blocks_to_archive = 1000000000;
         trigger_threshold = 1000000000;
         controller_id = principal \"${DEFAULT}\";
     };
     feature_flags = opt record {icrc2 = true;};
 }
})"
dfx canister stop tgc_rust || true
dfx canister delete tgc_rust --yes --no-withdrawal || true
dfx deploy --argument "record{payment_recipient= principal \"$RECIEVER\"}" tgc_rust