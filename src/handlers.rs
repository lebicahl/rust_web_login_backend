use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST, verify};
use crate::{models::NewUser, schema::users, db::DbPool};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn signup(
    pool: web::Data<DbPool>,
    form: web::Form<SignupForm>,
) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // Hash the password
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

    let new_user = NewUser {
        username: form.username.clone(),
        password_hash: hashed_password,
    };

    let inserted = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn);

    match inserted {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}


pub async fn login_user(pool: web::Data<DbPool>, form: web::Form<LoginForm>) -> HttpResponse {
    use crate::schema::users::dsl::{users, username};
    use crate::models::User;

    let conn = &mut pool.get().expect("Couldn't get DB connection");

    let user_result = users
        .filter(username.eq(&form.username))
        .first::<User>(conn);

    match user_result {
        Ok(user) => {
            if verify(&form.password, &user.password_hash).unwrap() {
                HttpResponse::Ok().body("Login successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}