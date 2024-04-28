use mysql::*;
pub fn establish_connection()-> Pool{
    let url = "mysql://root:Aa78611117@localhost:3306/pokupoku";
    Pool::new(url).expect("接続に失敗しました")
}