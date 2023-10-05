use std::collections::HashMap;
use candid::{self, CandidType, Deserialize, Principal};

#[derive(Clone, CandidType, Deserialize, Debug, Default)]
pub struct GameBlockNumber {
    payment_recipient: Option<Principal>, // payment recipient for purchasing sessions

    pub session_times: HashMap<Principal, u64>, // store valid session durations
    pub session_started_at: HashMap<Principal, u64>, // store session started at for users
}

// impl Default for GameBlockNumber {
//     fn default() -> Self {
//         Self {
//             payment_recipient: None,
//             session_times: HashMap::new(),
//             session_started_at: HashMap::new(),
//         }
//     }
// }

impl GameBlockNumber {
   
    pub fn get_payment_recipient(&self) -> Principal {
        self.payment_recipient.unwrap()
    }

    pub fn set_payment_recipient(&mut self, principal: Principal) {
        self.payment_recipient = Some(principal);
    }

    pub fn get_session_time_left(&self, user_principal: Principal, now: u64) -> u64 {

        let session_time = self.session_times.get(&user_principal).unwrap_or(&0);
        let elapsed_seconds = if let Some(session_started_at) = self.session_started_at.get(&user_principal) {
            (now / 1_000_000_000).saturating_sub(*session_started_at)
        } else {
            0
        };
        session_time.saturating_sub(elapsed_seconds) // >=0

    }

    pub fn buy_session(&mut self, time_to_buy: u64, user_principal: Principal, _token_to_charge: u64){
        
        let new_time = self.session_times.get(&user_principal).unwrap_or(&0) + time_to_buy;
        self.session_times.insert(user_principal, new_time);    
        
    }

    pub fn start_session(&mut self,user_principal: Principal, time: u64) {
    
        self.session_started_at.insert(user_principal, time / 1_000_000_000 );      
    
    }

    pub fn pause_session(&mut self, user_principal: Principal, now: u64) {
    
        let session_time_left = self.get_session_time_left(user_principal, now);
        
        if session_time_left > 0 {
            self.session_times.insert(user_principal, session_time_left);    
        } else{
            self.session_times.remove(&user_principal);
        }
        self.session_started_at.remove(&user_principal);
    }

    pub fn check_session(&self, user_principal: Principal, now: u64) -> (u64, bool) {
        let session_time_left = self.get_session_time_left(user_principal, now);
        (session_time_left, session_time_left > 0)
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    const SECONDS: u64 = 1_000_000_000;

    fn get_principal() -> Principal {
        Principal::from_text("bxquz-fu76r-igixs-bw537-mgkyg-h4goq-ldrwe-q4zg2-zxtqy-zupgm-nqe").unwrap()
    }

    #[test]
    fn test_payment_recipient() {
        let mut state = GameBlockNumber::default();
        state.set_payment_recipient(get_principal());
        assert_eq!(state.payment_recipient.unwrap(), get_principal());
    }

    #[test]
    fn test_buy_session() {
        let mut state = GameBlockNumber::default();
        state.buy_session(1000000, get_principal(), 0);
        assert_eq!(state.get_session_time_left(get_principal(), 1695732484), 1000000);
    }

    #[test]
    fn test_start_session() {
        let mut state = GameBlockNumber::default();
        state.buy_session(1000000, get_principal(), 0);
        state.start_session( get_principal(), 100 * SECONDS);
        assert_eq!(state.check_session(get_principal(), 100 * SECONDS).1, true);
        assert_eq!(state.session_started_at.get(&get_principal()).unwrap_or(&0), &(100 as u64) )
    }

    #[test]
    fn test_pause_session() {
        let mut state = GameBlockNumber::default();
        state.pause_session( get_principal(), 1695732484);
        assert_eq!(state.check_session(get_principal(), 1695732484).1, false);
    }

    #[test]
    fn test_start_session_variant() {



        let mut state = GameBlockNumber::default();
        
        assert_eq!(state.check_session(get_principal(), 0), (0, false));

        state.buy_session(1_000_000, get_principal(), 0);

        assert_eq!(state.check_session(get_principal(), 0), (1_000_000, true));

        state.start_session( get_principal(), 100 * SECONDS);

        assert_eq!(state.check_session(get_principal(), 200 * SECONDS), (999_900, true));
        
        state.buy_session(1_000_000, get_principal(), 0);

        assert_eq!(state.check_session(get_principal(), 200 * SECONDS), (1_999_900, true));

        assert_eq!(state.check_session(get_principal(), 300 * SECONDS), (1_999_800, true));

        state.pause_session( get_principal(), 300 * SECONDS);

        assert_eq!(state.check_session(get_principal(), 300 * SECONDS), (1_999_800, true));
        
        assert_eq!(state.session_started_at.get(&get_principal()), None);

        assert_eq!(state.check_session(get_principal(), 400 * SECONDS), (1_999_800, true));

    }

    
    

}