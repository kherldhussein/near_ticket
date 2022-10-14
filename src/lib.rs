use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
// use near_sdk::bs58::encode::Error;
use near_sdk::{env, log, Promise};
use std::collections::HashMap;

pub type AccountId = String;
pub type TicketId = String;
pub type EventId = String;

// Event creator && Ticket booking app for the Events

#[derive(Clone, BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct User {
  pub user_id: String,
}

impl User {
  fn new(user_id: String) -> Self {
    User { user_id }
  }
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct Event {
  description: String,
  price: i32,
  venue: String,
  // ticket_id: EventId,
  status: Status,
  mounts_tickets: i32,
  // ticket_type: String,
  event_organizer: AccountId,
  eid: u32,
}

impl Event {
  fn new(
    description: String,
    price: i32,
    venue: String,
    mounts_tickets: i32,
    // ticket_id: EventId,
    status: Status,
    // ticket_type: String,
    event_organizer: AccountId,
    eid: u32,
  ) -> Self {
    Event {
      description,
      price,
      venue,
      // ticket_id,
      mounts_tickets,
      status,
      // ticket_type,
      event_organizer,
      eid,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone)]
// #[serde(crate = "near_sdk::serde")]
pub struct Ticket {
  ticket_id: TicketId,
  ticket_owner: String,
  ticket_description: String,
  // Standard(Option<i32>),
  // Vip(Option<i32>),
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone, Copy)]
// #[serde(crate = "near_sdk::serde")]
pub enum Status {
  Available,
  Unavailable,
}

// impl Status {
//   fn get_status(&mut self) -> Self {

//     match self {
//       Status::Available => env::log_str("Ticket Available"),
//       Status::Unavailable => env::log_str("Ticket Not Available"),
//     }
//   }
// }

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct Booking {
  // status: Status,
  uid: AccountId,
  ticket: HashMap<AccountId, String>,
  users: HashMap<String, User>,
  event_creator: LookupMap<AccountId, Vec<EventId>>,
  creator_events: HashMap<EventId, Event>,
}

impl Booking {
  // #[init]
  pub fn new(uid: AccountId) -> Self {
    let users: HashMap<String, User> = HashMap::new();
    let ticket: HashMap<AccountId, String> = HashMap::new();
    let event_creator: LookupMap<AccountId, Vec<EventId>> = LookupMap::new(b"m");
    let creator_events: HashMap<EventId, Event> = HashMap::new();

    Booking {
      uid,
      ticket,
      users,
      event_creator,
      creator_events,
    }
  }

  // pub fn match_info(&mut self) -> Option<Vec<Matches>> {}
  fn get_ticket(&mut self, event_id: EventId) {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let organizer_id = env::current_account_id();
    let u_name: Vec<&str> = user.split('.').collect();
    // activate user
    match self.uid == user {
      true => {
        // Check available event and get details

        let event = self.creator_events.get(&event_id).expect("NO_EVENT_FOUND");
        let available = Status::Available;

        if let Status::Available = available {
          let available_tickets = event.mounts_tickets;
          // check event tickets availability
          if available_tickets != 0 {
            // let random_order = Rng::new(env::random_seed());
            let order_num = event.eid.to_string() + &event.mounts_tickets.to_string();
            const ONE_NEAR: u128 = u128::pow(10, 24);

            let acc_balance = env::account_balance();
            if acc_balance > 1 {
              // Create Order number of each event a random num
              self.ticket.insert(user.to_owned(), order_num);

              Promise::new(organizer_id).transfer(ONE_NEAR);
              log!(
                "{} You have successfully RSVP to {}",
                u_name[0],
                event.description
              );
            } else {
              env::log_str("You do not have sufficient funds to make this purchase");
            }
          } else {
            env::log_str("No tickets remaining ");
          }
        }

        // let book = 'booking: loop {};

        // let booked: Result<(), String> = Ok(book);
        // match booked {
        //   Ok(()) => todo!("Sent ticket details to email"),
        //   Err(_) => {
        //     env::log_str("Try Later");
        //   }
        // }
      }
      false => {
        // self.users.insert(user, User::new(user_id.to_string()));
        // continue 'booking;
      }
    }
  }

  // pub fn check_event_status(&mut self, available: i32) -> Status {
  //   // let available_std = Ticket::Standard(Some(available));
  //   let available_vip = Ticket::Vip(Some(available));

