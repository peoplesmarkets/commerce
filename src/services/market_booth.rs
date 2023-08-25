use deadpool_postgres::Pool;
use jwtk::{jwk::RemoteJwksVerifier, Claims};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::commerce::v1::market_booth_service_server::{
    self, MarketBoothServiceServer,
};
use crate::api::peoplesmarkets::commerce::v1::{
    CreateMarketBoothRequest, CreateMarketBoothResponse,
    DeleteMarketBoothRequest, DeleteMarketBoothResponse, GetMarketBoothRequest,
    GetMarketBoothResponse, ListMarketBoothsRequest, ListMarketBoothsResponse,
    UpdateMarketBoothRequest, UpdateMarketBoothResponse,
};
use crate::auth::get_auth_token;
use crate::error::db_err_to_grpc_status;
use crate::model::MarketBooth;
use crate::parse_uuid;

use super::paginate;

pub struct MarketBoothService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
}

impl MarketBoothService {
    fn new(pool: Pool, verifier: RemoteJwksVerifier) -> Self {
        Self { pool, verifier }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
    ) -> MarketBoothServiceServer<Self> {
        MarketBoothServiceServer::new(Self::new(pool, verifier))
    }
}

#[async_trait]
impl market_booth_service_server::MarketBoothService for MarketBoothService {
    async fn create_market_booth(
        &self,
        request: Request<CreateMarketBoothRequest>,
    ) -> Result<Response<CreateMarketBoothResponse>, Status> {
        let token = get_auth_token(request.metadata())
            .ok_or_else(|| Status::unauthenticated(""))?;

        let CreateMarketBoothRequest { name, description } =
            request.into_inner();

        let claims = self
            .verifier
            .verify::<Claims<()>>(&token)
            .await
            .map_err(|err| Status::unauthenticated(err.to_string()))?;

        let user_id = claims
            .claims()
            .sub
            .as_ref()
            .ok_or_else(|| Status::unauthenticated(""))?;

        let created_shop =
            MarketBooth::create(&self.pool, user_id, name, description)
                .await
                .map_err(db_err_to_grpc_status)?;

        Ok(Response::new(CreateMarketBoothResponse {
            market_booth: Some(created_shop.into()),
        }))
    }

    async fn get_market_booth(
        &self,
        request: Request<GetMarketBoothRequest>,
    ) -> Result<Response<GetMarketBoothResponse>, Status> {
        let market_booth_id = parse_uuid(
            request.into_inner().market_booth_id,
            "market_booth_id",
        )?;

        let found_market_booth = MarketBooth::get(&self.pool, &market_booth_id)
            .await
            .map_err(db_err_to_grpc_status)?
            .ok_or(Status::not_found(""))?;

        Ok(Response::new(GetMarketBoothResponse {
            market_booth: Some(found_market_booth.into()),
        }))
    }

    async fn list_market_booths(
        &self,
        request: Request<ListMarketBoothsRequest>,
    ) -> Result<Response<ListMarketBoothsResponse>, Status> {
        let ListMarketBoothsRequest {
            user_id,
            pagination,
            ..
        } = request.into_inner();

        let (limit, offset, pagination) = paginate(pagination);

        let found_market_booths =
            MarketBooth::list(&self.pool, user_id.as_ref(), limit, offset)
                .await
                .map_err(db_err_to_grpc_status)?;

        Ok(Response::new(ListMarketBoothsResponse {
            market_booths: found_market_booths
                .into_iter()
                .map(|mb| mb.into())
                .collect(),
            pagination: Some(pagination),
        }))
    }

    async fn update_market_booth(
        &self,
        request: Request<UpdateMarketBoothRequest>,
    ) -> Result<Response<UpdateMarketBoothResponse>, Status> {
        let UpdateMarketBoothRequest {
            market_booth_id,
            name,
            description,
        } = request.into_inner();

        let updated_market_booth = MarketBooth::update(
            &self.pool,
            &parse_uuid(market_booth_id, "market_booth_id")?,
            name,
            description,
        )
        .await
        .map_err(db_err_to_grpc_status)?;

        Ok(Response::new(UpdateMarketBoothResponse {
            market_booth: Some(updated_market_booth.into()),
        }))
    }

    async fn delete_market_booth(
        &self,
        request: Request<DeleteMarketBoothRequest>,
    ) -> Result<Response<DeleteMarketBoothResponse>, Status> {
        let market_booth_id = parse_uuid(
            request.into_inner().market_booth_id,
            "market_booth_id",
        )?;

        MarketBooth::delete(&self.pool, &market_booth_id)
            .await
            .map_err(db_err_to_grpc_status)?;

        Ok(Response::new(DeleteMarketBoothResponse {}))
    }
}
