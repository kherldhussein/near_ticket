use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, Promise};
use std::collections::HashMap;

pub type AccountId = String;
pub type TicketId = String;
pub type EventId = String;
pub type OrderNumber = String;

// Event creator && Ticket booking app for the Events

#[derive(Clone, BorshDeserialize, BorshSerialize, Debug)]
pub struct User {
  pub user_id: String,
}

impl User {
  fn new(user_id: String) -> Self {
    User { user_id }
  }
}

#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
  description: String,
  price: i32,
  venue: String,
  status: Status,
  mounts_tickets: i32,
  event_organizer: AccountId,
  eid: u32,
}

#[near_bindgen]
impl Event {
  fn new(
    description: String,
    price: i32,
    venue: String,
    mounts_tickets: i32,
    status: Status,
    event_organizer: AccountId,
    eid: u32,
  ) -> Self {
    Event {
      description,
      price,
      venue,
      mounts_tickets,
      status,
      event_organizer,
      eid,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone)]
pub struct Ticket {
  ticket_id: TicketId,
  event_id: EventId,
  ticket_owner: String,
  ticket_description: String,
  // Standard(Option<i32>),
  // Vip(Option<i32>),
}
impl Ticket {
  fn new(
    ticket_id: TicketId,
    event_id: EventId,
    ticket_owner: String,
    ticket_description: String,
  ) -> Self {
    Ticket {
      ticket_id,
      event_id,
      ticket_owner,
      ticket_description,
    }
  }
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone, Copy, Serialize)]
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Default)]
pub struct Contract {
  uid: AccountId,
  ticket: HashMap<OrderNumber, Ticket>,
  users: HashMap<String, User>,
  event_creator: Vec<Event>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(uid: AccountId) -> Self {
    let users: HashMap<String, User> = HashMap::new();
    let ticket: HashMap<OrderNumber, Ticket> = HashMap::new();
    let event_creator: Vec<Event> = Vec::new();

    Contract {
      uid,
      ticket,
      users,
      event_creator,
    }
  }

  // Create Events
  pub fn new_event(&mut self, description: String, price: i32, venue: String, ticket_amount: i32) {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let organizer_id = env::current_account_id();
    let event_id = self.event_creator.len() as u32;
    let available_ticket = Some(ticket_amount);

    match user == self.uid {
      true => {
        if available_ticket.unwrap() > 0 {
          let status = Status::Available;
          self.event_creator.push(Event::new(
            description,
            price,
            venue,
            ticket_amount,
            status,
            organizer_id.to_string(),
            event_id,
          ));
        }
      }
      false => (),
    }
  }

  // View events
  pub fn view_events(&self) -> Vec<Event> {
    let events = &self.event_creator;
    events.to_vec()
  }

  // Get events ticket
  fn get_ticket(&mut self, event_id: EventId) {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);

    let u_name: Vec<&str> = user.split('.').collect();
    let ticket_id = self.ticket.len() as u32;
    // activate user
    match self.uid == user {
      true => {
        // Check available event and get details

        let events = &mut self.event_creator;
        let ticket = &mut self.ticket;

        let available = Status::Available;

        if let Status::Available = available {
          events.iter().for_each(|event| {
            if event.eid.to_string() == event_id {
              let order_num = event.eid.to_string() + &event.mounts_tickets.to_string();
              const ONE_NEAR: u128 = u128::pow(10, 24);
              let acc_balance = env::account_balance();
              let organizer_id = env::current_account_id();
              if acc_balance > 1 {
                ticket.insert(
                  order_num,
                  Ticket::new(
                    ticket_id.to_string(),
                    event_id.to_owned(),
                    user.to_owned(),
                    event.description.to_owned(),
                  ),
                );

                Promise::new(organizer_id).transfer(ONE_NEAR);
                log!(
                  "{} You have successfully RSVP to {}\n Your ticket info has been sent to {}@near.io",
                  u_name[0],
                  event.description,
                  u_name[0],
                );
              } else {
                env::log_str("You do not have sufficient funds to make this purchase");
              }
            }
          });
          for ticket_info in ticket {
            log!("Your Order Number is {:?}", ticket_info.0);
          }
        }
      }
      false => {}
    }
  }

  pub fn count_events(&mut self) -> usize {
    self.event_creator.len()
  }

  pub fn count_tickets(&mut self) -> usize {
    self.ticket.len()
  }

  // View ticket
  pub fn check_ticket_info(&mut self, order_number: String) {
    let account_id = env::signer_account_id();
    let tickets = &self.ticket;
    let user = String::from(account_id);
    match self.uid == user {
      true => {
        for ticket_info in tickets {
          if ticket_info.0 == &order_number {
            log!("Your ticket info {:?}", ticket_info.1);
          }
        }
      }
      false => {}
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

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .account_balance(100 * ONE_NEAR)
      .signer_account_id(predecessor);
    builder
  }

  #[test]
  fn test_create_event() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment

    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Contract::new(kherld.to_string());

    contract.new_event("NEARCON 2022".to_string(), 500, "Lisbon".to_string(), 230);
    contract.new_event("NEARCON 2023".to_string(), 800, "Nairobi".to_string(), 380);

    assert_eq!(contract.count_events(), 2, "Expected 2 Events");
  }

  #[test]
  fn test_create_events() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Contract::new(kherld.to_string());
    contract.new_event(
      "NEARCON 2023".to_string(),
      800,
      "In-Person".to_string(),
      3000,
    );

    let events = contract.view_events();

    let status = Status::Available;
    assert_eq!(
      events[0].status, status,
      "For event to be available, amount of ticket must be > 0"
    );
  }

  #[test]
  fn test_get_ticket() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Contract::new(kherld.to_string());
    contract.new_event("NEARCON 2023".to_string(), 800, "In-Person".to_string(), 1);
    contract.get_ticket(0.to_string());
    // let ticket = contract.check_ticket_info();
    let events = contract.view_events();
    let status = Status::Available;
    assert_eq!(
      events[0].status, status,
      "For event to be available, amount of ticket must be > 0"
    );
  }

  #[test]
  fn test_check_ticket_info() {
    let kherld = AccountId::new_unchecked("kherld.testnet".to_string());
    // set up the mock context into the testing environment
    let context = get_context(to_valid_account("kherld.testnet"));

    testing_env!(context.build());
    let mut contract = Contract::new(kherld.to_string());
    contract.new_event("NEARCON 2023".to_string(), 800, "In-Person".to_string(), 66);
    contract.get_ticket(0.to_string());
    let ticket = contract.check_ticket_info("066".to_string());
    assert_eq!((), ticket);
  }
}
