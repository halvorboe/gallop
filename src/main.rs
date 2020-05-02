use gallop::table;

fn main() {
    let table = table::Table::new();
    let row = table::Row::new();
    table.insert(row);
    table.select();
    table.count();
}
