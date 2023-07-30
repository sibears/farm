use sibears_farm::db::connection::*;
use sibears_farm::models::flag::Flag;
use sibears_farm::repos::flags_repo::*;
use rand::Rng;

#[test]
fn test_connection() {
    let db_conn = init_db().db_conn_pool.get().unwrap();
    let flag = Flag::new(gen_flag());
    dbg!(&flag);
    let db_conn = DbConn { master: db_conn };
    let flag_repo = SqliteFlagRepo::new(&db_conn);
    let res = flag_repo.save_new(&flag);
    dbg!(&res);
    let res = flag_repo.find_all();
    dbg!(&res);
}

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    std::iter::repeat_with(one_char).take(len).collect()
}

// Генерация случайного флага
fn gen_flag() -> String {
    generate(31) + "="
}