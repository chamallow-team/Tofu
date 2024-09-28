use sqlx::{MySql, Pool};

pub type Database = Pool<MySql>;