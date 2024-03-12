struct Order {
    customer: Option<Customer>,
    drink: Option<Drink>,
}

#[derive(Clone)]
struct Customer {
    name: String,
    diet: Option<Diet>,
}

#[derive(Clone, Copy)]
struct Diet {
   diet_type: Option<DietType>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DietType{Unrestricted, Vegan,}

#[derive(Clone, Copy, Debug)]
pub enum Drink{Milk, Walter, Lemonade,}

impl Order{
    //Gets diet restriction of customer for this order
    fn order_diet_restriction(&self) -> Option<DietType>{
        self.customer.as_ref()?.diet?.diet_type
    }
}

fn main(){
    let o = Order{
        customer: Some(Customer{
            name: "Jane".to_string(),
            diet: Some(Diet{
                diet_type: Some(DietType::Vegan),
            })
        }),
        drink: Some(Drink::Milk,)
    };

    assert_eq!(o.order_diet_restriction(), Some(DietType::Unrestricted));
    println!("order customer name: {}", o.customer.clone().unwrap().name);
    println!("order customer diet: {:?}", o.customer.clone().unwrap().diet.unwrap().diet_type.unwrap());

    println!("order drink: {:?}", o.drink.unwrap());
}