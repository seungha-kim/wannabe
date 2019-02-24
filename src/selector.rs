use super::parser::{Connection, ParserResult};
use std::collections::HashMap;

pub type Table = Vec<Vec<usize>>;

pub fn compute_table(input: &ParserResult) -> Table {
    let len = input.available_members.len();
    let positions: HashMap<&&str, usize> = input
        .available_members
        .iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect();
    let mut table = vec![vec![0; len]; len];
    // 일단 막 짜보자
    for conn in input.connections.iter() {
        // available_members 인덱스 찾아서
        let &pos1 = positions.get(&conn.mate1).unwrap();
        let &pos2 = positions.get(&conn.mate2).unwrap();
        table[pos1][pos2] += 1;
        table[pos2][pos1] += 1;
    }
    table
}

pub fn selector(input: &ParserResult, table: &Table) {
    /*
    *신규멤버부터 계산해야 함*
    자기 자신 제외
    숫자가 제일 작은 것을 선택하되,
    같은 숫자가 존재하면 만난지 제일 오래된 사람 우선 선택
    종료조건? 안되면 스택 pop 하고 DFS
     */
}
