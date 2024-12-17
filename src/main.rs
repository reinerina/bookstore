use bookstore_user::global;
use bookstore_user::global::Token;
use bookstore_user::network::{
    url_get_image_buffer, url_post, BookDetailRequest, BookDetailResponse, BookListRequest,
    BookListResponse, LoginRequest, LoginResponse, LogoutRequest, LogoutResponse,
    OrderCreateRequest, OrderCreateResponse, OrderHistoryItemResponse, OrderHistoryRequest,
    OrderHistoryResponse, UserDetailRequest, UserDetailResponse,
};
use rust_decimal::Decimal;
use slint::{Image, Model, ModelRc, VecModel};
use std::mem::take;
use std::sync::Arc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    pretty_env_logger::init();

    let main_window = MainWindow::new()?;

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    main_window.on_user_login({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();

        move |username, password| {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();

            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        let login_request = LoginRequest {
                            username: username.to_string(),
                            password: password.to_string(),
                        };

                        match url_post::<LoginResponse, LoginRequest>("/user/login", login_request)
                            .await
                        {
                            Ok(mut login_response) => {
                                global::set_user_token(Token {
                                    token: take(&mut login_response.token),
                                    tag: take(&mut login_response.tag),
                                    nonce: take(&mut login_response.nonce),
                                })
                                .await
                                .unwrap();
                                Ok(())
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Err(e) => {
                        let main_window = main_window_weak.unwrap();
                        main_window.set_error_login(true);
                        main_window.set_error_login_message(e.to_string().into());
                        log::error!("{}", e.to_string());
                    }
                    Ok(_) => {
                        let main_window = main_window_weak.unwrap();
                        main_window.set_error_login(false);
                        main_window.set_has_login(true);
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_user_detail({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();

        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();

            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        match global::get_user_token().await {
                            Ok(token) => {
                                match url_post::<UserDetailResponse, UserDetailRequest>(
                                    "/user/detail",
                                    {
                                        match token.read().await.as_ref() {
                                            Some(token) => UserDetailRequest {
                                                token: token.token.clone(),
                                                tag: token.tag.clone(),
                                                nonce: token.nonce.clone(),
                                            },
                                            None => UserDetailRequest::default(),
                                        }
                                    },
                                )
                                .await
                                {
                                    Ok(user_detail) => Ok(user_detail),
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(user_detail) => {
                        let main_window = main_window_weak.unwrap();
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
                        let main_window = main_window_weak.unwrap();
                        main_window.set_has_login(false);
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_user_logout({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();

        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();

            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        match global::get_user_token().await {
                            Ok(token) => {
                                let token_request = match token.read().await.as_ref() {
                                    Some(token) => LogoutRequest {
                                        token: token.token.clone(),
                                        tag: token.tag.clone(),
                                        nonce: token.nonce.clone(),
                                    },
                                    None => LogoutRequest::default(),
                                };
                                match url_post::<LogoutResponse, LogoutRequest>(
                                    "/user/logout",
                                    token_request,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        global::clear_user_token().await?;
                                        Ok(())
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let main_window = main_window_weak.unwrap();
                        main_window.set_has_login(false);
                    }
                    Err(e) => {
                        let main_window = main_window_weak.unwrap();
                        main_window.set_error_logout(true);
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_book_list({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
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
                                        .map(|keyword| keyword.into())
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

                        let main_window = main_window_weak.unwrap();
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
        let rt_clone = rt.clone();
        move |book_id, quantity| {
            let rt_clone = rt_clone.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        global::add_item_to_cart(book_id as u32, quantity as u32).await
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_get_cart_book_list({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        let cart_items = global::get_cart_items().await?;
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
                        let main_window = main_window_weak.unwrap();
                        main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
                    }
                    Err(e) => {
                        // let main_window = main_window_weak.unwrap();
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
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move |book_id, books_in_cart| {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move { global::remove_item_from_cart(book_id as u32).await })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
                        let books_in_cart = books_in_cart
                            .iter()
                            .filter(|book| book.id != book_id)
                            .collect::<Vec<_>>();

                        let main_window = main_window_weak.unwrap();
                        main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_update_cart_book({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move |book_id, quantity, books_in_cart| {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        global::set_item_to_cart(book_id as u32, quantity as u32).await
                    })
                    .await
                    .unwrap()
                {
                    Ok(_) => {
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
                                        total: (price_dec * Decimal::from(quantity))
                                            .to_string()
                                            .into(),
                                    }
                                } else {
                                    book
                                }
                            })
                            .collect::<Vec<_>>();
                        let main_window = main_window_weak.unwrap();
                        main_window.set_books_in_cart(ModelRc::new(VecModel::from(books_in_cart)));
                    }
                    Err(e) => {
                        log::error!("{}", e.to_string());
                    }
                }
            })
            .unwrap();
        }
    });

    main_window.on_checkout({
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        match global::get_user_token().await {
                            Ok(token) => {
                                let order_create_request = match token.read().await.as_ref() {
                                    Some(token) => match global::get_cart_items().await {
                                        Ok(order_items) => OrderCreateRequest {
                                            token: token.token.clone(),
                                            tag: token.tag.clone(),
                                            nonce: token.nonce.clone(),
                                            items: order_items,
                                        },
                                        Err(e) => anyhow::bail!(e),
                                    },
                                    None => OrderCreateRequest::default(),
                                };
                                match url_post::<OrderCreateResponse, OrderCreateRequest>(
                                    "/order/create",
                                    order_create_request,
                                )
                                .await
                                {
                                    Ok(order_response) => {
                                        global::clear_shopping_cart().await?;
                                        Ok(order_response.order_id)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    })
                    .await
                    .unwrap()
                {
                    Ok(order_id) => {
                        log::info!("user order {} create", order_id);
                        let main_window = main_window_weak.unwrap();
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
        let rt_clone = rt.clone();
        let main_window_weak = main_window.as_weak();
        move || {
            let rt_clone = rt_clone.clone();
            let main_window_weak = main_window_weak.clone();
            slint::spawn_local(async move {
                match rt_clone
                    .spawn(async move {
                        match global::get_user_token().await {
                            Ok(token) => {
                                let order_history_request = match token.read().await.as_ref() {
                                    Some(token) => OrderHistoryRequest {
                                        token: token.token.clone(),
                                        tag: token.tag.clone(),
                                        nonce: token.nonce.clone(),
                                    },
                                    None => OrderHistoryRequest {
                                        token: "".into(),
                                        tag: "".into(),
                                        nonce: "".into(),
                                    },
                                };
                                match url_post::<OrderHistoryResponse, OrderHistoryRequest>(
                                    "/order/history",
                                    order_history_request,
                                )
                                .await
                                {
                                    Ok(order_history_response) => Ok(order_history_response),
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
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
                        let main_window = main_window_weak.unwrap();
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
        let rt_clone = rt.clone();
        move |book_id| {
            let rt_clone = rt_clone.clone();
            slint::spawn_local(async move {
                match rt_clone
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
                        let book = BookDetail {
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
                                    .map(|keyword| keyword.into())
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

    main_window.show()?;

    slint::run_event_loop()?;

    Ok(())
}
