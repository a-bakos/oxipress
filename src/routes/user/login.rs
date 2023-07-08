use crate::{
    database::{
        columns::{COL_INDEX_ACCOUNT_EMAIL, COL_INDEX_ACCOUNT_LOGIN},
        db::DB_Table,
    },
    errors::error_handler::ErrorResponse,
    http::status_code::HttpStatusCode,
    users::{
        credentials,
        credentials::{find_account_by_identifier, AccountIdentifier},
    },
};
use chrono::NaiveDateTime;

use crate::database::columns::{
    COL_INDEX_ACCOUNT_ID, COL_INDEX_ACCOUNT_REGISTERED, COL_INDEX_ACCOUNT_ROLE,
};
use crate::users::roles::get_role_variant;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub async fn try_login(
    pool: PgPool,
    password: String,
    binding: String,
    login_variant: AccountIdentifier,
) -> Result<warp::reply::Json, warp::Rejection> {
    let column_name = match login_variant {
        AccountIdentifier::Email => COL_INDEX_ACCOUNT_EMAIL,
        AccountIdentifier::Login => COL_INDEX_ACCOUNT_LOGIN,
    };
    let query = format!(
        "SELECT * FROM {} WHERE {} = $1 AND password = $2",
        DB_Table::Accounts,
        column_name
    );
    println!("{}", query);
    match sqlx::query(&query)
        .bind(binding.clone())
        .bind(password)
        .map(|row: PgRow| {
            println!("Were in trylogin");
            // Underscores' meaning here:
            // we don't need to specify a default/fallback value because the cell will never be empty
            let id = row.get::<i32, _>(COL_INDEX_ACCOUNT_ID) as u32;
            let role = row.get::<&str, _>(COL_INDEX_ACCOUNT_ROLE);
            LoginResponse {
                id,
                login_name: row.get(COL_INDEX_ACCOUNT_LOGIN),
                email: row.get(COL_INDEX_ACCOUNT_EMAIL),
                role: role.to_string(),
            }
        })
        .fetch_one(&pool)
        .await
    {
        Ok(user) => {
            println!("{:?}", user);

            let update_last_login_query = format!(
                "UPDATE {} SET last_login = CURRENT_TIMESTAMP WHERE {} = $1",
                DB_Table::Accounts,
                column_name // email or username
            );
            match sqlx::query(&update_last_login_query)
                .bind(binding)
                .execute(&pool)
                .await
            {
                Ok(_) => println!("Last login updated!"),
                Err(e) => println!("Last login update error"),
            }

            Ok(warp::reply::json(&LoginResponseWithStatusCode(200, user)))
        }
        Err(e) => Ok(warp::reply::json(&HttpStatusCode::Unauthorized.code())),
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponseWithStatusCode(pub u32, pub LoginResponse);

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub id: u32,
    pub login_name: String,
    pub email: String,
    pub role: String,
}

pub async fn login(
    pool: PgPool,
    params: LoginRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    // if email found, ignore login name
    // if no email, look for login name
    if let Some(email) = params.email {
        let account_exists_by_email =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Email, email.clone()).await;

        return match account_exists_by_email {
            Ok(true) => {
                // Acc exists / go login

                let binding = email.clone();
                let password = params.password.clone();
                if credentials::is_password_match(
                    &pool,
                    &password,
                    AccountIdentifier::Email,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(pool.clone(), password, binding, AccountIdentifier::Email).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);
                    // Client response
                    Ok(warp::reply::json(&HttpStatusCode::Unauthorized.code()))
                }
            }
            Ok(false) => Ok(warp::reply::json(&HttpStatusCode::Unauthorized.code())),
            Err(e) => Ok(warp::reply::json(
                &HttpStatusCode::InternalServerError.code(),
            )),
        };
    }

    if let Some(login) = params.login {
        let account_exists_by_login =
            find_account_by_identifier(pool.clone(), AccountIdentifier::Login, login.clone()).await;

        return match account_exists_by_login {
            Ok(true) => {
                println!("We're here trying to get user");
                // Acc exists

                let binding = login.clone();
                let password = params.password.clone();
                if credentials::is_password_match(
                    &pool,
                    &password,
                    AccountIdentifier::Login,
                    &binding,
                )
                .await
                {
                    // Try login and return result
                    try_login(pool.clone(), password, binding, AccountIdentifier::Login).await
                } else {
                    // System log
                    println!("Wrong password used for: {}", &binding);
                    // Client response
                    Ok(warp::reply::json(&HttpStatusCode::Unauthorized.code()))
                }
            }
            Ok(false) => Ok(warp::reply::json(&HttpStatusCode::Unauthorized.code())),
            Err(e) => Ok(warp::reply::json(
                &HttpStatusCode::InternalServerError.code(),
            )),
        };
    }

    Ok(warp::reply::json(&ErrorResponse::new(
        "Empty login".to_owned(),
    )))
}
