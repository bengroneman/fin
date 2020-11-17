extern crate clap;
use clap::{Arg, App, SubCommand};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Transaction {
    id: i32,
    title: String,
    amount: i32,
}

fn main() -> Result<()> {
    // Init DB
    let conn = Connection::open_in_memory()?;
    conn.execute(
       "CREATE TABLE transactions (
                  id              INTEGER PRIMARY KEY,
                  title           TEXT,
                  amount          INTEGER
                  )",
        params![],
    )?;

    // TODO: abstract this into a yml config file    
    let matches = App::new("Fin Calc")
                          .version("0.1")
                          .author("Ben G. <hello@bluedojo.dev>")
                          .about("A simple and fun way to budget.")
                          //.arg(Arg::with_name("config")
                               //.short("c")
                               //.long("config")
                               //.value_name("FILE")
                               //.help("Sets a custom config file")
                               //.takes_value(true))
                          //.arg(Arg::with_name("INPUT")
                               //.help("Sets the input file to use")
                               //.required(true)
                               //.index(1))
                          //.arg(Arg::with_name("v")
                               //.short("v")
                               //.multiple(true)
                               //.help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("add")
                                      .about("Add a transaction")
                                      .version("0.1")
                                      .author("Ben G. <hello@bluedojo.dev>")
                                      .arg(Arg::with_name("title")
                                          .short("t")
                                          .long("title")
                                          .takes_value(true)
                                          .help("How would you describe the transaction?")))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    //let config = matches.value_of("config").unwrap_or("default.conf");
    //println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    //println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    //match matches.occurrences_of("v") {
        //0 => println!("No verbose info"),
        //1 => println!("Some verbose info"),
        //2 => println!("Tons of verbose info"),
        //3 | _ => println!("Don't be crazy"),
    //}

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("add") {
        let test = Transaction {
            id: 0,
            title: "test".to_string(),
            amount: 100,
        };
        conn.execute(
            "INSERT INTO transactions (title, amount) VALUES (?1, ?2)",
            params![test.title, test.amount],
        )?;
    }

    let mut stmt = conn.prepare("SELECT id, title, amount FROM transactions")?;
    let transactions_iter = stmt.query_map(params![], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            title: row.get(1)?,
            amount: row.get(2)?,
        })
    })?;

    for transaction in transactions_iter {
        println!("Found person {:?}", transaction.unwrap());
    }
    Ok(())

}
