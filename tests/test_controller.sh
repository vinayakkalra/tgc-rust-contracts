# dfx stop
# set -e
# dfx start --clean --background

dfx identity use minter
MINTER=$(dfx identity get-principal)
dfx identity use default
DEFAULT=$(dfx identity get-principal)
dfx deploy
OWNER=$(dfx canister call tgc_rust getPrincipal)
echo "OWNER: $OWNER"
dfx identity use minter
dfx canister call tgc_rust setOwner "(principal \"$MINTER\")"
dfx identity use default
dfx canister update-settings tgc_rust --add-controller $MINTER
dfx identity use minter
dfx canister call tgc_rust setOwner "(principal \"$MINTER\")"
echo "MINTER: $MINTER"
OWNER=$(dfx canister call tgc_rust getPrincipal)
echo "OWNER: $OWNER"