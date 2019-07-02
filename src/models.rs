#[derive(Queryable, Debug)]
pub struct Cat {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Associations, Debug)]
#[belongs_to(Cat)]
pub struct Bird {
    pub id: i32,
    pub cat_id: i32,
    pub species: String,
    pub colors: String
}
