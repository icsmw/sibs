#[derive(Default)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn push_header<S>(&mut self, header: S)
    where
        S: Into<String>,
    {
        self.headers.push(header.into());
    }

    pub fn push_row<I, S>(&mut self, row: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(row.into_iter().map(Into::into).collect());
    }

    pub fn print(&self) {
        for line in self.render_lines() {
            println!("{line}");
        }
    }

    fn render_lines(&self) -> Vec<String> {
        let widths = self.widths();
        if widths.is_empty() {
            return Vec::new();
        }

        let mut lines = Vec::new();
        if !self.headers.is_empty() {
            lines.push(self.render_line(&self.headers, &widths));
        }
        for row in &self.rows {
            lines.push(self.render_line(row, &widths));
        }
        lines
    }

    fn widths(&self) -> Vec<usize> {
        let mut widths = Vec::new();
        let max_cols = self
            .rows
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0)
            .max(self.headers.len());
        widths.resize(max_cols, 0);

        for (idx, header) in self.headers.iter().enumerate() {
            widths[idx] = widths[idx].max(header.len());
        }
        for row in &self.rows {
            for (idx, cell) in row.iter().enumerate() {
                widths[idx] = widths[idx].max(cell.len());
            }
        }
        widths
    }

    fn render_line(&self, row: &[String], widths: &[usize]) -> String {
        let mut line = String::new();
        for (idx, width) in widths.iter().enumerate() {
            if idx > 0 {
                line.push_str(" | ");
            }
            let cell = row.get(idx).map(|s| s.as_str()).unwrap_or("");
            line.push_str(&format!("{:<width$}", cell, width = *width));
        }
        line.push_str(" |");
        line
    }
}
