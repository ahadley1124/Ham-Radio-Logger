//use crate::database::init_db;
//use sqlite::Connection;
//
//pub fn insert_callsign(conn: &Connection, callsign: &str) -> &Connection {
//    let mut stmt = conn.prepare("INSERT INTO callsigns (callsign) VALUES (?1)")?;
//    stmt.execute(params![callsign])?;
//    conn
//}
//
//pub fn insert_qsl(conn: &Connection, callsign: &str, qsl_date: &str, qsl_sent: &str, qsl_rcvd: &str, qsl_freq: &str) -> &Connection {
//    let mut stmt = conn.prepare("INSERT INTO qsl (callsign_id, qsl_date, qsl_sent, qsl_rcvd, qsl_freq) VALUES (?1, ?2, ?3, ?4, ?5)")?;
//    stmt.execute(params![callsign, qsl_date, qsl_sent, qsl_rcvd, qsl_freq])?;
//    conn
//}
//
//pub fn insert_name(conn: &Connection, callsign: &str, name: &str) -> &Connection {
//    let mut stmt = conn.prepare("INSERT INTO name (callsign_id, name) VALUES (?1, ?2)")?;
//    stmt.execute(params![callsign, name])?;
//    conn
//}