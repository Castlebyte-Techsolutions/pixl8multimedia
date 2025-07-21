#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> pixl8multimedia::error::Result<()> {
    use axum::{Router};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use pixl8multimedia::{app::*};

    dotenv::dotenv().ok();

    // let db_url = &core_config().DB_URL;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // let db_pool = sqlx::postgres::PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(db_url.as_ref())
    //     .await
    //     .expect("Failed to initialize database");

    // let state = AppState {
    //     leptos_options: leptos_options.clone(),
    //     db_pool,
    // };

    let app = Router::new()
        // .route(
        //     "/api/{*fn_name}",
        //     get(server_fn_handler::server_fn_handler).post(server_fn_handler::server_fn_handler),
        // )
        // .leptos_routes_with_context(
        //     &state,
        //     routes,
        //     {
        //         let db_pool = state.db_pool.clone();
        //         move || provide_context(db_pool.clone())
        //     },
        //     {
        //         let state = state.clone();
        //         move || shell(state.leptos_options.clone())
        //     },
        // )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// #[cfg(feature = "ssr")]
// mod server_fn_handler {
//     use axum::{
//         extract::{Request, State},
//         response::IntoResponse,
//     };
//     use leptos::prelude::*;
//     use leptos_axum::handle_server_fns_with_context;
//     use pixl8multimedia::app_state::AppState;

//     pub async fn server_fn_handler(
//         State(app_state): State<AppState>,
//         request: Request,
//     ) -> impl IntoResponse {
//         handle_server_fns_with_context(
//             move || {
//                 provide_context(app_state.db_pool.clone());
//             },
//             request,
//         )
//         .await
//     }
// }

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
