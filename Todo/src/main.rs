use aws_sdk_rds::config::Region;
use aws_sdk_rds::{Client as RdsClient, Config};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::process;

fn startup() {
    println!(
        r#"
           **********   *******   *******     *******    |
          /////**///   **/////** /**////**   **/////**   |   TODO APP
              /**     **     //**/**    /** **     //**  |
              /**    /**      /**/**    /**/**      /**  |   Version 1.0
              /**    /**      /**/**    /**/**      /**  |
              /**    //**     ** /**    ** //**     **   |   Help or ? for help
              /**     //*******  /*******   //*******    |
              //       ///////   ///////     ///////     |
        "#
    );
}

fn connect_to_database() -> MysqlConnection {
    // Hardcoded database URL
    let database_url = "mysql://jon429r:Ds0OyRFTBgOHC5dWdmwX@todo-database.ciitmbvlwert.us-east-2.rds.amazonaws.com:3306/todo-database";

    match MysqlConnection::establish(&database_url) {
        Ok(connection) => {
            println!("Database connection established.");
            connection
        }
        Err(err) => {
            eprintln!("Error connecting to database: {}", err);
            process::exit(1);
        }
    }
}

fn list_rds_instances() -> Result<(), Box<dyn std::error::Error>> {
    // Hardcoded AWS region
    let region = "us-east-2";
    let config = Config::builder().region(Region::new(region)).build();

    let client = RdsClient::from_conf(config);

    let resp = client.describe_db_instances().send().await?;

    for instance in resp.db_instances.unwrap_or_default() {
        println!(
            "DB Instance ID: {}",
            instance.db_instance_identifier.unwrap_or_default()
        );
    }

    Ok(())
}

#[tokio::main]
fn main() {
    startup();

    println!("Welcome to Your Project Manager\n");

    println!("Connecting to database...");
    let _connection = connect_to_database();

    println!("Listing RDS instances...");
    if let Err(err) = list_rds_instances().await {
        eprintln!("Error listing RDS instances: {}", err);
        process::exit(1);
    }

    // Other operations
}
