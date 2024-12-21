use crate::entity::{Author, Book, BookInSeries, Keyword, Publisher, Supplier};
use mysql_async::prelude::{Query, WithParams};
use mysql_async::{params, Conn};
use mysql_common::bigdecimal::BigDecimal;

pub struct BookRepo;

impl BookRepo {
    pub async fn get_book_detail(conn: &mut Conn, book_id: u32) -> anyhow::Result<Option<Book>> {
        let query = r"SELECT
	DISTINCT books.book_id,
	books.isbn,
	books.title,
	GROUP_CONCAT( DISTINCT CONCAT( `authors`.author_id, ',', `authors`.`name` ) ORDER BY book_authors.`order` ASC SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( keywords.keyword_id, ',', keywords.keyword ) SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( publishers.publisher_id, ',', publishers.`name` ) SEPARATOR ';' ),
	GROUP_CONCAT(
		DISTINCT CONCAT(
			suppliers.supplier_id,
			',',
			suppliers.`name`,
			',',
			suppliers.telephone,
			',',
			suppliers.email,
			',',
			suppliers.address,
			',',
			suppliers.fax
		) SEPARATOR ';'
),
GROUP_CONCAT( DISTINCT CONCAT( series.series_id, ',', series.series_title, ',', series_books.column_num ) SEPARATOR ';' ) AS series,
books.price,
books.catalog,
books.cover,
books.is_onstore
FROM
	books
	LEFT JOIN publishers ON publishers.publisher_id = books.publisher_id
	LEFT JOIN book_authors ON book_authors.book_id = books.book_id
	LEFT JOIN `authors` ON `authors`.author_id = book_authors.author_id
	LEFT JOIN book_suppliers ON book_suppliers.book_id = books.book_id
	LEFT JOIN suppliers ON book_suppliers.supplier_id = suppliers.supplier_id
	LEFT JOIN book_keywords ON book_keywords.book_id = books.book_id
	LEFT JOIN keywords ON keywords.keyword_id = keywords.keyword_id
	LEFT JOIN series_books ON series_books.book_id = books.book_id
	LEFT JOIN series ON series_books.series_id = series.series_id
WHERE
	books.book_id = :book_id
GROUP BY books.book_id;";
        let params = params! {
            "book_id" => book_id,
        };
        let mut result = query
            .with(params)
            .map(
                conn,
                |(
                    book_id,
                    isbn,
                    title,
                    authors,
                    keywords,
                    publisher,
                    suppliers,
                    series,
                    price,
                    catalog,
                    cover,
                    is_onstore,
                )| Book {
                    id: book_id,
                    isbn,
                    title,
                    authors: {
                        let authors: Option<String> = authors;
                        match authors {
                            Some(authors) => authors
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Author {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    keywords: {
                        let keywords: Option<String> = keywords;
                        match keywords {
                            Some(keywords) => keywords
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Keyword {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        keyword: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    publisher: {
                        let publisher: Option<String> = publisher;
                        match publisher {
                            Some(publisher) => {
                                let mut iter = publisher.split(',');
                                Publisher {
                                    id: iter.next().unwrap().parse().unwrap(),
                                    name: iter.next().unwrap().to_string(),
                                }
                            }
                            None => Publisher {
                                id: 0,
                                name: "".to_string(),
                            },
                        }
                    },
                    suppliers: {
                        let suppliers: Option<String> = suppliers;
                        match suppliers {
                            Some(suppliers) => suppliers
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Supplier {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                        telephone: iter.next().unwrap().to_string(),
                                        email: iter.next().unwrap().to_string(),
                                        address: iter.next().unwrap().to_string(),
                                        fax: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    in_series: {
                        let series: Option<String> = series;

                        match series {
                            Some(series) => series
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    BookInSeries {
                                        series_id: iter.next().unwrap().parse().unwrap(),
                                        title: iter.next().unwrap().to_string(),
                                        column: iter.next().unwrap().parse().unwrap(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    price,
                    catalog,
                    cover,
                    is_onstore,
                },
            )
            .await?;

        Ok(result.pop())
    }

    pub async fn get_author(conn: &mut Conn, author_id: u32) -> anyhow::Result<Option<Author>> {
        let query = r"SELECT author_id,name FROM authors WHERE author_id=:author_id;";
        let params = params! {
            "author_id" => author_id,
        };
        let mut result = query
            .with(params)
            .map(conn, |(author_id, name)| Author {
                id: author_id,
                name,
            })
            .await?;

        Ok(result.pop())
    }

    pub async fn get_book_list(conn: &mut Conn) -> anyhow::Result<Vec<Book>> {
        let query = r"SELECT
	books.book_id,
	books.isbn,
	books.title,
	GROUP_CONCAT( DISTINCT CONCAT( `authors`.author_id, ',', `authors`.`name` ) ORDER BY book_authors.`order` ASC SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( keywords.keyword_id, ',', keywords.keyword ) SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( publishers.publisher_id, ',', publishers.`name` ) SEPARATOR ';' ),
	GROUP_CONCAT(
		DISTINCT CONCAT(
			suppliers.supplier_id,
			',',
			suppliers.`name`,
			',',
			suppliers.telephone,
			',',
			suppliers.email,
			',',
			suppliers.address,
			',',
			suppliers.fax
		) SEPARATOR ';'
),
GROUP_CONCAT( DISTINCT CONCAT( series.series_id, ',', series.series_title, ',', series_books.column_num ) SEPARATOR ';' ) AS series,
books.price,
books.catalog,
books.cover,
books.is_onstore
FROM
	books
	LEFT JOIN publishers ON publishers.publisher_id = books.book_id
	LEFT JOIN book_authors ON book_authors.book_id = books.book_id
	LEFT JOIN `authors` ON `authors`.author_id = book_authors.author_id
	LEFT JOIN book_suppliers ON book_suppliers.book_id = books.book_id
	LEFT JOIN suppliers ON book_suppliers.supplier_id = suppliers.supplier_id
	LEFT JOIN book_keywords ON book_keywords.book_id = books.book_id
	LEFT JOIN keywords ON keywords.keyword_id = keywords.keyword_id
	LEFT JOIN series_books ON series_books.book_id = books.book_id
	LEFT JOIN series ON series_books.series_id = series.series_id
GROUP BY books.book_id;";
        let result = query
            .with(())
            .map(
                conn,
                |(
                    book_id,
                    isbn,
                    title,
                    authors,
                    keywords,
                    publisher,
                    suppliers,
                    series,
                    price,
                    catalog,
                    cover,
                    is_onstore,
                )| Book {
                    id: book_id,
                    isbn,
                    title,
                    authors: {
                        let authors: Option<String> = authors;
                        match authors {
                            Some(authors) => authors
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Author {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    keywords: {
                        let keywords: Option<String> = keywords;
                        match keywords {
                            Some(keywords) => keywords
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Keyword {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        keyword: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    publisher: {
                        let publisher: Option<String> = publisher;
                        match publisher {
                            Some(publisher) => {
                                let mut iter = publisher.split(',');
                                Publisher {
                                    id: iter.next().unwrap().parse().unwrap(),
                                    name: iter.next().unwrap().to_string(),
                                }
                            }
                            None => Publisher {
                                id: 0,
                                name: "".to_string(),
                            },
                        }
                    },
                    suppliers: {
                        let suppliers: Option<String> = suppliers;
                        match suppliers {
                            Some(suppliers) => suppliers
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    Supplier {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                        telephone: iter.next().unwrap().to_string(),
                                        email: iter.next().unwrap().to_string(),
                                        address: iter.next().unwrap().to_string(),
                                        fax: iter.next().unwrap().to_string(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    in_series: {
                        let series: Option<String> = series;

                        match series {
                            Some(series) => series
                                .split(';')
                                .map(|s| {
                                    let mut iter = s.split(',');
                                    BookInSeries {
                                        series_id: iter.next().unwrap().parse().unwrap(),
                                        title: iter.next().unwrap().to_string(),
                                        column: iter.next().unwrap().parse().unwrap(),
                                    }
                                })
                                .collect(),
                            None => Vec::new(),
                        }
                    },
                    price,
                    catalog,
                    cover,
                    is_onstore,
                },
            )
            .await?;
        Ok(result)
    }

    pub async fn search_by_title_natural(
        conn: &mut Conn,
        title: &str,
    ) -> anyhow::Result<Vec<Book>> {
        let query = r"SELECT
	books.book_id,
	books.isbn,
	books.title,
	GROUP_CONCAT( DISTINCT CONCAT( `authors`.author_id, ',', `authors`.`name` ) ORDER BY book_authors.`order` ASC SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( keywords.keyword_id, ',', keywords.keyword ) SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( publishers.publisher_id, ',', publishers.`name` ) SEPARATOR ';' ),
	GROUP_CONCAT(
		DISTINCT CONCAT(
			suppliers.supplier_id,
			',',
			suppliers.`name`,
			',',
			suppliers.telephone,
			',',
			suppliers.email,
			',',
			suppliers.address,
			',',
			suppliers.fax
		) SEPARATOR ';'
),
GROUP_CONCAT( DISTINCT CONCAT( series.series_id, ',', series.series_title, ',', series_books.column_num ) SEPARATOR ';' ) AS series,
books.price,
books.catalog,
books.cover,
books.is_onstore
FROM
	books
	LEFT JOIN publishers ON publishers.publisher_id = books.book_id
	LEFT JOIN book_authors ON book_authors.book_id = books.book_id
	LEFT JOIN `authors` ON `authors`.author_id = book_authors.author_id
	LEFT JOIN book_suppliers ON book_suppliers.book_id = books.book_id
	LEFT JOIN suppliers ON book_suppliers.supplier_id = suppliers.supplier_id
	LEFT JOIN book_keywords ON book_keywords.book_id = books.book_id
	LEFT JOIN keywords ON keywords.keyword_id = keywords.keyword_id
	LEFT JOIN series_books ON series_books.book_id = books.book_id
	LEFT JOIN series ON series_books.series_id = series.series_id
WHERE
	MATCH ( books.title ) AGAINST ( :title IN NATURAL LANGUAGE MODE )
GROUP BY
	books.book_id
ORDER BY
	MATCH ( books.title ) AGAINST ( :title IN NATURAL LANGUAGE MODE ) DESC;";
        let params = params! {
            "title" => title,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    book_id,
                    isbn,
                    title,
                    authors,
                    keywords,
                    publisher,
                    suppliers,
                    series,
                    price,
                    catalog,
                    cover,
                    is_onstore,
                )| {
                    Book {
                        id: book_id,
                        isbn,
                        title,
                        authors: {
                            let authors: Option<String> = authors;
                            match authors {
                                Some(authors) => authors
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Author {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        keywords: {
                            let keywords: Option<String> = keywords;
                            match keywords {
                                Some(keywords) => keywords
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Keyword {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            keyword: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        publisher: {
                            let publisher: Option<String> = publisher;
                            match publisher {
                                Some(publisher) => {
                                    let mut iter = publisher.split(',');
                                    Publisher {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                    }
                                }
                                None => Publisher {
                                    id: 0,
                                    name: "".to_string(),
                                },
                            }
                        },
                        suppliers: {
                            let suppliers: Option<String> = suppliers;
                            match suppliers {
                                Some(suppliers) => suppliers
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Supplier {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                            telephone: iter.next().unwrap().to_string(),
                                            email: iter.next().unwrap().to_string(),
                                            address: iter.next().unwrap().to_string(),
                                            fax: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        in_series: {
                            let series: Option<String> = series;

                            match series {
                                Some(series) => series
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        BookInSeries {
                                            series_id: iter.next().unwrap().parse().unwrap(),
                                            title: iter.next().unwrap().to_string(),
                                            column: iter.next().unwrap().parse().unwrap(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        price,
                        catalog,
                        cover,
                        is_onstore,
                    }
                },
            )
            .await?;

        Ok(result)
    }

    pub async fn search_by_author_natural(
        conn: &mut Conn,
        authors: &str,
    ) -> anyhow::Result<Vec<Book>> {
        let query = r"SELECT
	books.book_id,
	books.isbn,
	books.title,
	GROUP_CONCAT( DISTINCT CONCAT( `authors`.author_id, ',', `authors`.`name` ) ORDER BY book_authors.`order` ASC SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( keywords.keyword_id, ',', keywords.keyword ) SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( publishers.publisher_id, ',', publishers.`name` ) SEPARATOR ';' ),
GROUP_CONCAT(
	DISTINCT CONCAT(
		suppliers.supplier_id,
		',',
		suppliers.`name`,
		',',
		suppliers.telephone,
		',',
		suppliers.email,
		',',
		suppliers.address,
		',',
		suppliers.fax
	) SEPARATOR ';'
),
GROUP_CONCAT( DISTINCT CONCAT( series.series_id, ',', series.series_title, ',', series_books.column_num ) SEPARATOR ';' ) AS series,
books.price,
books.catalog,
books.cover,
books.is_onstore
FROM
	books
	LEFT JOIN publishers ON publishers.publisher_id = books.book_id
	LEFT JOIN book_authors ON book_authors.book_id = books.book_id
	LEFT JOIN `authors` ON `authors`.author_id = book_authors.author_id
	LEFT JOIN book_suppliers ON book_suppliers.book_id = books.book_id
	LEFT JOIN suppliers ON book_suppliers.supplier_id = suppliers.supplier_id
	LEFT JOIN book_keywords ON book_keywords.book_id = books.book_id
	LEFT JOIN keywords ON keywords.keyword_id = keywords.keyword_id
	LEFT JOIN series_books ON series_books.book_id = books.book_id
	LEFT JOIN series ON series_books.series_id = series.series_id
WHERE
	MATCH ( `authors`.`name` ) AGAINST ( :authors IN NATURAL LANGUAGE MODE )
GROUP BY
	books.book_id
ORDER BY
	SUM( MATCH ( `authors`.`name` ) AGAINST ( :authors IN NATURAL LANGUAGE MODE ) ) DESC;";
        let params = params! {
            "authors" => authors,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    book_id,
                    isbn,
                    title,
                    authors,
                    keywords,
                    publisher,
                    suppliers,
                    series,
                    price,
                    catalog,
                    cover,
                    is_onstore,
                )| {
                    Book {
                        id: book_id,
                        isbn,
                        title,
                        authors: {
                            let authors: Option<String> = authors;
                            match authors {
                                Some(authors) => authors
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Author {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        keywords: {
                            let keywords: Option<String> = keywords;
                            match keywords {
                                Some(keywords) => keywords
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Keyword {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            keyword: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        publisher: {
                            let publisher: Option<String> = publisher;
                            match publisher {
                                Some(publisher) => {
                                    let mut iter = publisher.split(',');
                                    Publisher {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                    }
                                }
                                None => Publisher {
                                    id: 0,
                                    name: "".to_string(),
                                },
                            }
                        },
                        suppliers: {
                            let suppliers: Option<String> = suppliers;
                            match suppliers {
                                Some(suppliers) => suppliers
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Supplier {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                            telephone: iter.next().unwrap().to_string(),
                                            email: iter.next().unwrap().to_string(),
                                            address: iter.next().unwrap().to_string(),
                                            fax: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        in_series: {
                            let series: Option<String> = series;

                            match series {
                                Some(series) => series
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        BookInSeries {
                                            series_id: iter.next().unwrap().parse().unwrap(),
                                            title: iter.next().unwrap().to_string(),
                                            column: iter.next().unwrap().parse().unwrap(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        price,
                        catalog,
                        cover,
                        is_onstore,
                    }
                },
            )
            .await?;

        Ok(result)
    }

    pub async fn search_by_keyword_natural(
        conn: &mut Conn,
        keywords: &str,
    ) -> anyhow::Result<Vec<Book>> {
        let query = r"SELECT
	books.book_id,
	books.isbn,
	books.title,
	GROUP_CONCAT( DISTINCT CONCAT( `authors`.author_id, ',', `authors`.`name` ) ORDER BY book_authors.`order` ASC SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( keywords.keyword_id, ',', keywords.keyword ) SEPARATOR ';' ),
GROUP_CONCAT( DISTINCT CONCAT( publishers.publisher_id, ',', publishers.`name` ) SEPARATOR ';' ),
GROUP_CONCAT(
	DISTINCT CONCAT(
		suppliers.supplier_id,
		',',
		suppliers.`name`,
		',',
		suppliers.telephone,
		',',
		suppliers.email,
		',',
		suppliers.address,
		',',
		suppliers.fax
	) SEPARATOR ';'
),
GROUP_CONCAT( DISTINCT CONCAT( series.series_id, ',', series.series_title, ',', series_books.column_num ) SEPARATOR ';' ) AS series,
books.price,
books.catalog,
books.cover,
books.is_onstore
FROM
	books
	LEFT JOIN publishers ON publishers.publisher_id = books.book_id
	LEFT JOIN book_authors ON book_authors.book_id = books.book_id
	LEFT JOIN `authors` ON `authors`.author_id = book_authors.author_id
	LEFT JOIN book_suppliers ON book_suppliers.book_id = books.book_id
	LEFT JOIN suppliers ON book_suppliers.supplier_id = suppliers.supplier_id
	LEFT JOIN book_keywords ON book_keywords.book_id = books.book_id
	LEFT JOIN keywords ON keywords.keyword_id = keywords.keyword_id
	LEFT JOIN series_books ON series_books.book_id = books.book_id
	LEFT JOIN series ON series_books.series_id = series.series_id
WHERE
	MATCH ( keywords.keyword ) AGAINST ( :keywords IN NATURAL LANGUAGE MODE )
GROUP BY
	books.book_id
ORDER BY
	SUM( MATCH ( keywords.keyword ) AGAINST ( :keywords IN NATURAL LANGUAGE MODE ) ) DESC;";
        let params = params! {
            "keywords" => keywords,
        };
        let result = query
            .with(params)
            .map(
                conn,
                |(
                    book_id,
                    isbn,
                    title,
                    authors,
                    keywords,
                    publisher,
                    suppliers,
                    series,
                    price,
                    catalog,
                    cover,
                    is_onstore,
                )| {
                    Book {
                        id: book_id,
                        isbn,
                        title,
                        authors: {
                            let authors: Option<String> = authors;
                            match authors {
                                Some(authors) => authors
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Author {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        keywords: {
                            let keywords: Option<String> = keywords;
                            match keywords {
                                Some(keywords) => keywords
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Keyword {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            keyword: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        publisher: {
                            let publisher: Option<String> = publisher;
                            match publisher {
                                Some(publisher) => {
                                    let mut iter = publisher.split(',');
                                    Publisher {
                                        id: iter.next().unwrap().parse().unwrap(),
                                        name: iter.next().unwrap().to_string(),
                                    }
                                }
                                None => Publisher {
                                    id: 0,
                                    name: "".to_string(),
                                },
                            }
                        },
                        suppliers: {
                            let suppliers: Option<String> = suppliers;
                            match suppliers {
                                Some(suppliers) => suppliers
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        Supplier {
                                            id: iter.next().unwrap().parse().unwrap(),
                                            name: iter.next().unwrap().to_string(),
                                            telephone: iter.next().unwrap().to_string(),
                                            email: iter.next().unwrap().to_string(),
                                            address: iter.next().unwrap().to_string(),
                                            fax: iter.next().unwrap().to_string(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        in_series: {
                            let series: Option<String> = series;

                            match series {
                                Some(series) => series
                                    .split(';')
                                    .map(|s| {
                                        let mut iter = s.split(',');
                                        BookInSeries {
                                            series_id: iter.next().unwrap().parse().unwrap(),
                                            title: iter.next().unwrap().to_string(),
                                            column: iter.next().unwrap().parse().unwrap(),
                                        }
                                    })
                                    .collect(),
                                None => Vec::new(),
                            }
                        },
                        price,
                        catalog,
                        cover,
                        is_onstore,
                    }
                },
            )
            .await?;

        Ok(result)
    }

    pub async fn add_book(
        conn: &mut Conn,
        isbn: &str,
        title: &str,
        authors: &Vec<u32>,
        keywords: &Vec<u32>,
        publisher: u32,
        price: BigDecimal,
        catalog: &str,
        cover: &str,
        is_onstore: bool,
    ) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO books(isbn,title,publisher_id,price,catalog,cover,is_onstore) VALUES(:isbn,:title,:publisher_id,:price,:catalog,:cover,:is_onstore);";
        let params = params! {
            "isbn" => isbn,
            "title" => title,
            "publisher_id" => publisher,
            "price" => price,
            "catalog" => catalog,
            "cover" => cover,
            "is_onstore" => is_onstore,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as book_id;";
        let book_id = query.with(()).first::<u32, &mut Conn>(conn).await?;

        let book_id = match book_id {
            Some(book_id) => {
                for author in authors {
                    let query =
                        r"INSERT INTO book_authors(book_id,author_id) VALUES(:book_id,:author_id);";
                    let params = params! {
                        "book_id" => book_id,
                        "author_id" => author,
                    };
                    query.with(params).run(&mut *conn).await?;
                }
                for keyword in keywords {
                    let query = r"INSERT INTO book_keywords(book_id,keyword_id) VALUES(:book_id,:keyword_id);";
                    let params = params! {
                        "book_id" => book_id,
                        "keyword_id" => keyword,
                    };
                    query.with(params).run(&mut *conn).await?;
                }
                Ok(Some(book_id))
            }
            None => anyhow::bail!("add book failed"),
        };
        book_id
    }

    pub async fn get_keyword(conn: &mut Conn, keyword_id: u32) -> anyhow::Result<Option<Keyword>> {
        let query = r"SELECT keyword_id,keyword FROM keywords WHERE keyword_id=:keyword_id;";
        let params = params! {
            "keyword_id" => keyword_id,
        };
        let mut result = query
            .with(params)
            .map(conn, |(keyword_id, keyword)| Keyword {
                id: keyword_id,
                keyword,
            })
            .await?;

        Ok(result.pop())
    }

    pub async fn get_keyword_list(conn: &mut Conn) -> anyhow::Result<Vec<Keyword>> {
        let query = r"SELECT keyword_id,keyword FROM keywords;";
        let result = query
            .with(())
            .map(conn, |(keyword_id, keyword)| Keyword {
                id: keyword_id,
                keyword,
            })
            .await?;
        Ok(result)
    }

    pub async fn add_keyword(conn: &mut Conn, keyword: &str) -> anyhow::Result<Option<u32>> {
        let query = r"INSERT INTO keywords(keyword) VALUES(:keyword);";
        let params = params! {
            "keyword" => keyword,
        };
        query.with(params).run(&mut *conn).await?;
        let query = r"SELECT LAST_INSERT_ID() as keyword_id;";
        let keyword_id = query.with(()).first::<u32, &mut Conn>(conn).await?;
        Ok(keyword_id)
    }
}