  //   if
  //   // available_std != Ticket::Standard(Some(0))
  //   // ||
  //   available_vip != Ticket::Vip(Some(0)) {
  //     log!(
  //       "There is available ticket for Standard , and for VIP {:?}",
  //       // Some(available_std),
  //       Some(available_vip)
  //     );
  //     Status::Available
  //   } else {
  //     log!("No available Tickets!");
  //     Status::Unavailable
  //   }
  // }

  pub fn view_events(&self, organizer_id: AccountId) -> Vec<&Event> {
    let mut registered_events = Vec::new();

    match self.event_creator.get(&organizer_id) {
      Some(events) => {
        for event_id in events {
          let event = self.creator_events.get(&event_id).expect("No_EVENTS_FOUND");
          match event.status {
            Status::Available => {
              let available_tickets = event.mounts_tickets;
              if available_tickets > 0 {
                log!("{} Tickets Available", available_tickets);
              }
              env::log_str("Event Available");
            }
            Status::Unavailable => env::log_str("Event Unavailable"),
          }
          registered_events.push(event);
        }
      }
      None => {}
    }
    if registered_events.is_empty() {
      registered_events.to_vec()
    } else {
      return vec![];
    }
  }

  pub fn count_events(&mut self) -> usize {
    self.creator_events.len()
  }

  pub fn check_ticket_info(&mut self, ticket_id: String) -> Vec<&Ticket> {
    let mut owned_tickets: Vec<&Ticket> = vec![];
    let tickets = &self.ticket;
    tickets.iter().for_each(|ticket| {
      // owned_tickets.push(ticket);
      match ticket.try_to_vec() {
        Ok(_) => log!("{} ticket", ticket.0),
        Err(_) => todo!(),
      }
    });

    if !owned_tickets.is_empty() {
      return owned_tickets.to_vec();
    } else {
      return vec![];
    }
    // match self.ticket.get(&ticket_id) {
    //   Some(ticket_info) => {
    //           for  ticket_id in ticket_info {

    //           }
    //   }
    //   None => todo!(),
    // }
  }

  pub fn new_event(&mut self, description: String, price: i32, venue: String, ticket_amount: i32) {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let organizer_id = env::current_account_id();
    let event_id = self.creator_events.len() as u32;
    match user == self.uid {
      true => {
        self.creator_events.insert(
          event_id.to_string(),
          Event::new(
            description,
            price,
            venue,
            ticket_amount,
            Status::Available,
            organizer_id.to_string(),
            event_id,
          ),
        );
        match self.event_creator.get(&organizer_id.to_string()) {
          Some(mut events) => {
            events.push(event_id.clone().to_string());
            self
              .event_creator
              .insert(&organizer_id.to_string(), &events.to_owned());
          }
          None => {
            let new_event = vec![event_id.to_string()];
            self
              .event_creator
              .insert(&organizer_id.to_string(), &new_event);
          }
        }
      }
      false => (),
    }
  }
}

// impl Booking {
//   fn buy_ticket(&mut self, ticket_owner: AccountId, event_id: EventId, ticket_id: TicketId) {
//     match self.ticket.get(&ticket_owner) {
//       Some(ticket_info) => {
//         ticket_info.insert(ticket_id.to_owned(), event_id.clone());
//       }
//       None => todo!(),
//     }
//   }
// }
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::testing_env;
  use near_sdk::AccountId;

  fn to_valid_account(account: &str) -> AccountId {
    AccountId::try_from(account.to_string()).expect("Invalid account")
  }

  fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.signer_account_id(predecessor);
    builder
  }

  const ONE_NEAR: u128 = u128::pow(10, 24);

  #[test]
  fn test_create_event() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment

    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Booking::new(kherld.to_string());

    contract.new_event("NEARCON 2022".to_string(), 500, "Lisbon".to_string(), 230);
    contract.new_event("NEARCON 2023".to_string(), 800, "Nairobi".to_string(), 380);

    assert_eq!(contract.count_events(), 3, "Expected 2 Events");
  }

  #[test]
  fn test_get_ticket() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Booking::new(kherld.to_string());
    // print ticket id here
    contract.get_ticket(2.to_string());
    let events = contract.view_events(kherld.to_string());
    events.to_vec();
    // assert_eq!(contract.get_ticket());
  }
}
