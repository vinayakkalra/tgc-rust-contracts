set -e

dfx identity use default
DEFAULT=$(dfx --identity default identity get-principal)
USER=$(dfx --identity testing identity get-principal)
MINTER=$(dfx --identity minter identity get-principal)
RECIEVER=$(dfx --identity reciever identity get-principal)
CANISTER=$(dfx canister id tgc_rust)
echo "DEFAULT: $DEFAULT"
echo "USER: $USER"
echo "MINTER: $MINTER"
echo "RECIEVER: $RECIEVER"

function debug_print() {
    echo "State at checkpoint $1"
    echo "Balance of minter: $(dfx canister call ledger icrc1_balance_of "(record {owner = principal \"$MINTER\"})")"
    echo "Balance of default: $(dfx canister call ledger icrc1_balance_of "(record {owner = principal \"$DEFAULT\"})")"
    echo "Balance of user: $(dfx canister call ledger icrc1_balance_of "(record {owner = principal \"$USER\"})")"
    echo "Balance of reciever: $(dfx canister call ledger icrc1_balance_of "(record {owner = principal \"$RECIEVER\"})")"
}

# mint to user for testing
dfx --identity default canister call ledger icrc1_transfer "(record { to = record { owner = principal \"$USER\" }; amount = 1000000000 })"

# approve
dfx --identity testing canister call ledger icrc2_approve "(record { amount = 9999999999999999; spender = record { owner = principal \"$CANISTER\"} })"

echo "getPaymentRecipient: $(dfx canister call tgc_rust getPaymentRecipient)"
echo "Initial Session Time: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"

debug_print 1
# Buying
dfx canister call tgc_rust buySession "(600, principal \"$USER\", 100000000)"
echo "After Buying: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"

debug_print 2

# before start pausing
echo "Before start: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"
echo "Before start check session: $(dfx canister call tgc_rust checkSession "(principal \"$USER\")")"

# Start Session
dfx canister call tgc_rust startSession "(principal \"$USER\")"
echo "Started Session"

# after start pausing
echo "after start: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"
echo "after start check session: $(dfx canister call tgc_rust checkSession "(principal \"$USER\")")"

# Sleep to test time elapsed
sleep 10
echo "Slept for 10 seconds"

# Before pausing
echo "Before pausing: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"
echo "Before pausing check session: $(dfx canister call tgc_rust checkSession "(principal \"$USER\")")"

# Pause Session
dfx canister call tgc_rust pauseSession "(principal \"$USER\")"

# Get session time left and check the session
echo "After pausing: $(dfx canister call tgc_rust getSessionTimeLeft "(principal \"$USER\")")"
echo "After pausing check session: $(dfx canister call tgc_rust checkSession "(principal \"$USER\")")"

# debug_print 3