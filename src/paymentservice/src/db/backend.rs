use mysql::prelude::Queryable;
use mysql::{Conn, Opts, Params, Statement, Value};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;

pub struct MySqlBackend {
    pub handle: Conn,
    // pub log: slog::Logger,
    //_schema: String,
    prep_stmts: HashMap<String, Statement>,
    db_user: String,
    db_password: String,
    db_name: String,
}

impl MySqlBackend {
    pub fn new(
        user: &str,
        password: &str,
        dbname: &str,
        // log: Option<slog::Logger>,
        prime: bool,
    ) -> Result<Self, Box<dyn Error>> {
        // let log = match log {
        //     None => slog::Logger::root(slog::Discard, o!()),
        //     Some(l) => l,
        let schema = std::fs::read_to_string("data/schema.sql")?;
        //
        // debug!(
        //     log,
        //     "Connecting to MySql DB and initializing schema {}...", dbname
        // );
        // let password = "";
        // println!("password is `{}`", password);
        let mut db = Conn::new(
            // this is the user and password from the config.toml file
            Opts::from_url(&format!("mysql://{}:{}@127.0.0.1/", user, password)).unwrap(),
        )
        .unwrap();
        assert_eq!(db.ping(), true);

        if prime {
            db.query_drop(format!("DROP DATABASE IF EXISTS {};", dbname))
                .unwrap();
            db.query_drop(format!("CREATE DATABASE {};", dbname))
                .unwrap();
            db.query_drop(format!("USE {};", dbname)).unwrap();
            for line in schema.lines() {
                if line.starts_with("--") || line.is_empty() {
                    continue;
                }
                println!("line is : {}", line);
                db.query_drop(line).unwrap();
            }
        } else {
            db.query_drop(format!("USE {};", dbname)).unwrap();
        }

        Ok(MySqlBackend {
            handle: db,
            // log: log,
            // _schema: schema.to_owned(),
            prep_stmts: HashMap::new(),
            db_user: String::from(user),
            db_password: String::from(password),
            db_name: String::from(dbname),
        })
    }

    fn reconnect(&mut self) {
        self.handle = Conn::new(
            Opts::from_url(&format!(
                "mysql://{}:{}@127.0.0.1/{}",
                self.db_user, self.db_password, self.db_name
            ))
            .unwrap(),
        )
        .unwrap();
    }

    pub fn prep_exec<P: Into<Params>>(
        &mut self,
        sql: &str,
        params: P,
    ) -> Vec<Vec<Value>> {
        if !self.prep_stmts.contains_key(sql) {
            let stmt = self
                .handle
                .prep(sql)
                .expect(&format!("failed to prepare statement \'{}\'", sql));
            self.prep_stmts.insert(sql.to_owned(), stmt);
        }

        let params: Params = params.into();
        loop {
            match self.handle.exec_iter(
                self.prep_stmts[sql].clone(),
                params.clone(),
            ) {
                Err(e) => {
                    eprintln!("query \'{}\' failed ({}), reconnecting to database", sql, e);
                }
                Ok(res) => {
                    let mut rows = vec![];
                    for row in res {
                        rows.push(row.unwrap().unwrap());
                    }
                    //debug!(self.log, "executed query {}, got {} rows", sql, rows.len());
                    return rows;
                }
            }
            self.reconnect();
        }
    }

    fn do_insert<P: Into<Params>>(
        &mut self,
        table: &str,
        vals: P,
        replace: bool,
    ) -> () {
        let vals: Params = vals.into();
        let mut param_count = 0;
        if let Params::Positional(vec) = &vals {
            param_count = vec.len();
        }

        let op = if replace { "REPLACE" } else { "INSERT" };
        let q = format!(
            "{} INTO {} VALUES ({})",
            op,
            table,
            (0..param_count)
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(",")
        );
        loop {
            if let Err(e) = self
                .handle
                .exec_drop(q.clone(), vals.clone())
            {
                eprintln!(
                    "failed to insert into {}, query {} ({}), reconnecting to database",
                    table, q, e
                );
            } else {
                break;
            }
            self.reconnect();
        }
    }

    pub fn insert<P: Into<Params>>(
        &mut self,
        table: &str,
        vals: P,
    ) -> () {
        self.do_insert(table, vals, false)
    }
}
