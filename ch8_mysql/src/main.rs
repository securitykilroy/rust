use mysql::*;

fn main() {
    
    let connstr = "mysql://kilroy:Passw0rd!@192.168.4.127:3306/stratapp";
    let sqlpool = Pool::new(connstr).unwrap();

    let connection = sqlpool.get_conn().unwrap();

    println!("{}", connection.connection_id());

}
