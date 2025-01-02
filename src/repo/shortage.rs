use crate::entity::{Shortage, ShortageItem};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};

pub struct ShortageRepo;

impl ShortageRepo {
    pub async fn create_book_shortage(
        conn: &mut Conn,
        book_suppliers: &Vec<(u32, u32, u32)>,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO shortages(registration_date) VALUES(NOW());";
        query.with(()).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as shortage_id;";
        let shortage_id = query.with(()).first::<u32, &mut Conn>(conn).await?;

        let shortage_id = match shortage_id {
            Some(shortage_id) => {
                for (book_id, supplier_id, shortage) in book_suppliers {
                    let query = r"INSERT INTO book_shortages(shortage_id,book_id,supplier_id,shortage) VALUES(:shortage_id,:book_id,:supplier_id,:shortage);";
                    let params = params! {
                        "shortage_id" => shortage_id,
                        "book_id" => book_id,
                        "supplier_id" => supplier_id,
                        "shortage" => shortage,
                    };
                    query.with(params).run(&mut *conn).await?;
                }
                Ok(Some(shortage_id))
            }
            None => anyhow::bail!("create shortage failed"),
        };
        shortage_id
    }

    pub async fn get_shortage_list(conn: &mut Conn) -> anyhow::Result<Vec<Shortage>> {
        let query = r"SELECT shortage_id,registration_date,is_resolved FROM shortages;";
        let result = query
            .map(conn, |(shortage_id, registration_date, is_resolved)| {
                Shortage {
                    id: shortage_id,
                    registration_date,
                    is_resolved,
                    ..Default::default()
                }
            })
            .await?;
        Ok(result)
    }

    pub async fn get_shortage_detail(
        conn: &mut Conn,
        shortage_id: u32,
    ) -> anyhow::Result<Option<Shortage>> {
        let query = r"SELECT
	shortages.shortage_id,
	shortages.registration_date,
	shortages.is_resolved,
	GROUP_CONCAT(
		DISTINCT CONCAT( book_shortages.shortage_item_id, ',', book_shortages.shortage_id, ',', book_shortages.book_id, ',', book_shortages.supplier_id, ',', book_shortages.shortage ) SEPARATOR ';'
	)
FROM
	shortages
	LEFT JOIN book_shortages ON book_shortages.shortage_id = shortages.shortage_id
WHERE
	shortages.shortage_id = :shortage_id
GROUP BY
	shortages.shortage_id;";
        let params = params! {
            "shortage_id" => shortage_id,
        };
        let mut result = query
            .with(params)
            .map(
                conn,
                |(shortage_id, registration_date, is_resolved, items)| Shortage {
                    id: shortage_id,
                    registration_date,
                    items: {
                        let items: Option<String> = items;
                        match items {
                            Some(items) => items
                                .split(';')
                                .map(|item| {
                                    let mut iter = item.split(',');
                                    ShortageItem {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        shortage_id: iter.next().unwrap().parse().unwrap(),
                                        book_id: iter.next().unwrap().parse().unwrap(),
                                        supplier_id: iter.next().unwrap().parse().unwrap(),
                                        shortage: iter.next().unwrap().parse().unwrap(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    is_resolved,
                },
            )
            .await?;

        Ok(result.pop())
    }
}
