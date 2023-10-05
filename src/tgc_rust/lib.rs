mod state_handler;
use candid::{Principal, CandidType, Deserialize};
use ic_cdk::{query, update, init};
use std::cell::RefCell;
use state_handler::GameBlockNumber;
mod declarations;
use declarations::ledger::{ledger, TransferFromArgs, Account, TransferFromResult};

thread_local! {
    static STATE: RefCell<GameBlockNumber> = RefCell::new(GameBlockNumber::default());
}

// Initial argument structure to be used while deployment
#[derive(CandidType, Deserialize)]
struct InitArgs {
    payment_recipient: Principal // payment recipient principal address
}

// Initialization function running on deployment
#[init]
fn init(args: InitArgs) {
    
    // ic_cdk::print("!!! Calling init");
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.set_payment_recipient(args.payment_recipient);
    })
}

// private function to check if the caller is one of the controllers of the canister
fn check_admin() {
    if !ic_cdk::api::is_controller(&ic_cdk::api::caller()){
        ic_cdk::api::trap("This user is unauthorised to use this function");
    }
}

// get the payment recipient principal 
#[query(name="getPaymentRecipient")]
fn get_payment_recipient() -> Principal {
    STATE.with(|state| {
        let state = state.borrow();
        state.get_payment_recipient()
    })
}

// Buy a session 
#[update(name="buySession")]
async fn buy_session(time_to_buy: u64, user_principal: Principal, token_to_charge: u64) {
    
    check_admin();

    transfer(token_to_charge, user_principal).await;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.buy_session(time_to_buy, user_principal, token_to_charge);
    })  
        

}

async fn transfer(token_to_charge: u64, user_principal: Principal) {
    let payment_recipient = STATE.with(|state| {
        let state = state.borrow();
        state.get_payment_recipient()
    });
    
    ic_cdk::println!("Transferring {} tokens to principal {}", token_to_charge, payment_recipient);
    let transfer_args = TransferFromArgs {
        amount: token_to_charge.into(),
        to: Account { owner: payment_recipient, subaccount: None },
        fee: None,
        memo: None,
        created_at_time: None,
        spender_subaccount: None,
        from: Account { owner: user_principal, subaccount: None },
    };
    
    match ledger.icrc_2_transfer_from(transfer_args).await.unwrap() {
        (TransferFromResult::Ok(_),) => ic_cdk::println!("Okay"),
        (TransferFromResult::Err(e),) => ic_cdk::api::trap(&format!("{:?}", e))
    };

}

// Start a session
#[update(name="startSession")]
fn start_session(user_principal: Principal) {
    
    check_admin();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.start_session(user_principal, ic_cdk::api::time() );
    })

}

// Pause a session
#[update(name="pauseSession")]
fn pause_session(user_principal: Principal) {
    
    check_admin();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.pause_session(user_principal, ic_cdk::api::time());
    })
    
}

// Check a session
#[query(name="checkSession")]
fn check_session(user_principal: Principal) -> (u64, bool) {
    STATE.with(|state| {
        let state = state.borrow();
        state.check_session(user_principal, ic_cdk::api::time())
    })
}

// Update a session with payment - Same as buy session
// Todo: Delete?
#[update(name="updateSession")]
async fn update_session(time_to_buy: u64, user_principal: Principal, token_to_charge: u64) {
    
    check_admin();

    // Confirm this function with TGC Team
    
    // let session_time_left = get_session_time_left(user_principal);
    // let new_time = session_time_left + time_to_buy;
    // STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    //     state.session_times.insert(user_principal, new_time);
    // })

    buy_session(time_to_buy, user_principal, token_to_charge).await;
}

// Get total session time left
#[query(name="getSessionTimeLeft")]
fn get_session_time_left(user_principal: Principal) -> u64 {
    
    STATE.with(|state| {
        let state = state.borrow();
        state.get_session_time_left(user_principal, ic_cdk::api::time())
    })
}