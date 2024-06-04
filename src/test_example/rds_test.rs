use plier::rds;

use fred::prelude::*;
use serde::{Deserialize, Serialize};
use fred::clients::Transaction;

#[cfg(test)]
mod files_test {

    use super::*;
    use tokio::test;

    #[test]
    async fn t1() {
        let url = "redis://rust1:rust1_abc1234@34.118.182.28:26379/0";
        let size: usize = 2;

        let res_rds = rds::init_rds(url, size).await;
        println!("{:?}", res_rds);

        let rds = res_rds.unwrap();
        let res_set = rds.set("k1", "heihei").await;
        println!("{:?}", res_set);

        let res_set = rds.set_expire("k2", "xixi", 100).await;
        println!("{:?}", res_set);

        let res_get = rds.get_string("k1").await;
        println!("{:?}", res_get);

        let res_set = rds.set("k3", 1).await;
        println!("{:?}", res_set);

        let res_incr = rds.incr("k3").await;
        println!("{:?}", res_incr);

        let res_get = rds.get_i64("k3").await;
        println!("{:?}", res_get);

        let r = test_tx(rds.get_tx().await).await;
        println!("{:?}", r);

        let person = Person {
            user_name: "MZR".into(),
            name: "木真人".into(),
            age: 100,
            phones: vec!["abc".into(), "123".into()],
        };
        let serialized = serde_json::to_string(&person).unwrap();

        let r2 = rds.set("test_json1", serialized).await;
        println!("{:?}", r2);

        let person_json = rds.get_string("test_json1").await.unwrap();
        println!("{:?}", person_json);

        let deserialized: Person = serde_json::from_str(&person_json).unwrap();
        println!("{:?}", deserialized);

        let d = rds.del("k1").await;
        println!("{:?}", d);
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Person {
        #[serde(rename = "userName")]
        user_name: String,
        name: String,
        age: u8,
        phones: Vec<String>,
    }


    async fn test_tx(trx: Transaction) -> Result<(), RedisError> {
        println!("{:?}", trx);
        let result: RedisValue = trx.get("foo11").await?;
        assert!(result.is_queued());
        let result: RedisValue = trx.set("foo11", "bar11", None, None, false).await?;
        assert!(result.is_queued());
        let result: RedisValue = trx.get("foo11").await?;
        assert!(result.is_queued());

        // automatically send `WATCH ...` before `MULTI`
        trx.watch_before(vec!["foo11", "bar11"]);
        let values: (Option<String>, (), String) = trx.exec(true).await?;
        println!("Transaction results: {:?}", values);

        Ok(())
    }
}