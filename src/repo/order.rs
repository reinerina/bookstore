use crate::entity::{Book, Order, OrderItem, OrderPaymentStatus, OrderShippingStatus};
use crate::repo::{BookRepo, StockRepo, UserRepo};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct OrderRepo;

impl OrderRepo {
    pub async fn create_order(
        conn: &mut Conn,
        username: &str,
        books: &Vec<(u32, u32)>,
    ) -> anyhow::Result<Option<u32>> {
        let order_id = match UserRepo::get_user_detail(&mut *conn, username).await? {
            None => anyhow::bail!("user {} not found", username),
            Some(user) => {
                let query = r"INSERT INTO orders (customer_id,order_date,shipping_address) VALUES(:customer_id,NOW(),:shipping_address);";
                let params = params! {
                    "customer_id" => user.id,
                    "shipping_address" => user.address,
                };
                query.with(params).run(&mut *conn).await?;
                let query = r"SELECT LAST_INSERT_ID() AS order_id;";
                let order_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
                match order_id {
                    None => anyhow::bail!("failed to create order"),
                    Some(order_id) => {
                        let query = r"INSERT INTO order_items (order_id,book_id,quantity) VALUES(:order_id,:book_id,:quantity);";
                        for (book_id, quantity) in books {
                            let params = params! {
                                "order_id" => order_id,
                                "book_id" => book_id,
                                "quantity" => quantity,
                            };
                            query.with(params).run(&mut *conn).await?;
                        }

                        Ok(Some(order_id))
                    }
                }
            }
        };
        order_id
    }

    pub async fn get_order_list(conn: &mut Conn, username: &str) -> anyhow::Result<Vec<Order>> {
        let query = r"SELECT
	orders.order_id,
	orders.customer_id,
	orders.order_date,
	orders.shipping_address,
	orders.payment_status,
	orders.shipping_status,
	credit_rules.discount_percentage,
	SUM( books.price * order_items.quantity ) AS original_price,
	SUM( books.price * order_items.quantity ) * credit_rules.discount_percentage * 0.01 AS discount_amount,
	SUM( books.price * order_items.quantity ) * ( 100.0 - credit_rules.discount_percentage ) * 0.01 AS total_price
FROM
	orders
	LEFT JOIN customers ON customers.customer_id = orders.customer_id
	LEFT JOIN credit_rules ON credit_rules.credit_level = customers.credit_level
	LEFT JOIN order_items ON order_items.order_id = orders.order_id
	LEFT JOIN books ON books.book_id = order_items.book_id
WHERE
	orders.customer_id = :customer_id
GROUP BY
	orders.order_id,
	orders.customer_id
ORDER BY
	orders.order_id ASC;";
        let customer_id = match UserRepo::get_user_id(&mut *conn, username).await? {
            None => anyhow::bail!("user {} not found", username),
            Some(user_id) => user_id,
        };

        let params = params! {
            "customer_id" => customer_id,
        };

        let result = query
            .with(params)
            .map(
                conn,
                |(
                    order_id,
                    customer_id,
                    order_date,
                    shipping_address,
                    payment_status,
                    shipping_status,
                    discount_percentage,
                    original_price,
                    discount_amount,
                    total_price,
                )| Order {
                    id: order_id,
                    customer_id,
                    items: Vec::new(),
                    date: order_date,
                    discount_percentage,
                    discount_amount,
                    original_amount: original_price,
                    total_amount: total_price,
                    shipping_address,
                    payment_status: {
                        let payment_status: String = payment_status;
                        payment_status.parse().unwrap()
                    },
                    shipping_status: {
                        let shipping_status: String = shipping_status;
                        shipping_status.parse().unwrap()
                    },
                },
            )
            .await?;

        Ok(result)
    }

