use std::collections::HashSet;

use redis::{Client, Commands, Connection, RedisError, RedisResult};

pub struct DatabaseActions {
  client: Option<Client>,
  connection: Option<Connection>,
}

pub type DatabaseError = RedisError;
pub type DatabaseResult<T> = RedisResult<T>;

enum ConstantKey {
  CurrentDigitCount,
}

impl DatabaseActions {
  pub fn new(hostname: &str, port: i32) -> DatabaseResult<Self> {
    let client = redis::Client::open(format!("redis://{}:{}/", hostname, port))?;
    Ok(Self {
      client: Some(client),
      connection: None,
    })
  }

  pub fn connect(&self) -> DatabaseResult<Self> {
    match &self.client {
      None => panic!("Tried to connect with no client!"),
      Some(client) => {
        let connection = client.get_connection()?;
        Ok(Self {
          client: None,
          connection: Some(connection),
        })
      }
    }
  }

  pub fn get_url_cardinality(&mut self) -> DatabaseResult<u32> {
    let s = self.get_constant_key(ConstantKey::CurrentDigitCount);
    match &mut self.connection {
      None => panic!("Tried to connect without a connection to the database!"),
      Some(connection) => {
        connection.get(s)
      }
    }
  }

  pub fn get_range(&mut self, key: u32, start: i64, end: i64) -> DatabaseResult<HashSet<String>> {
    match &mut self.connection {
      None => panic!("Tried to connect without a connection to the database!"),
      Some(connection) => {
        connection.zrangebyscore(key, start, end)
      }
    }
  }

  fn get_constant_key(&self, variant: ConstantKey) -> String {
    match variant {
      ConstantKey::CurrentDigitCount => String::from("CURRENT_DIGIT_COUNT"),
    }
  }
}
