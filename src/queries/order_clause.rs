#[derive(Debug, PartialEq)]
pub struct OrderClause {
    pub column: String,
    pub direccion: OrderDirection
}

#[derive(Debug, PartialEq)]
pub enum OrderDirection {
    Asc,
    Desc
}