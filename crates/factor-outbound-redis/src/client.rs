use redis::{ConnectionLike, RedisError};
use redis_test::{MockCmd, MockRedisConnection};

fn my_exists<C: ConnectionLike>(conn: &mut C, key: &str) -> Result<bool, RedisError> {
    let exists: bool = redis::cmd("EXISTS").arg(key).query(conn)?;
    Ok(exists)
}

let mut mock_connection = MockRedisConnection::new(vec![
    MockCmd::new(redis::cmd("EXISTS").arg("foo"), Ok("1")),
]);

let result = my_exists(&mut mock_connection, "foo").unwrap();
assert_eq!(result, true);