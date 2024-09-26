# powerpod-api
Api to manage kubernetes resources in order to limit overconsumption

## Motivation

Rewrite an existing application with different technologies in order to discover and improve

## Tech

For the moment those will be the technologies use. It can change throught time: 

- [actix](https://actix.rs/) : web framework
- [utoipa](https://github.com/juhaku/utoipa) : open api generation and docs ([poc](https://github.com/hugoponthieu/poc-actix-scalar))
- [seaORM](https://www.sea-ql.org/SeaORM/) : Orm based on sqlx

## Used documentation

Some subject are not completely coverd by documentations. Therefore here are some of my reseaches:

- (authentication)[https://herewecode.io/fr/blog/creer-middleware-authentification-actix-rust/]
- (generating tokens)[https://codevoweb.com/rust-and-axum-jwt-access-and-refresh-tokens/]

## Some more

I considered at one point using [casbin-rs](https://github.com/casbin/casbin-rs) to manage authorization but the case I have to treat will be fine with just a n*n relationship. I am open to discuss that. 