use bookstore_admin::network::{
    url_get_image_buffer, url_post, AdminDetailRequest, AdminDetailResponse, AdminLoginRequest,
    AdminLoginResponse, AdminRegisterRequest, AdminRegisterResponse, AuthorListRequest,
    AuthorListResponse, BookAddRequest, BookAddResponse, BookDetailRequest, BookDetailResponse,
    BookListRequest, BookListResponse, BookUpdateRequest, BookUpdateResponse,
    CustomerBalanceRequest, CustomerBalanceResponse, CustomerCreditRequest, CustomerCreditResponse,
    CustomerListRequest, CustomerListResponse, CustomerOrderListRequest, CustomerOrderListResponse,
    KeywordListRequest, KeywordListResponse, LocationListRequest, LocationListResponse,
    PublisherListRequest, PublisherListResponse, SeriesListRequest, SeriesListResponse,
    ShipOrderAutoRequest, ShipOrderAutoResponse, StockChangeRequest, StockChangeResponse,
    StockTransferRequest, StockTransferResponse, SupplierListRequest, SupplierListResponse,
};
use bookstore_admin::util::runtime::Runtime;
use bookstore_admin::util::token::{AdminToken, Token};
use slint::{Image, Model, ModelRc, VecModel};

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    pretty_env_logger::init();

    let main_window = MainWindow::new()?;
    let rt = Runtime::new().unwrap();

    let admin_token = AdminToken::default();

    main_window.on_user_login({
        let main_window = main_window.as_weak();
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        move |username, password| {
            let main_window = main_window.clone();
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match url_post::<AdminLoginResponse, AdminLoginRequest>(
                            "/admin/login",
                            AdminLoginRequest {
                                username: username.to_string(),
                                password: password.to_string(),
                            },
                        )
                        .await
                        {
                            Ok(response) => {
                                admin_token
                                    .set(Token {
                                        token: response.token,
                                        tag: response.tag,
                                        nonce: response.nonce,
                                    })
                                    .await;
                                Ok(())
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.set_has_login(true);
                        main_window.set_error_login(false);
                    }
                    Err(e) => {
                        let main_window = main_window.unwrap();
                        main_window.set_error_login(true);
                        main_window.set_error_login_message(e.to_string().into());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_user_register({
        // let main_window = main_window.as_weak();
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        move || {
            let register_window = RegisterWindow::new().unwrap();
            register_window.on_register({
                let register_window = register_window.as_weak();
                let rt = rt.clone();
                let admin_token = admin_token.clone();
                move |username, password, role| {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let register_window = register_window.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => {
                                        match url_post::<AdminRegisterResponse, AdminRegisterRequest>(
                                            "/admin/register",
                                            AdminRegisterRequest {
                                                username: username.to_string(),
                                                password: password.to_string(),
                                                role: match role {
                                                    0 => "admin".to_string(),
                                                    1 => "staff".to_string(),
                                                    _ => anyhow::bail!("invalid role"),
                                                },
                                                token: token.token,
                                                tag: token.tag,
                                                nonce: token.nonce,
                                            },
                                        )
                                            .await
                                        {
                                            Ok(response) => Ok(response),
                                            Err(e) => Err(e),
                                        }
                                    }
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok(response) => {
                                // let main_window = main_window.unwrap();
                                // main_window.set_has_register(true);
                                // main_window.set_error_register(false);
                                let register_window = register_window.unwrap();
                                register_window.set_register_message(format!("register success, admin_id: {}", response.admin_id).into());
                            }
                            Err(e) => {
                                // let main_window = main_window.unwrap();
                                // main_window.set_error_register(true);
                                // main_window.set_error_register_message(e.to_string().into());
                                let register_window = register_window.unwrap();
                                register_window.set_register_message(e.to_string().into());
                            }
                        }
                    }).unwrap();
                }
            }
            );
            register_window.show().unwrap();
        }
    });

    main_window.on_get_user_detail({
        let main_window = main_window.as_weak();
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        move || {
            let main_window = main_window.clone();
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<AdminDetailResponse, AdminDetailRequest>(
                                    "/admin/detail",
                                    AdminDetailRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(admin_detail) => {
                        let main_window = main_window.unwrap();
                        let admin_detail = UserDetail {
                            admin_id: admin_detail.admin_id as i32,
                            username: admin_detail.username.into(),
                            role: admin_detail.role.into(),
                        };
                        main_window.set_user_detail(admin_detail);
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_book_list({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                match rt
                    .spawn(async move {
                        match url_post::<BookListResponse, BookListRequest>(
                            "/book/list",
                            Default::default(),
                        )
                        .await
                        {
                            Ok(book_list) => {
                                let mut book_cover_buffers =
                                    Vec::with_capacity(book_list.books.len());

                                for book in book_list.books.iter() {
                                    book_cover_buffers
                                        .push(url_get_image_buffer(&book.cover).await.ok());
                                }
                                Ok((book_list, book_cover_buffers))
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok((book_list, book_cover_buffers)) => {
                        let books = book_list
                            .books
                            .into_iter()
                            .zip(book_cover_buffers.into_iter())
                            .map(|(book, cover)| {
                                let book_id = book.book_id as i32;
                                let title = book.title.into();
                                let isbn = book.isbn.into();
                                let authors = ModelRc::new(VecModel::from(
                                    book.authors
                                        .into_iter()
                                        .map(|author| author.name.into())
                                        .collect::<Vec<_>>(),
                                ));
                                let publisher = book.publisher.name.into();
                                let price = book.price.into();
                                let keywords = ModelRc::new(VecModel::from(
                                    book.keywords
                                        .into_iter()
                                        .map(|keyword| keyword.keyword.into())
                                        .collect::<Vec<_>>(),
                                ));
                                let cover = match cover {
                                    Some(buffer) => Image::from_rgba8(buffer),
                                    None => Image::default(),
                                };
                                Book {
                                    id: book_id,
                                    title,
                                    isbn,
                                    authors,
                                    publisher,
                                    price,
                                    keywords,
                                    cover,
                                }
                            })
                            .collect::<Vec<_>>();

                        let main_window = main_window.unwrap();
                        main_window.set_books(ModelRc::new(VecModel::from(books)));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_open_book_detail({
        let rt = rt.as_weak();
        // let main_window = main_window.as_weak();
        let admin_token = admin_token.as_weak();
        move |book_id| {
            let rt = rt.clone();
            // let main_window = main_window.clone();
            let admin_token = admin_token.clone();
            let admin_token_weak = admin_token.clone();

            let book_detail_window = BookDetailWindow::new().unwrap();
            let book_detail_window_weak = book_detail_window.as_weak();

            book_detail_window.on_get_book_detail({
                let book_detail_window = book_detail_window.as_weak();
                let rt = rt.clone();
                let admin_token = admin_token_weak.clone();
                move || {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let book_detail_window = book_detail_window.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => match url_post::<
                                        BookDetailResponse,
                                        BookDetailRequest,
                                    >(
                                        "/admin/book/detail",
                                        BookDetailRequest {
                                            token: token.token,
                                            tag: token.tag,
                                            nonce: token.nonce,
                                            book_id: book_id as u32,
                                        },
                                    )
                                        .await
                                    {
                                        Ok(response) => {
                                            let book_cover_buffer =
                                                url_get_image_buffer(&response.cover).await.ok();
                                            Ok((response, book_cover_buffer))
                                        }
                                        Err(e) => Err(e),
                                    },
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok((book_detail, book_cover_buffer)) => {
                                let book_detail = BookInDetail {
                                    id: book_detail.book_id as i32,
                                    isbn: book_detail.isbn.into(),
                                    title: book_detail.title.into(),
                                    authors: ModelRc::new(VecModel::from(
                                        book_detail
                                            .authors
                                            .into_iter()
                                            .map(|author| author.name.into())
                                            .collect::<Vec<_>>(),
                                    )),
                                    publisher: book_detail.publisher.name.into(),
                                    suppliers: ModelRc::new(VecModel::from(
                                        book_detail
                                            .suppliers
                                            .into_iter()
                                            .map(|supplier| supplier.name.into())
                                            .collect::<Vec<_>>(),
                                    )),
                                    series: ModelRc::new(VecModel::from(
                                        book_detail
                                            .in_series
                                            .into_iter()
                                            .map(|series| series.name.into())
                                            .collect::<Vec<_>>(),
                                    )),
                                    price: book_detail.price.into(),
                                    keywords: ModelRc::new(VecModel::from(
                                        book_detail
                                            .keywords
                                            .into_iter()
                                            .map(|keyword| keyword.keyword.into())
                                            .collect::<Vec<_>>(),
                                    )),
                                    locations: ModelRc::new(VecModel::from(
                                        book_detail
                                            .locations
                                            .into_iter()
                                            .map(|location| LocationInDetail {
                                                id: location.id as i32,
                                                description: location.description.into(),
                                                quantity: location.quantity as i32,
                                            })
                                            .collect::<Vec<_>>(),
                                    )),
                                    catalog: book_detail.catalog.into(),
                                    cover: match book_cover_buffer {
                                        Some(buffer) => Image::from_rgba8(buffer),
                                        None => Image::default(),
                                    },
                                    on_store: book_detail.is_onstore,
                                };
                                let book_detail_window = book_detail_window.unwrap();
                                book_detail_window.set_book(book_detail);
                            }
                            Err(e) => {
                                log::error!("{}", e.to_string());
                            }
                        }
                    })
                        .unwrap();
                }
            });

            book_detail_window.on_get_location_list({
                let book_detail_window = book_detail_window.as_weak();
                let rt = rt.clone();
                let admin_token = admin_token_weak.clone();
                move || {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let book_detail_window = book_detail_window.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => {
                                        match url_post::<LocationListResponse, LocationListRequest>(
                                            "/admin/location/list",
                                            LocationListRequest {
                                                token: token.token,
                                                tag: token.tag,
                                                nonce: token.nonce,
                                            },
                                        )
                                            .await
                                        {
                                            Ok(response) => Ok(response),
                                            Err(e) => Err(e),
                                        }
                                    }
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok(location_list) => {
                                let location_ids = location_list
                                    .locations
                                    .iter()
                                    .map(|location| location.id as i32)
                                    .collect::<Vec<_>>();
                                let locations = location_list
                                    .locations
                                    .into_iter()
                                    .map(|location| location.description.into())
                                    .collect::<Vec<_>>();
                                let book_detail_window = book_detail_window.unwrap();

                                book_detail_window
                                    .set_location_ids(ModelRc::new(VecModel::from(location_ids)));
                                book_detail_window
                                    .set_locations(ModelRc::new(VecModel::from(locations)));
                            }
                            Err(e) => {
                                log::error!("{}", e.to_string());
                            }
                        }
                    })
                        .unwrap();
                }
            });

            book_detail_window.on_edit_stock({
                let book_detail_window = book_detail_window.as_weak();
                let rt = rt.clone();
                let admin_token = admin_token_weak.clone();
                move |book_id, location_id, quantity| {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let book_detail_window = book_detail_window.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => {
                                        match url_post::<StockChangeResponse, StockChangeRequest>(
                                            "/admin/stock/change",
                                            StockChangeRequest {
                                                token: token.token,
                                                tag: token.tag,
                                                nonce: token.nonce,
                                                book_id: book_id as u32,
                                                location_id: location_id as u32,
                                                quantity,
                                            },
                                        )
                                            .await
                                        {
                                            Ok(_) => Ok(()),
                                            Err(e) => Err(e),
                                        }
                                    }
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok(_) => {
                                let book_detail_window = book_detail_window.unwrap();
                                book_detail_window.invoke_get_book_detail();
                                // book_detail_window.set_stock_message("stock changed".into());
                            }
                            Err(e) => {
                                // let book_detail_window = book_detail_window.unwrap();
                                // book_detail_window.set_stock_message(e.to_string().into());
                                log::error!("{}", e.to_string());
                            }
                        }
                    })
                        .unwrap();
                }
            });

            book_detail_window.on_transfer_stock({
                let book_detail_window = book_detail_window.as_weak();
                let rt = rt.clone();
                let admin_token = admin_token_weak.clone();
                move |book_id, from_location_id, to_location_id, quantity| {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let book_detail_window = book_detail_window.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => {
                                        match url_post::<StockTransferResponse, StockTransferRequest>(
                                            "/admin/stock/transfer",
                                            StockTransferRequest {
                                                token: token.token,
                                                tag: token.tag,
                                                nonce: token.nonce,
                                                book_id: book_id as u32,
                                                from_location_id: from_location_id as u32,
                                                to_location_id: to_location_id as u32,
                                                quantity: quantity as u32,
                                            },
                                        )
                                            .await
                                        {
                                            Ok(_) => Ok(()),
                                            Err(e) => Err(e),
                                        }
                                    }
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok(_) => {
                                let book_detail_window = book_detail_window.unwrap();
                                book_detail_window.invoke_get_book_detail();
                                // book_detail_window.set_stock_message("stock changed".into());
                            }
                            Err(e) => {
                                // let book_detail_window = book_detail_window.unwrap();
                                // book_detail_window.set_stock_message(e.to_string().into());
                                log::error!("{}", e.to_string());
                            }
                        }
                    })
                        .unwrap();
                }
            });

            book_detail_window.on_open_edit_book({
                let rt = rt.clone();
                let admin_token = admin_token_weak.clone();
                move |book_id| {
                    let rt = rt.clone();
                    let admin_token = admin_token.clone();
                    let admin_token_weak = admin_token.clone();
                    let book_detail_window = book_detail_window_weak.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let admin_token = admin_token.unwrap();
                        match rt
                            .spawn(async move {
                                match admin_token.get().await {
                                    Some(token) => match url_post::<
                                        BookDetailResponse,
                                        BookDetailRequest,
                                    >(
                                        "/admin/book/detail",
                                        BookDetailRequest {
                                            token: token.token,
                                            tag: token.tag,
                                            nonce: token.nonce,
                                            book_id: book_id as u32,
                                        },
                                    )
                                        .await
                                    {
                                        Ok(response) => {
                                            Ok(response)
                                        }
                                        Err(e) => Err(e),
                                    },
                                    None => Err(anyhow::anyhow!("token not found")),
                                }
                            })
                            .await
                            .unwrap() {
                            Ok(book_detail) => {
                                let book_detail = BookInEdit {
                                    authors: ModelRc::new(VecModel::from(
                                        book_detail
                                            .authors
                                            .into_iter()
                                            .map(|author| AuthorInEdit {
                                                id: author.author_id as i32,
                                                name: author.name.into(),
                                            })
                                            .collect::<Vec<_>>(),
                                    )),
                                    catalog: book_detail.catalog.into(),
                                    cover: book_detail.cover.into(),
                                    id: book_detail.book_id as i32,
                                    isbn: book_detail.isbn.into(),
                                    keywords: ModelRc::new(VecModel::from(
                                        book_detail
                                            .keywords
                                            .into_iter()
                                            .map(|keyword| KeywordInEdit {
                                                id: keyword.keyword_id as i32,
                                                keyword: keyword.keyword.into(),
                                            })
                                            .collect::<Vec<_>>(),
                                    )),
                                    on_store: book_detail.is_onstore,
                                    price: book_detail.price.into(),
                                    publisher: PublisherInEdit {
                                        id: book_detail.publisher.publisher_id as i32,
                                        name: book_detail.publisher.name.into(),
                                    },
                                    series: ModelRc::new(VecModel::from(
                                        book_detail
                                            .in_series
                                            .into_iter()
                                            .map(|series| SeriesInEdit {
                                                id: series.series_id as i32,
                                                name: series.name.into(),
                                                column: series.column as i32,
                                            })
                                            .collect::<Vec<_>>(),
                                    )),
                                    suppliers: ModelRc::new(VecModel::from(
                                        book_detail
                                            .suppliers
                                            .into_iter()
                                            .map(|supplier| SupplierInEdit {
                                                id: supplier.supplier_id as i32,
                                                name: supplier.name.into(),
                                            })
                                            .collect::<Vec<_>>(),
                                    )),
                                    title: book_detail.title.into(),
                                };
                                let edit_book_window = BookEditWindow::new().unwrap();

                                edit_book_window.on_get_author_list({
                                    let edit_book_window = edit_book_window.as_weak();
                                    let rt = rt.as_weak();
                                    move || {
                                        let rt = rt.clone();
                                        let edit_book_window = edit_book_window.clone();
                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            match rt
                                                .spawn(async move {
                                                    match url_post::<AuthorListResponse, AuthorListRequest>(
                                                        "/book/author/list",
                                                        Default::default(),
                                                    )
                                                        .await
                                                    {
                                                        Ok(response) => Ok(response),
                                                        Err(e) => Err(e),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(author_list) => {
                                                    let author_ids = author_list
                                                        .authors
                                                        .iter()
                                                        .map(|author| author.author_id as i32)
                                                        .collect::<Vec<_>>();
                                                    let authors = author_list.authors
                                                        .into_iter()
                                                        .map(|author|
                                                            author.name.into()
                                                        )
                                                        .collect::<Vec<_>>();
                                                    let edit_book_window = edit_book_window.unwrap();
                                                    edit_book_window
                                                        .set_authors(ModelRc::new(VecModel::from(authors)));
                                                    edit_book_window
                                                        .set_author_ids(ModelRc::new(VecModel::from(author_ids)));
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        })
                                            .unwrap();
                                    }
                                });

                                edit_book_window.on_remove_author({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, author_id| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let authors = book.authors.iter().filter(|author| author.id != author_id).collect::<Vec<_>>();
                                        book.authors = ModelRc::new(VecModel::from(authors));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_add_author({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, author_id, author_name| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let authors = book.authors.iter().chain(VecModel::from(vec![AuthorInEdit {
                                            id: author_id,
                                            name: author_name.into(),
                                        }]).iter().filter(|author| !book.authors.iter().any(|a| a.id == author.id) && !author.name.is_empty())).collect::<Vec<_>>();
                                        book.authors = ModelRc::new(VecModel::from(authors));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_get_keyword_list({
                                    let edit_book_window = edit_book_window.as_weak();
                                    let rt = rt.as_weak();
                                    move || {
                                        let rt = rt.clone();
                                        let edit_book_window = edit_book_window.clone();
                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            match rt
                                                .spawn(async move {
                                                    match url_post::<KeywordListResponse, KeywordListRequest>(
                                                        "/book/keyword/list",
                                                        Default::default(),
                                                    )
                                                        .await
                                                    {
                                                        Ok(response) => Ok(response),
                                                        Err(e) => Err(e),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(keyword_list) => {
                                                    let keyword_ids = keyword_list
                                                        .keywords
                                                        .iter()
                                                        .map(|keyword| keyword.keyword_id as i32)
                                                        .collect::<Vec<_>>();
                                                    let keywords = keyword_list.keywords
                                                        .into_iter()
                                                        .map(|keyword|
                                                            keyword.keyword.into()
                                                        )
                                                        .collect::<Vec<_>>();
                                                    let edit_book_window = edit_book_window.unwrap();
                                                    edit_book_window
                                                        .set_keywords(ModelRc::new(VecModel::from(keywords)));
                                                    edit_book_window
                                                        .set_keyword_ids(ModelRc::new(VecModel::from(keyword_ids)));
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        })
                                            .unwrap();
                                    }
                                });

                                edit_book_window.on_remove_keyword({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, keyword_id| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let keywords = book.keywords.iter().filter(|keyword| keyword.id != keyword_id).collect::<Vec<_>>();
                                        book.keywords = ModelRc::new(VecModel::from(keywords));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_add_keyword({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, keyword_id, keyword_name| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let keywords = book.keywords.iter().chain(VecModel::from(vec![KeywordInEdit {
                                            id: keyword_id,
                                            keyword: keyword_name.into(),
                                        }]).iter().filter(|keyword| !book.keywords.iter().any(|k| k.id == keyword.id) && !keyword.keyword.is_empty())).collect::<Vec<_>>();
                                        book.keywords = ModelRc::new(VecModel::from(keywords));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_get_publisher_list({
                                    let edit_book_window = edit_book_window.as_weak();
                                    let rt = rt.as_weak();
                                    move || {
                                        let rt = rt.clone();
                                        let edit_book_window = edit_book_window.clone();
                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            match rt
                                                .spawn(async move {
                                                    match url_post::<PublisherListResponse, PublisherListRequest>(
                                                        "/book/publisher/list",
                                                        Default::default(),
                                                    )
                                                        .await
                                                    {
                                                        Ok(response) => Ok(response),
                                                        Err(e) => Err(e),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(publisher_list) => {
                                                    let publisher_ids = publisher_list
                                                        .publishers
                                                        .iter()
                                                        .map(|publisher| publisher.publisher_id as i32)
                                                        .collect::<Vec<_>>();
                                                    let publishers = publisher_list.publishers
                                                        .into_iter()
                                                        .map(|publisher|
                                                            publisher.name.into()
                                                        )
                                                        .collect::<Vec<_>>();
                                                    let edit_book_window = edit_book_window.unwrap();
                                                    edit_book_window
                                                        .set_publishers(ModelRc::new(VecModel::from(publishers)));
                                                    edit_book_window
                                                        .set_publisher_ids(ModelRc::new(VecModel::from(publisher_ids)));
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        })
                                            .unwrap();
                                    }
                                });

                                edit_book_window.on_change_publisher({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, publisher_id, publisher_name| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let publisher = PublisherInEdit {
                                            id: publisher_id,
                                            name: publisher_name.into(),
                                        };
                                        book.publisher = publisher;
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_get_supplier_list({
                                    let edit_book_window = edit_book_window.as_weak();
                                    let rt = rt.as_weak();
                                    move || {
                                        let rt = rt.clone();
                                        let edit_book_window = edit_book_window.clone();
                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            match rt
                                                .spawn(async move {
                                                    match url_post::<SupplierListResponse, SupplierListRequest>(
                                                        "/supplier/list",
                                                        Default::default(),
                                                    )
                                                        .await
                                                    {
                                                        Ok(response) => Ok(response),
                                                        Err(e) => Err(e),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(supplier_list) => {
                                                    let supplier_ids = supplier_list
                                                        .suppliers
                                                        .iter()
                                                        .map(|supplier| supplier.supplier_id as i32)
                                                        .collect::<Vec<_>>();
                                                    let suppliers = supplier_list.suppliers
                                                        .into_iter()
                                                        .map(|supplier|
                                                            supplier.name.into()
                                                        )
                                                        .collect::<Vec<_>>();
                                                    let edit_book_window = edit_book_window.unwrap();
                                                    edit_book_window
                                                        .set_suppliers(ModelRc::new(VecModel::from(suppliers)));
                                                    edit_book_window
                                                        .set_supplier_ids(ModelRc::new(VecModel::from(supplier_ids)));
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        }).unwrap();
                                    }
                                });

                                edit_book_window.on_remove_supplier({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, supplier_id| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let suppliers = book.suppliers.iter().filter(|supplier| supplier.id != supplier_id).collect::<Vec<_>>();
                                        book.suppliers = ModelRc::new(VecModel::from(suppliers));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_add_supplier({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, supplier_id, supplier_name| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let suppliers = book.suppliers.iter().chain(VecModel::from(vec![SupplierInEdit {
                                            id: supplier_id,
                                            name: supplier_name.into(),
                                        }]).iter().filter(|supplier| !book.suppliers.iter().any(|s| s.id == supplier.id) && !supplier.name.is_empty())).collect::<Vec<_>>();
                                        book.suppliers = ModelRc::new(VecModel::from(suppliers));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_get_series_list({
                                    let edit_book_window = edit_book_window.as_weak();
                                    let rt = rt.as_weak();
                                    move || {
                                        let rt = rt.clone();
                                        let edit_book_window = edit_book_window.clone();
                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            match rt
                                                .spawn(async move {
                                                    match url_post::<SeriesListResponse, SeriesListRequest>(
                                                        "/book/series/list",
                                                        Default::default(),
                                                    )
                                                        .await
                                                    {
                                                        Ok(response) => Ok(response),
                                                        Err(e) => Err(e),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(series_list) => {
                                                    let series_ids = series_list
                                                        .series
                                                        .iter()
                                                        .map(|series| series.series_id as i32)
                                                        .collect::<Vec<_>>();
                                                    let series = series_list.series
                                                        .into_iter()
                                                        .map(|series|
                                                            series.name.into()
                                                        )
                                                        .collect::<Vec<_>>();
                                                    let edit_book_window = edit_book_window.unwrap();
                                                    edit_book_window
                                                        .set_series(ModelRc::new(VecModel::from(series)));
                                                    edit_book_window
                                                        .set_series_ids(ModelRc::new(VecModel::from(series_ids)));
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        })
                                            .unwrap();
                                    }
                                });

                                edit_book_window.on_remove_series({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, series_id| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let series = book.series.iter().filter(|series| series.id != series_id).collect::<Vec<_>>();
                                        book.series = ModelRc::new(VecModel::from(series));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_add_series({
                                    let edit_book_window = edit_book_window.as_weak();
                                    move |mut book, series_id, series_name, series_column| {
                                        let edit_book_window = edit_book_window.unwrap();
                                        let isbn = edit_book_window.get_isbn();
                                        let title = edit_book_window.get_book_title();
                                        let price = edit_book_window.get_price();
                                        let catalog = edit_book_window.get_catalog();
                                        let cover = edit_book_window.get_cover();
                                        let on_store = edit_book_window.get_on_store();
                                        let series = book.series.iter().chain(VecModel::from(vec![SeriesInEdit {
                                            id: series_id,
                                            name: series_name.into(),
                                            column: series_column,
                                        }]).iter().filter(|series| !book.series.iter().any(|s| s.id == series.id) && !series.name.is_empty())).collect::<Vec<_>>();
                                        book.series = ModelRc::new(VecModel::from(series));
                                        book.isbn = isbn;
                                        book.title = title;
                                        book.price = price;
                                        book.catalog = catalog;
                                        book.cover = cover;
                                        book.on_store = on_store;
                                        edit_book_window.set_book(book);
                                    }
                                });

                                edit_book_window.on_edit_book({
                                    let rt = rt.as_weak();
                                    let admin_token = admin_token_weak.clone();
                                    let edit_book_window = edit_book_window.as_weak();
                                    let book_detail_window = book_detail_window.clone();
                                    move |book| {
                                        let rt = rt.clone();
                                        let admin_token = admin_token.clone();
                                        let edit_book_window = edit_book_window.unwrap();
                                        let book_detail_window = book_detail_window.clone();

                                        slint::spawn_local(async move {
                                            let rt = rt.unwrap();
                                            let admin_token = admin_token.unwrap();
                                            let book_id = book.id as u32;
                                            let isbn = edit_book_window.get_isbn();
                                            let title = edit_book_window.get_book_title();
                                            let authors = book.authors.iter().map(|author| author.id as u32).collect::<Vec<_>>();
                                            let publisher = book.publisher.id as u32;
                                            let suppliers = book.suppliers.iter().map(|supplier| supplier.id as u32).collect::<Vec<_>>();
                                            let series = book.series.iter().map(|series| (
                                                series.id as u32,
                                                series.column as u32,
                                            )).collect::<Vec<_>>();
                                            let keywords = book.keywords.iter().map(|keyword| keyword.id as u32).collect::<Vec<_>>();
                                            let price = edit_book_window.get_price();
                                            let catalog = edit_book_window.get_catalog();
                                            let cover = edit_book_window.get_cover();
                                            let on_store = edit_book_window.get_on_store();
                                            match rt
                                                .spawn(async move {
                                                    match admin_token.get().await {
                                                        Some(token) => {
                                                            match url_post::<BookUpdateResponse, BookUpdateRequest>(
                                                                "/admin/book/update",
                                                                BookUpdateRequest {
                                                                    token: token.token,
                                                                    tag: token.tag,
                                                                    nonce: token.nonce,
                                                                    book_id,
                                                                    isbn: isbn.to_string(),
                                                                    title: title.to_string(),
                                                                    authors,
                                                                    publisher,
                                                                    suppliers,
                                                                    series,
                                                                    price: price.to_string(),
                                                                    keywords,
                                                                    catalog: catalog.to_string(),
                                                                    cover: cover.to_string(),
                                                                    is_onstore: on_store,
                                                                },
                                                            )
                                                                .await
                                                            {
                                                                Ok(_) => Ok(()),
                                                                Err(e) => Err(e),
                                                            }
                                                        }
                                                        None => Err(anyhow::anyhow!("token not found")),
                                                    }
                                                })
                                                .await
                                                .unwrap()
                                            {
                                                Ok(_) => {
                                                    let book_detail_window = book_detail_window.unwrap();
                                                    book_detail_window.invoke_get_book_detail();
                                                }
                                                Err(e) => {
                                                    log::error!("{}", e.to_string());
                                                }
                                            }
                                        })
                                            .unwrap();
                                    }
                                });

                                edit_book_window.set_book(book_detail);
                                edit_book_window.show().unwrap();
                            }
                            Err(e) => {
                                log::error!("{}", e.to_string());
                            }
                        }
                    }).unwrap();
                }
            });


            book_detail_window.invoke_get_book_detail();
            book_detail_window.show().unwrap();
        }
    });

    main_window.on_add_book({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        move || {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let admin_token_weak = admin_token.clone();
            slint::spawn_local(async move {
                let add_book_window = BookAddWindow::new().unwrap();

                add_book_window.on_get_author_list({
                    let add_book_window = add_book_window.as_weak();
                    let rt = rt.clone();
                    move || {
                        let rt = rt.clone();
                        let add_book_window = add_book_window.clone();
                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            match rt
                                .spawn(async move {
                                    match url_post::<AuthorListResponse, AuthorListRequest>(
                                        "/book/author/list",
                                        Default::default(),
                                    )
                                    .await
                                    {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(author_list) => {
                                    let author_ids = author_list
                                        .authors
                                        .iter()
                                        .map(|author| author.author_id as i32)
                                        .collect::<Vec<_>>();
                                    let authors = author_list
                                        .authors
                                        .into_iter()
                                        .map(|author| author.name.into())
                                        .collect::<Vec<_>>();
                                    let add_book_window = add_book_window.unwrap();
                                    add_book_window
                                        .set_authors(ModelRc::new(VecModel::from(authors)));
                                    add_book_window
                                        .set_author_ids(ModelRc::new(VecModel::from(author_ids)));
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });

                add_book_window.on_remove_author({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, author_id| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let authors = book
                            .authors
                            .iter()
                            .filter(|author| author.id != author_id)
                            .collect::<Vec<_>>();
                        book.authors = ModelRc::new(VecModel::from(authors));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_add_author({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, author_id, author_name| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let authors = book
                            .authors
                            .iter()
                            .chain(
                                VecModel::from(vec![AuthorInAdd {
                                    id: author_id,
                                    name: author_name.into(),
                                }])
                                .iter()
                                .filter(|author| {
                                    !book.authors.iter().any(|a| a.id == author.id)
                                        && !author.name.is_empty()
                                }),
                            )
                            .collect::<Vec<_>>();
                        book.authors = ModelRc::new(VecModel::from(authors));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_get_keyword_list({
                    let add_book_window = add_book_window.as_weak();
                    let rt = rt.clone();
                    move || {
                        let rt = rt.clone();
                        let add_book_window = add_book_window.clone();
                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            match rt
                                .spawn(async move {
                                    match url_post::<KeywordListResponse, KeywordListRequest>(
                                        "/book/keyword/list",
                                        Default::default(),
                                    )
                                    .await
                                    {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(keyword_list) => {
                                    let keyword_ids = keyword_list
                                        .keywords
                                        .iter()
                                        .map(|keyword| keyword.keyword_id as i32)
                                        .collect::<Vec<_>>();
                                    let keywords = keyword_list
                                        .keywords
                                        .into_iter()
                                        .map(|keyword| keyword.keyword.into())
                                        .collect::<Vec<_>>();
                                    let add_book_window = add_book_window.unwrap();
                                    add_book_window
                                        .set_keywords(ModelRc::new(VecModel::from(keywords)));
                                    add_book_window
                                        .set_keyword_ids(ModelRc::new(VecModel::from(keyword_ids)));
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });

                add_book_window.on_remove_keyword({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, keyword_id| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let keywords = book
                            .keywords
                            .iter()
                            .filter(|keyword| keyword.id != keyword_id)
                            .collect::<Vec<_>>();
                        book.keywords = ModelRc::new(VecModel::from(keywords));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_add_keyword({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, keyword_id, keyword_name| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let keywords = book
                            .keywords
                            .iter()
                            .chain(
                                VecModel::from(vec![KeywordInAdd {
                                    id: keyword_id,
                                    keyword: keyword_name.into(),
                                }])
                                .iter()
                                .filter(|keyword| {
                                    !book.keywords.iter().any(|k| k.id == keyword.id)
                                        && !keyword.keyword.is_empty()
                                }),
                            )
                            .collect::<Vec<_>>();
                        book.keywords = ModelRc::new(VecModel::from(keywords));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_get_publisher_list({
                    let add_book_window = add_book_window.as_weak();
                    let rt = rt.clone();
                    move || {
                        let rt = rt.clone();
                        let add_book_window = add_book_window.clone();
                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            match rt
                                .spawn(async move {
                                    match url_post::<PublisherListResponse, PublisherListRequest>(
                                        "/book/publisher/list",
                                        Default::default(),
                                    )
                                    .await
                                    {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(publisher_list) => {
                                    let publisher_ids = publisher_list
                                        .publishers
                                        .iter()
                                        .map(|publisher| publisher.publisher_id as i32)
                                        .collect::<Vec<_>>();
                                    let publishers = publisher_list
                                        .publishers
                                        .into_iter()
                                        .map(|publisher| publisher.name.into())
                                        .collect::<Vec<_>>();
                                    let add_book_window = add_book_window.unwrap();
                                    add_book_window
                                        .set_publishers(ModelRc::new(VecModel::from(publishers)));
                                    add_book_window.set_publisher_ids(ModelRc::new(
                                        VecModel::from(publisher_ids),
                                    ));
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });

                add_book_window.on_change_publisher({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, publisher_id, publisher_name| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let publisher = PublisherInAdd {
                            id: publisher_id,
                            name: publisher_name.into(),
                        };
                        book.publisher = publisher;
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_get_supplier_list({
                    let add_book_window = add_book_window.as_weak();
                    let rt = rt.clone();
                    move || {
                        let rt = rt.clone();
                        let add_book_window = add_book_window.clone();
                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            match rt
                                .spawn(async move {
                                    match url_post::<SupplierListResponse, SupplierListRequest>(
                                        "/supplier/list",
                                        Default::default(),
                                    )
                                    .await
                                    {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(supplier_list) => {
                                    let supplier_ids = supplier_list
                                        .suppliers
                                        .iter()
                                        .map(|supplier| supplier.supplier_id as i32)
                                        .collect::<Vec<_>>();
                                    let suppliers = supplier_list
                                        .suppliers
                                        .into_iter()
                                        .map(|supplier| supplier.name.into())
                                        .collect::<Vec<_>>();
                                    let add_book_window = add_book_window.unwrap();
                                    add_book_window
                                        .set_suppliers(ModelRc::new(VecModel::from(suppliers)));
                                    add_book_window.set_supplier_ids(ModelRc::new(VecModel::from(
                                        supplier_ids,
                                    )));
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });

                add_book_window.on_remove_supplier({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, supplier_id| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let suppliers = book
                            .suppliers
                            .iter()
                            .filter(|supplier| supplier.id != supplier_id)
                            .collect::<Vec<_>>();
                        book.suppliers = ModelRc::new(VecModel::from(suppliers));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_add_supplier({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, supplier_id, supplier_name| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let suppliers = book
                            .suppliers
                            .iter()
                            .chain(
                                VecModel::from(vec![SupplierInAdd {
                                    id: supplier_id,
                                    name: supplier_name.into(),
                                }])
                                .iter()
                                .filter(|supplier| {
                                    !book.suppliers.iter().any(|s| s.id == supplier.id)
                                        && !supplier.name.is_empty()
                                }),
                            )
                            .collect::<Vec<_>>();
                        book.suppliers = ModelRc::new(VecModel::from(suppliers));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_get_series_list({
                    let add_book_window = add_book_window.as_weak();
                    let rt = rt.clone();
                    move || {
                        let rt = rt.clone();
                        let add_book_window = add_book_window.clone();
                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            match rt
                                .spawn(async move {
                                    match url_post::<SeriesListResponse, SeriesListRequest>(
                                        "/book/series/list",
                                        Default::default(),
                                    )
                                    .await
                                    {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(series_list) => {
                                    let series_ids = series_list
                                        .series
                                        .iter()
                                        .map(|series| series.series_id as i32)
                                        .collect::<Vec<_>>();
                                    let series = series_list
                                        .series
                                        .into_iter()
                                        .map(|series| series.name.into())
                                        .collect::<Vec<_>>();
                                    let add_book_window = add_book_window.unwrap();
                                    add_book_window
                                        .set_series(ModelRc::new(VecModel::from(series)));
                                    add_book_window
                                        .set_series_ids(ModelRc::new(VecModel::from(series_ids)));
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });

                add_book_window.on_remove_series({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, series_id| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let series = book
                            .series
                            .iter()
                            .filter(|series| series.id != series_id)
                            .collect::<Vec<_>>();
                        book.series = ModelRc::new(VecModel::from(series));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_add_series({
                    let add_book_window = add_book_window.as_weak();
                    move |mut book, series_id, series_name, series_column| {
                        let add_book_window = add_book_window.unwrap();
                        let isbn = add_book_window.get_isbn();
                        let title = add_book_window.get_book_title();
                        let price = add_book_window.get_price();
                        let catalog = add_book_window.get_catalog();
                        let cover = add_book_window.get_cover();
                        let on_store = add_book_window.get_on_store();
                        let series = book
                            .series
                            .iter()
                            .chain(
                                VecModel::from(vec![SeriesInAdd {
                                    id: series_id,
                                    name: series_name.into(),
                                    column: series_column,
                                }])
                                .iter()
                                .filter(|series| {
                                    !book.series.iter().any(|s| s.id == series.id)
                                        && !series.name.is_empty()
                                }),
                            )
                            .collect::<Vec<_>>();
                        book.series = ModelRc::new(VecModel::from(series));
                        book.isbn = isbn;
                        book.title = title;
                        book.price = price;
                        book.catalog = catalog;
                        book.cover = cover;
                        book.on_store = on_store;
                        add_book_window.set_book(book);
                    }
                });

                add_book_window.on_add_book({
                    let rt = rt.clone();
                    let admin_token = admin_token_weak.clone();
                    let add_book_window = add_book_window.as_weak();
                    let add_book_window_weak = add_book_window.clone();
                    move |book| {
                        let rt = rt.clone();
                        let admin_token = admin_token.clone();
                        let add_book_window = add_book_window.unwrap();
                        let add_book_window_weak = add_book_window_weak.clone();

                        slint::spawn_local(async move {
                            let rt = rt.unwrap();
                            let admin_token = admin_token.unwrap();
                            let isbn = add_book_window.get_isbn();
                            let title = add_book_window.get_book_title();
                            let authors = book
                                .authors
                                .iter()
                                .map(|author| author.id as u32)
                                .collect::<Vec<_>>();
                            let publisher = book.publisher.id as u32;
                            let suppliers = book
                                .suppliers
                                .iter()
                                .map(|supplier| supplier.id as u32)
                                .collect::<Vec<_>>();
                            let series = book
                                .series
                                .iter()
                                .map(|series| (series.id as u32, series.column as u32))
                                .collect::<Vec<_>>();
                            let keywords = book
                                .keywords
                                .iter()
                                .map(|keyword| keyword.id as u32)
                                .collect::<Vec<_>>();
                            let price = add_book_window.get_price();
                            let catalog = add_book_window.get_catalog();
                            let cover = add_book_window.get_cover();
                            let on_store = add_book_window.get_on_store();
                            match rt
                                .spawn(async move {
                                    match admin_token.get().await {
                                        Some(token) => {
                                            match url_post::<BookAddResponse, BookAddRequest>(
                                                "/admin/book/add",
                                                BookAddRequest {
                                                    token: token.token,
                                                    tag: token.tag,
                                                    nonce: token.nonce,
                                                    isbn: isbn.to_string(),
                                                    title: title.to_string(),
                                                    authors,
                                                    publisher,
                                                    suppliers,
                                                    series,
                                                    price: price.to_string(),
                                                    keywords,
                                                    catalog: catalog.to_string(),
                                                    cover: cover.to_string(),
                                                    is_onstore: on_store,
                                                },
                                            )
                                            .await
                                            {
                                                Ok(message) => Ok(message),
                                                Err(e) => Err(e),
                                            }
                                        }
                                        None => Err(anyhow::anyhow!("token not found")),
                                    }
                                })
                                .await
                                .unwrap()
                            {
                                Ok(message) => {
                                    let add_book_window = add_book_window_weak.unwrap();
                                    add_book_window.set_add_message(message.message.into());
                                }
                                Err(e) => {
                                    log::error!("{}", e.to_string());
                                }
                            }
                        })
                        .unwrap();
                    }
                });
                add_book_window.show().unwrap();
            })
            .unwrap();
        }
    });

    main_window.on_get_customers_list({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        let main_window = main_window.as_weak();
        move || {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<CustomerListResponse, CustomerListRequest>(
                                    "/admin/customer/list",
                                    CustomerListRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(customer_list) => {
                        let customers = customer_list
                            .customers
                            .into_iter()
                            .map(|customer| CustomerInDetail {
                                account_banlance: customer.balance.into(),
                                id: customer.user_id as i32,
                                realname: customer.name.into(),
                                email: customer.email.into(),
                                address: customer.address.into(),
                                credit_level: customer.credit_level as i32,
                                username: customer.username.into(),
                            })
                            .collect::<Vec<_>>();
                        let main_window = main_window.unwrap();
                        main_window.set_customers(ModelRc::new(VecModel::from(customers)));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_edit_balance({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        let main_window = main_window.as_weak();
        move |customer_id, balance| {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<CustomerBalanceResponse, CustomerBalanceRequest>(
                                    "/admin/customer/balance",
                                    CustomerBalanceRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                        user_id: customer_id as u32,
                                        balance: balance.to_string(),
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.invoke_get_customers_list();
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_edit_credit({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        let main_window = main_window.as_weak();
        move |customer_id, credit| {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<CustomerCreditResponse, CustomerCreditRequest>(
                                    "/admin/customer/credit",
                                    CustomerCreditRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                        user_id: customer_id as u32,
                                        credit_level: credit as u32,
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.invoke_get_customers_list();
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_customer_orders_list({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        let main_window = main_window.as_weak();
        move || {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<CustomerOrderListResponse, CustomerOrderListRequest>(
                                    "/admin/order/list",
                                    CustomerOrderListRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(order_list) => {
                        let orders = order_list
                            .orders
                            .into_iter()
                            .map(|order| CustomerOrder {
                                customer_id: order.user_id as i32,
                                id: order.order_id as i32,
                                items: ModelRc::new(VecModel::from(order.items.into_iter().map(|item| BookInOrder {
                                    id: item.book_id as i32,
                                    quantity: item.quantity as i32,
                                    price: item.price.into(),
                                }).collect::<Vec<_>>())),
                                order_date: order.date.into(),
                                original_price: order.original_amount.into(),
                                payment_status: order.payment_status.into(),
                                shipping_status: order.shipping_status.into(),
                                total_price: order.total_amount.into(),
                            })
                            .collect::<Vec<_>>();
                        let main_window = main_window.unwrap();
                        main_window.set_customer_orders(ModelRc::new(VecModel::from(orders)));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_send_goods({
        let rt = rt.as_weak();
        let admin_token = admin_token.as_weak();
        let main_window = main_window.as_weak();
        move |order_id| {
            let rt = rt.clone();
            let admin_token = admin_token.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let admin_token = admin_token.unwrap();
                match rt
                    .spawn(async move {
                        match admin_token.get().await {
                            Some(token) => {
                                match url_post::<ShipOrderAutoResponse, ShipOrderAutoRequest>(
                                    "/admin/order/ship/auto",
                                    ShipOrderAutoRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                        order_id: order_id as u32,
                                    },
                                )
                                .await
                                {
                                    Ok(response) => Ok(response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => Err(anyhow::anyhow!("token not found")),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.invoke_get_customer_orders_list();
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.show()?;

    slint::run_event_loop()?;
    Ok(())
}
