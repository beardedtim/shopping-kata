use std::collections::HashMap;
#[allow(dead_code)]
/// An Item represents something that we hold in our inventory
pub struct Item {
    /// an Item as a unique ID
    id: String,
    /// an Item as a price that is can be sold at
    price: f32,
    /// an Item has a title
    title: String,
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum USState {
    WA,
    OR,
    CA,
    NY,
    TN,
    FL,
}
#[derive(Hash, Eq, PartialEq, Debug)]

/// A User is the person that is buying our wares
pub struct User {
    /// a User has a unique ID
    id: String,
    /// a User is inside of a State. This allows you to have
    /// different tax rates per User
    state: USState,
}
#[allow(dead_code)]
/// An Order is a grouping of Items together
pub struct Order {
    /// an Order has a unique ID
    id: String,
    /// an Order has a grouping of Items that are sold with this Order
    items: Vec<Item>,
    user_id: String,
}

pub fn calculate_order(_order: Order, _user: User, _taxes: HashMap<USState, f32>) -> f32 {
    return 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn calculates_single_order_correctly() {
        let mut current_tax_levels: HashMap<USState, f32> = HashMap::new();
        current_tax_levels.insert(USState::TN, 9.75);
        current_tax_levels.insert(USState::WA, 7.75);
        current_tax_levels.insert(USState::OR, 8.15);
        current_tax_levels.insert(USState::FL, 11.40);
        current_tax_levels.insert(USState::NY, 15.20);

        let user = User {
            id: String::from("123"),
            state: USState::TN,
        };

        let item = Item {
            id: String::from("item_foo"),
            price: 12.34,
            title: String::from("Some item"),
        };

        let order = Order {
            id: String::from("456"),
            items: vec![item],
            user_id: user.id.clone(),
        };

        assert_eq!(calculate_order(order, user, current_tax_levels), 13.54)
    }
}
