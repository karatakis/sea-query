use super::*;

impl IndexBuilder for SqliteQueryBuilder {
    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        if let Some(name) = &create.index.name {
            write!(sql, "{}{}{}", self.quote(), name, self.quote()).unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }

        write!(sql, " ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(_) => self.prepare_table_ref_iden(table_ref, sql),
            _ => panic!("Not supported"),
        }
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            let quote = self.quote();
            write!(sql, "{}{}{}", quote, name, quote).unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &drop.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY KEY ").unwrap();
        } else if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }

    fn write_column_index_prefix(&self, _col_prefix: &Option<u32>, _sql: &mut dyn SqlWriter) {}
}
