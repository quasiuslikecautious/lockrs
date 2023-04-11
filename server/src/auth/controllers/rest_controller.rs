// use axum::{
//     Json,
//     Router,
//     extract::Path,
//     response::Response,
//     routing::{delete, get, post, put}, async_trait,
// };
//
// #[async_trait]
// pub trait RestController<IdType, CreateRequestType> {
//     async fn create(_: Json<CreateRequestType>) -> Response; 
//     async fn get(_: Path<IdType>) -> Response;
//     async fn update(_: Path<IdType>) -> Response;
//     async fn delete(_: Path<IdType>) -> Response;
//
//     fn into_routes(base: &str) -> Router {
//         let rest_routes = Router::new()
//             .route("/", post(Self::create))
//             .route("/:id", get(Self::get))
//             .route("/:id", put(Self::update))
//             .route("/:id", delete(Self::delete));
//
//         Router::new()
//             .nest(base, rest_routes)
//     }
// }
//