    pub async fn get_order_detail(
        conn: &mut Conn,
        username: &str,
        order_id: u32,
    ) -> anyhow::Result<(Option<Order>, Vec<Book>)> {
        let query = r"SELECT
	orders.order_id,
	orders.customer_id,
	GROUP_CONCAT( DISTINCT CONCAT( order_items.order_item_id, ',', order_items.book_id, ',', order_items.quantity , ',', books.price * order_items.quantity ) SEPARATOR ';' ) AS items,
	orders.order_date,
	orders.shipping_address,
	orders.payment_status,
	orders.shipping_status,
	credit_rules.discount_percentage,
	SUM( books.price * order_items.quantity ) AS original_price,
	SUM( books.price * order_items.quantity ) * credit_rules.discount_percentage * 0.01 AS discount_amount,
	SUM( books.price * order_items.quantity ) * ( 100 - credit_rules.discount_percentage ) * 0.01 AS total_price
FROM
	orders
	LEFT JOIN customers ON customers.customer_id = orders.customer_id
	LEFT JOIN credit_rules ON credit_rules.credit_level = customers.credit_level
	LEFT JOIN order_items ON order_items.order_id = orders.order_id
	LEFT JOIN books ON books.book_id = order_items.book_id
WHERE
	orders.order_id = :order_id
	AND orders.customer_id = :customer_id
GROUP BY
	orders.order_id,
	orders.customer_id;";
        let customer_id = match UserRepo::get_user_id(&mut *conn, username).await? {
            None => anyhow::bail!("user {} not found", username),
            Some(user_id) => user_id,
        };

        let params = params! {
            "order_id" => order_id,
            "customer_id" => customer_id,
        };

        let mut result = query
            .with(params)
            .map(
                &mut *conn,
                |(
                    order_id,
                    customer_id,
                    items,
                    order_date,
                    shipping_address,
                    payment_status,
                    shipping_status,
                    discount_percentage,
                    original_price,
                    discount_amount,
                    total_price,
                )| Order {
                    id: order_id,
                    customer_id,
                    items: {
                        let items: Option<String> = items;
                        match items {
                            Some(items) => items
                                .split(';')
                                .map(|item| {
                                    let mut iter = item.split(',');
                                    OrderItem {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        order_id,
                                        book_id: iter.next().unwrap().parse().unwrap(),
                                        quantity: iter.next().unwrap().parse().unwrap(),
                                        total_price: iter.next().unwrap().parse().unwrap(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    date: order_date,
                    discount_percentage,
                    discount_amount,
                    original_amount: original_price,
                    total_amount: total_price,
                    shipping_address,
                    payment_status: {
                        let payment_status: String = payment_status;
                        payment_status.parse().unwrap()
                    },
                    shipping_status: {
                        let shipping_status: String = shipping_status;
                        shipping_status.parse().unwrap()
                    },
                },
            )
            .await?;

        let mut books = Vec::new();
        match result.pop() {
            Some(order) => {
                for item in order.items.iter() {
                    match BookRepo::get_book_detail(&mut *conn, item.book_id).await? {
                        Some(book) => books.push(book),
                        None => anyhow::bail!("book {} not found", item.book_id),
                    }
                }
                match books.len() == order.items.len() {
                    true => Ok((Some(order), books)),
                    false => anyhow::bail!("failed to get order detail"),
                }
            }
            None => anyhow::bail!("order {} not found", order_id),
        }
    }

    pub async fn get_order_items(conn: &mut Conn, order_id: u32) -> anyhow::Result<Vec<OrderItem>> {
        let query = r"SELECT order_items.order_item_id,order_items.book_id,order_items.quantity,
books.price * order_items.quantity AS total_price FROM order_items
LEFT JOIN books ON books.book_id = order_items.book_id
WHERE order_items.order_id = :order_id;";
        let params = params! {
            "order_id" => order_id,
        };

        let result = query
            .with(params)
            .map(conn, |(order_item_id, book_id, quantity, total_price)| {
                OrderItem {
                    id: order_item_id,
                    order_id,
                    book_id,
                    quantity,
                    total_price,
                }
            })
            .await?;

        Ok(result)
    }

    pub async fn update_order_payment_status(
        conn: &mut Conn,
        order_id: u32,
        status: OrderPaymentStatus,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE orders SET payment_status = :status WHERE order_id = :order_id;";
        let params = params! {
            "status" => status.to_string(),
            "order_id" => order_id,
        };

        query.with(params).run(&mut *conn).await?;

        match status {
            OrderPaymentStatus::Paid => {
                let query = r"UPDATE customers
SET total_purchase = total_purchase + ( SELECT SUM( books.price * order_items.quantity ) FROM order_items LEFT JOIN books ON books.book_id = order_items.book_id WHERE order_items.order_id = :order_id )
WHERE
	customers.customer_id = ( SELECT customer_id FROM orders WHERE order_id = :order_id );";
                let params = params! {
                    "order_id" => order_id,
                };
                query.with(params).run(&mut *conn).await?;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn update_order_shipping_status(
        conn: &mut Conn,
        order_id: u32,
        status: OrderShippingStatus,
    ) -> anyhow::Result<()> {
        let query = r"UPDATE orders SET shipping_status = :status WHERE order_id = :order_id;";
        let params = params! {
            "status" => status.to_string(),
            "order_id" => order_id,
        };

        query.with(params).run(&mut *conn).await?;

        Ok(())
    }

    pub async fn ship_order(
        conn: &mut Conn,
        order_id: u32,
        stock_locations: &Vec<(u32, u32, u32)>,
    ) -> anyhow::Result<()> {
        for (book_id, location_id, quantity) in stock_locations {
            StockRepo::out_stock(&mut *conn, *book_id, *location_id, *quantity).await?;
        }
        let query = r"UPDATE orders SET shipping_status = 'shipped' WHERE order_id = :order_id;";
        let params = params! {
            "order_id" => order_id,
        };
        query.with(params).run(&mut *conn).await?;
        Ok(())
    }
}
