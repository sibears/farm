use sibears_farm::db::connection::*;
use sibears_farm::models::flag::Flag;
use sibears_farm::repos::flag::*;
use rand::Rng;

#[test]
fn test_connection() {
    let db_conn = init_db().db_conn_pool.get().unwrap();
    let flag = Flag::new(gen_flag());
    dbg!(&flag);

    let db_conn = DbConn { master: db_conn };
    let flag_repo = SqliteFlagRepo::new(&db_conn);
    flag_repo.save_new(&flag);

    let res = flag_repo.find_all();
    dbg!(&res);

    let flag = res.into_iter().find(|item| item.flag == flag.flag).unwrap();
    flag_repo.delete_by_id(flag.id.unwrap());

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