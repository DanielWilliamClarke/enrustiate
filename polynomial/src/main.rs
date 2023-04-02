use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use solver::{Parser, ParserError, Solver};

mod solver;

enum AppError {
    PolynomialError(ParserError),
}

impl From<ParserError> for AppError {
    fn from(inner: ParserError) -> Self {
        AppError::PolynomialError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::PolynomialError(ParserError::EmptyPolynomial) => (
                StatusCode::NOT_ACCEPTABLE,
                "Polynomial is empty!".to_string(),
            ),
            AppError::PolynomialError(ParserError::IsScalar(err)) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("Polynomial is scalar value: {}", err),
            ),
            AppError::PolynomialError(ParserError::InvalidTerm(err)) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("Polynomial has invalid term {}", err),
            ),
            AppError::PolynomialError(ParserError::MissingTerm(err)) => (
                StatusCode::NOT_ACCEPTABLE,
                format!("Polynomial has missing term {}", err),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn solve_polynomial(body: String) -> Result<Json<Vec<i32>>, AppError> {
    let polynomial = Parser::parse(&body)?;

    Ok(Json(Solver::solve(&polynomial)))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/polynomial_solver", post(solve_polynomial));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
