use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::bs58::encode::Error;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, Promise};
use std::collections::HashMap;

pub type AccountId = String;
pub type TicketId = String;
pub type EventId = u32;

// Event creator && Ticket booking app for the Events

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  pub user_id: String,
}

impl User {
  fn new(user_id: String) -> Self {
    User { user_id }
  }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket {
  ticket_id: TicketId,
  ticket_owner: String,
  ticket_description: String,
  // Standard(Option<i32>),
  // Vip(Option<i32>),
}

#[derive(
  BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq, Clone, Copy,
)]
#[serde(crate = "near_sdk::serde")]
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

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Booking {
  // status: Status,
  uid: AccountId,
  ticket: HashMap<String, TicketId>,
  users: HashMap<String, User>,
  event_creator: Vec<Event>,
}

impl Booking {
  // #[init]
  pub fn new(uid: AccountId) -> Self {
    let users: HashMap<String, User> = HashMap::new();
    let ticket: HashMap<String, TicketId> = HashMap::new();
    let event_creator: Vec<Event> = Vec::new();

    Booking {
      uid,
      ticket,
      users,
      event_creator,
    }
  }

  // pub fn match_info(&mut self) -> Option<Vec<Matches>> {}
  fn get_ticket(&mut self, event_id: EventId) {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let ticket_id = self.event_creator.len() as u32;
    let u_name: Vec<&str> = user.split('.').collect();
    // activate user
    match self.uid == user {
      true => {
        // Check available event and get details

        let events = &mut self.event_creator;
        let book = 'booking: loop {
          events.iter().for_each(|event| {
            if event.eid == event_id {
              // check event tickets availability
              let available = Status::Available;
              if let Status::Available = available {
                let available_tickets = event.mounts_tickets;
                if available_tickets != 0 {
                  const ONE_NEAR: u128 = u128::pow(10, 24);
                  let acc_balance = env::account_balance();
                  if acc_balance > 1 {
                    let organizer_id = env::current_account_id();
                    self
                      .ticket
                      .insert(user.to_owned(), available_tickets.to_string());
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
              } else {
                env::log_str("Event Closed ");
              }
            }
          })
        };
        
        let booked: Result<(), String> = Ok(book);
        match booked {
          Ok(()) => todo!("Sent ticket details"),
          Err(_) => {
            env::log_str("Try Later");
          }
        }
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
    let events = &self.event_creator;
    let mut registered_events = Vec::new();

    match self.uid == organizer_id {
      true => {
        events.iter().for_each(|event| {
          if event.event_organizer == organizer_id {
            registered_events.push(event);
            match event.status {
              Status::Available => {
                let available_tickets = event.mounts_tickets;
                if available_tickets > 0 {
                  log!("Tickets {}", available_tickets);
                }
                env::log_str("Event Available");
              }
              Status::Unavailable => env::log_str("Event Unavailable"),
            }
          }
        });
      }
      false => (),
    }
    if !registered_events.is_empty() {
      registered_events.to_vec()
    } else {
      return vec![];
    }
  }

  pub fn check_ticket_info(&mut self) {}

  pub fn new_event(
    &mut self,
    description: String,
    price: i32,
    venue: String,
    ticket_amount: i32,
  ) -> Result<Promise, String> {
    const ONE_NEAR: u128 = u128::pow(10, 24);
    let acc_balance = env::account_balance();

    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let organizer_id = env::current_account_id();
    let event_id = self.event_creator.len() as u32;
    match user == self.uid {
      true => {
        let amount = acc_balance / ONE_NEAR;
        if amount > 1 {
          self.event_creator.push(Event::new(
            description,
            price,
            venue,
            ticket_amount,
            Status::Available,
            organizer_id.to_owned().to_string(),
            event_id,
          ));

          Ok(Promise::new(organizer_id).transfer(ONE_NEAR))
        } else {
          let err = format!("Account insufficient balance");
          Err(err)
        }
      }
      false => {
        let err = format!("Account does not exists");
        Err(err)
      }
    }
  }
}

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
  fn booking() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
  }

  #[test]
  fn check_status() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
  }
}
