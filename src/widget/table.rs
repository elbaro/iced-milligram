pub fn text_table<
    'a,
    Message: 'a,
    HeaderString: AsRef<str>,
    HeaderRow: std::iter::IntoIterator<Item = HeaderString>,
    S: AsRef<str>,
    Row: std::iter::IntoIterator<Item = S>,
    Matrix: std::iter::IntoIterator<Item = Row>,
>(
    headers: HeaderRow,
    column_sizes: &[u16],
    rows: Matrix,
) -> iced::Column<'a, Message> {
    let mut col = iced::Column::new();

    {
        let mut header_row = iced::Row::new().padding(12).spacing(15);
        for (&width, item) in column_sizes.iter().zip(headers.into_iter()) {
            header_row = header_row
                .push(crate::widget::bold_text(item.as_ref()).width(iced::Length::Units(width)));
        }
        col = col.push(header_row);
    }

    for row in rows.into_iter() {
        let mut r = iced::Row::new().padding(12).spacing(15);
        for (&width, item) in column_sizes.iter().zip(row.into_iter()) {
            r = r.push(crate::widget::text(item.as_ref()).width(iced::Length::Units(width)));
        }
        col = col.push(r);
    }

    col
}
