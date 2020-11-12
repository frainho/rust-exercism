pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.char_indices()
                .map(|(col_index, cell_state)| match cell_state {
                    '*' => '*',
                    _ => count_mines(row_index, col_index, minefield),
                })
                .collect()
        })
        .collect()
}

fn count_mines(row_index: usize, col_index: usize, minefield: &[&str]) -> char {
    let previous_row_index = row_index.saturating_sub(1);
    let next_row = (row_index + 1).min(minefield.len() - 1);

    let previous_col = col_index.saturating_sub(1);
    let next_col = (col_index + 1).min(minefield[row_index].len() - 1);

    let mut counter = 0;

    for row in &minefield[previous_row_index..=next_row] {
        for col in previous_col..=next_col {
            if row.as_bytes()[col] == b'*' {
                counter += 1;
            }
        }
    }
    match counter {
        0 => ' ',
        count => std::char::from_digit(count, 10).unwrap(),
    }
}
