use bookstore_user::network::*;
use bookstore_user::util::cart::ShoppingCart;
use bookstore_user::util::runtime::Runtime;
use bookstore_user::util::token::{Token, UserToken};
use rust_decimal::Decimal;
use slint::{Image, Model, ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    pretty_env_logger::init();

    let main_window = MainWindow::new()?;

    let rt = Runtime::new().unwrap();

    let user_token = UserToken::default();
    let shopping_cart = ShoppingCart::default();

    main_window.on_user_login({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();

        move |username, password| {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();

            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                match rt
                    .spawn(async move {
                        let login_request = LoginRequest {
                            username: username.to_string(),
                            password: password.to_string(),
                        };

                        match url_post::<LoginResponse, LoginRequest>("/user/login", login_request)
                            .await
                        {
                            Ok(login_response) => {
                                user_token
                                    .set(Token {
                                        token: login_response.token,
                                        tag: login_response.tag,
                                        nonce: login_response.nonce,
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
                    Err(e) => {
                        let main_window = main_window.unwrap();
                        main_window.set_error_login(true);
                        main_window.set_error_login_message(e.to_string().into());
                        log::error!("{}", e.to_string());
                    }
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.set_error_login(false);
                        main_window.set_has_login(true);
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_user_register({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();
        move |username, password| {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                match rt
                    .spawn(async move {
                        let register_request = RegisterRequest {
                            username: username.to_string(),
                            password: password.to_string(),
                            name: username.to_string(),
                        };

                        match url_post::<RegisterResponse, RegisterRequest>(
                            "/user/register",
                            register_request,
                        )
                        .await
                        {
                            Ok(register_response) => {
                                user_token
                                    .set(Token {
                                        token: register_response.token,
                                        tag: register_response.tag,
                                        nonce: register_response.nonce,
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
                    Err(e) => {
                        // let main_window = main_window.unwrap();
                        // main_window.set_error_register(true);
                        // main_window.set_error_register_message(e.to_string().into());
                        log::error!("{}", e.to_string());
                    }
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        // main_window.set_error_register(false);
                        main_window.set_has_login(true);
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_user_detail({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();

        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();

            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                match rt
                    .spawn(async move {
                        match user_token.get().await {
                            Some(token) => match url_post::<UserDetailResponse, UserDetailRequest>(
                                "/user/detail",
                                UserDetailRequest {
                                    token: token.token,
                                    tag: token.tag,
                                    nonce: token.nonce,
                                },
                            )
                            .await
                            {
                                Ok(user_detail) => Ok(user_detail),
                                Err(e) => Err(e),
                            },
                            None => anyhow::bail!("user token not found"),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(user_detail) => {
                        let main_window = main_window.unwrap();
                        let user_detail = UserDetail {
                            account_balance: user_detail.account_balance.into(),
                            username: user_detail.username.into(),
                            email: user_detail.email.into(),
                            name: user_detail.name.into(),
                            overdraft_limit: user_detail.overdraft_limit.into(),
                            address: user_detail.address.into(),
                            credit_level: user_detail.credit_level.to_string().into(),
                            total_purchase: user_detail.total_purchase.into(),
                        };
                        main_window.set_user_detail(user_detail);
                    }

                    Err(e) => {
                        let main_window = main_window.unwrap();
                        main_window.set_has_login(false);
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_user_update({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();
        move |detail| {
            let user_update_window = UserDetailUpdateWindow::new().unwrap();
            user_update_window.on_update_user_detail({
                let rt = rt.clone();
                let main_window = main_window.clone();
                let user_token = user_token.clone();

                move |detail1| {
                    let rt = rt.clone();
                    let main_window = main_window.clone();
                    let user_token = user_token.clone();
                    slint::spawn_local(async move {
                        let rt = rt.unwrap();
                        let user_token = user_token.unwrap();
                        match rt
                            .spawn(async move {
                                match user_token.get().await {
                                    Some(token) => {
                                        match url_post::<UserUpdateResponse, UserUpdateRequest>(
                                            "/user/update",
                                            UserUpdateRequest {
                                                token: token.token,
                                                tag: token.tag,
                                                nonce: token.nonce,
                                                username: detail1.username.to_string(),
                                                name: detail1.name.to_string(),
                                                email: detail1.email.to_string(),
                                                address: detail1.address.to_string(),
                                            },
                                        )
                                        .await
                                        {
                                            Ok(user_update_response) => {
                                                user_token
                                                    .set(Token {
                                                        token: user_update_response.token,
                                                        tag: user_update_response.tag,
                                                        nonce: user_update_response.nonce,
                                                    })
                                                    .await;
                                                Ok(())
                                            }
                                            Err(e) => Err(e),
                                        }
                                    }
                                    None => anyhow::bail!("user token not found"),
                                }
                            })
                            .await
                            .unwrap()
                        {
                            Ok(_) => {
                                let main_window = main_window.unwrap();
                                main_window.invoke_get_user_detail();
                            }
                            Err(e) => {
                                log::error!("{}", e.to_string());
                            }
                        }
                    })
                    .unwrap();
                }
            });

            user_update_window.set_user_detail(detail);

            user_update_window.show().unwrap();
        }
    });

    main_window.on_user_logout({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();

        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();

            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                match rt
                    .spawn(async move {
                        match user_token.get().await {
                            Some(token) => {
                                match url_post::<LogoutResponse, LogoutRequest>(
                                    "/user/logout",
                                    LogoutRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                    },
                                )
                                .await
                                {
                                    Ok(_) => {
                                        user_token.clear().await;
                                        Ok(())
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            None => anyhow::bail!("user token not found"),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window.unwrap();
                        main_window.set_has_login(false);
                    }
                    Err(e) => {
                        let main_window = main_window.unwrap();
                        main_window.set_error_logout(true);
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

    main_window.on_add_to_cart({
        let rt = rt.as_weak();
        let shopping_cart = shopping_cart.as_weak();
        move |book_id, quantity| {
            let rt = rt.clone();
            let shopping_cart = shopping_cart.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let shopping_cart = shopping_cart.unwrap();
                rt.spawn(async move {
                    shopping_cart
                        .add_item(book_id as u32, quantity as u32)
                        .await;
                })
                .await
                .unwrap()
            })
            .unwrap();
        }
    });

    main_window.on_get_cart_book_list({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let shopping_cart = shopping_cart.as_weak();
        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let shopping_cart = shopping_cart.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let shopping_cart = shopping_cart.unwrap();
                match rt
                    .spawn(async move {
                        let cart_items = shopping_cart.get_total_items().await;
                        let mut books_in_cart = Vec::with_capacity(cart_items.len());
                        let mut book_cover_buffers = Vec::with_capacity(cart_items.len());
                        for (book_id, quantity) in cart_items.iter() {
                            match url_post::<BookDetailResponse, BookDetailRequest>(
                                &format!("book/{}/detail", book_id),
                                BookDetailRequest::default(),
                            )
                            .await
                            {
                                Ok(book_detail) => {
                                    book_cover_buffers
                                        .push(url_get_image_buffer(&book_detail.cover).await.ok());
                                    books_in_cart.push((book_detail, *quantity));
                                }
                                Err(e) => anyhow::bail!(e),
                            }
                        }
                        Ok((books_in_cart, book_cover_buffers))
                    })
                    .await
                    .unwrap()
                {
                    Ok((cart_items, book_cover_buffers)) => {
                        let books_in_cart = cart_items
                            .into_iter()
                            .zip(book_cover_buffers.into_iter())
                            .map(|((book, quantity), cover)| {
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
                                let price_dec = Decimal::from_str_exact(&book.price).unwrap();
                                let price = book.price.into();

                                let cover = match cover {
                                    Some(buffer) => Image::from_rgba8(buffer),
                                    None => Image::default(),
                                };
                                BookInCart {
                                    id: book_id,
                                    title,
                                    isbn,
                                    authors,
                                    publisher,
                                    price,
                                    cover,
                                    quantity: quantity as i32,
                                    total: (price_dec * Decimal::from(quantity)).to_string().into(),
                                }
                            })
                            .collect::<Vec<_>>();
                        let main_window = main_window.unwrap();
                        main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
                    }
                    Err(e) => {
                        // let main_window = main_window.unwrap();
                        // main_window.set_error_add_to_cart(true);
                        // main_window.set_error_add_to_cart_message(e.to_string().into());
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_remove_cart_book({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let shopping_cart = shopping_cart.as_weak();
        move |book_id, books_in_cart| {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let shopping_cart = shopping_cart.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let shopping_cart = shopping_cart.unwrap();
                rt.spawn(async move { shopping_cart.remove_item(book_id as u32).await })
                    .await
                    .unwrap();

                let books_in_cart = books_in_cart
                    .iter()
                    .filter(|book| book.id != book_id)
                    .collect::<Vec<_>>();

                let main_window = main_window.unwrap();
                main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
            })
            .unwrap();
        }
    });

    main_window.on_update_cart_book({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let shopping_cart = shopping_cart.as_weak();
        move |book_id, quantity, books_in_cart| {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let shopping_cart = shopping_cart.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let shopping_cart = shopping_cart.unwrap();
                rt.spawn(async move {
                    shopping_cart
                        .set_item(book_id as u32, quantity as u32)
                        .await;
                })
                .await
                .unwrap();

                let books_in_cart = books_in_cart
                    .iter()
                    .into_iter()
                    .map(|book| {
                        if book.id == book_id {
                            let price_dec = Decimal::from_str_exact(&book.price).unwrap();
                            BookInCart {
                                id: book.id,
                                title: book.title,
                                isbn: book.isbn,
                                authors: book.authors,
                                publisher: book.publisher,
                                price: book.price,
                                cover: book.cover,
                                quantity,
                                total: (price_dec * Decimal::from(quantity)).to_string().into(),
                            }
                        } else {
                            book
                        }
                    })
                    .collect::<Vec<_>>();
                let main_window = main_window.unwrap();
                main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
            })
            .unwrap();
        }
    });

    main_window.on_checkout({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();
        let shopping_cart = shopping_cart.as_weak();
        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();
            let shopping_cart = shopping_cart.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                let shopping_cart = shopping_cart.unwrap();
                match rt
                    .spawn(async move {
                        match user_token.get().await {
                            Some(token) => {
                                match url_post::<OrderCreateResponse, OrderCreateRequest>(
                                    "/order/create",
                                    OrderCreateRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                        items: shopping_cart
                                            .get_total_items()
                                            .await
                                            .into_iter()
                                            .map(|(book_id, quantity)| (book_id, quantity))
                                            .collect(),
                                    },
                                )
                                .await
                                {
                                    Ok(order_response) => {
                                        shopping_cart.clear().await;
                                        Ok(order_response.order_id)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            None => anyhow::bail!("user token not found"),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(order_id) => {
                        log::info!("user order {} create", order_id);
                        let main_window = main_window.unwrap();
                        main_window.set_books_in_cart(ModelRc::new(VecModel::from(Vec::new())));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_order_history_list({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        let user_token = user_token.as_weak();
        move || {
            let rt = rt.clone();
            let main_window = main_window.clone();
            let user_token = user_token.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                let user_token = user_token.unwrap();
                match rt
                    .spawn(async move {
                        match user_token.get().await {
                            Some(token) => {
                                match url_post::<OrderHistoryResponse, OrderHistoryRequest>(
                                    "/order/history",
                                    OrderHistoryRequest {
                                        token: token.token,
                                        tag: token.tag,
                                        nonce: token.nonce,
                                    },
                                )
                                .await
                                {
                                    Ok(order_history_response) => Ok(order_history_response),
                                    Err(e) => Err(e),
                                }
                            }
                            None => anyhow::bail!("user token not found"),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(order_history) => {
                        let orders = order_history
                            .orders
                            .into_iter()
                            .map(
                                |OrderHistoryItemResponse {
                                     order_id,
                                     discount_percentage,
                                     discount_amount,
                                     original_price,
                                     total_price,
                                     order_date,
                                     payment_status,
                                     shipping_status,
                                 }| {
                                    OrderHistory {
                                        id: order_id as i32,
                                        discount_percentage: discount_percentage.into(),
                                        discount_amount: discount_amount.into(),
                                        original_price: original_price.into(),
                                        total_price: total_price.into(),
                                        order_date: order_date.into(),
                                        payment_status: payment_status.into(),
                                        shipping_status: shipping_status.into(),
                                    }
                                },
                            )
                            .collect::<Vec<_>>();
                        let main_window = main_window.unwrap();
                        main_window.set_orders(ModelRc::new(VecModel::from(orders)));
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
        move |book_id| {
            let rt = rt.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                match rt
                    .spawn(async move {
                        match url_post::<BookDetailResponse, BookDetailRequest>(
                            &format!("book/{}/detail", book_id),
                            BookDetailRequest::default(),
                        )
                        .await
                        {
                            Ok(book_detail) => {
                                let cover = url_get_image_buffer(&book_detail.cover).await.ok();
                                Ok((book_detail, cover))
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok((book_detail, cover)) => {
                        let book = BookInDetail {
                            id: book_detail.book_id as i32,
                            title: book_detail.title.into(),
                            isbn: book_detail.isbn.into(),
                            authors: ModelRc::new(VecModel::from(
                                book_detail
                                    .authors
                                    .into_iter()
                                    .map(|author| author.name.into())
                                    .collect::<Vec<_>>(),
                            )),
                            publisher: book_detail.publisher.name.into(),
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
                            catalog: book_detail.catalog.into(),
                            cover: match cover {
                                Some(buffer) => Image::from_rgba8(buffer),
                                None => Image::default(),
                            },
                            suppliers: ModelRc::new(VecModel::from(
                                book_detail
                                    .suppliers
                                    .into_iter()
                                    .map(|supplier| supplier.name.into())
                                    .collect::<Vec<_>>(),
                            )),
                        };
                        let book_detail_window = BookDetailWindow::new().unwrap();
                        book_detail_window.set_book(book);
                        book_detail_window.show().unwrap();
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_search_book_list({
        let rt = rt.as_weak();
        let main_window = main_window.as_weak();
        move |search_text, mode| {
            let rt = rt.clone();
            let main_window = main_window.clone();
            slint::spawn_local(async move {
                let rt = rt.unwrap();
                match rt
                    .spawn(async move {
                        match mode {
                            0 => match url_post::<BookTitleSearchResponse, BookTitleSearchRequest>(
                                "book/search/title",
                                BookTitleSearchRequest {
                                    title: search_text.to_string(),
                                },
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
                            },
                            1 => match url_post::<BookAuthorsSearchResponse, BookAuthorsSearchRequest>(
                                "book/search/authors",
                                BookAuthorsSearchRequest {
                                    authors: search_text.to_string(),
                                },
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
                                    Ok((book_list.into(), book_cover_buffers))
                                }
                                Err(e) => Err(e),
                            },
                            2 => {
                                match url_post::<
                                    BookKeywordsSearchResponse,
                                    BookKeywordsSearchRequest,
                                >(
                                    "book/search/keywords",
                                    BookKeywordsSearchRequest {
                                        keywords: search_text.trim().to_string(),
                                    },
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
                                        Ok((book_list.into(), book_cover_buffers))
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            _ => unreachable!("mode out of range"),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok((search_items, cover)) => {
                        let books = search_items
                            .books
                            .into_iter()
                            .zip(cover.into_iter())
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
                                BookInSearch {
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
                        main_window.set_books_in_search(ModelRc::new(VecModel::from(books)));
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
