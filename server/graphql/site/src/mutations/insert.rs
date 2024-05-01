use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use repository::{RepositoryError, Site, SiteRepository};
use service::auth::{Resource, ResourceAccessRequest};

#[derive(InputObject)]
#[graphql(name = "InsertSiteInput")]
pub struct InsertSiteInput {
    pub id: String,
    pub site_id: i32,
}

#[derive(Union)]
#[graphql(name = "InsertSiteResponse")]
pub enum InsertResponse {
    Response(SiteNode),
}

pub fn insert_site(
    ctx: &Context<'_>,
    store_id: String,
    input: InsertSiteInput,
) -> Result<InsertResponse> {
    // validate_auth(
    //     ctx,
    //     &ResourceAccessRequest {
    //         resource: Resource::ServerAdmin,
    //         store_id: Some(store_id.to_string()),
    //     },
    // )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.basic_context()?;

    let site_repo = SiteRepository::new(&service_context.connection);

    let res = site_repo.upsert_one(&Site {
        id: input.id,
        site_id: input.site_id,
    });

    map_response(res)
}

pub struct SiteNode {
    site: Site,
}

#[Object]
impl SiteNode {
    pub async fn id(&self) -> &str {
        &self.site.id
    }

    pub async fn site_id(&self) -> &i32 {
        &self.site.site_id
    }
}

impl SiteNode {
    pub fn from_domain(site: Site) -> SiteNode {
        SiteNode { site }
    }

    pub fn from_vec(sites: Vec<Site>) -> Vec<SiteNode> {
        sites.into_iter().map(SiteNode::from_domain).collect()
    }
}

fn map_response(from: Result<(), RepositoryError>) -> Result<InsertResponse> {
    let result = match from {
        Ok(()) => InsertResponse::Response(SiteNode::from_domain(Site {
            id: "".to_string(),
            site_id: 0,
        })),
        // todo map error
        Err(error) => Err(StandardGraphqlError::InternalError(format!("{:?}", error)).extend())?,
    };

    Ok(result)
}

// fn map_error(error: ServiceError) -> Result<ErrorInterface> {
//     use StandardGraphqlError::*;
//     let formatted_error = format!("{:#?}", error);

//     let graphql_error = match error {
//         ServiceError::VariantWithPackSizeAlreadyExists => {
//             return Ok(ErrorInterface::VariantWithPackSizeAlreadyExists(
//                 VariantWithPackSizeAlreadyExists,
//             ))
//         }
//         ServiceError::CannotAddPackSizeOfZero => {
//             return Ok(ErrorInterface::CannotAddPackSizeOfZero(
//                 CannotAddPackSizeOfZero,
//             ))
//         }
//         ServiceError::CannotAddWithNoAbbreviationAndName => {
//             return Ok(ErrorInterface::CannotAddWithNoAbbreviationAndName(
//                 CannotAddWithNoAbbreviationAndName,
//             ))
//         }

//         ServiceError::ItemDoesNotExist | ServiceError::SiteAlreadyExists => {
//             BadUserInput(formatted_error)
//         }
//         ServiceError::DatabaseError(_) | ServiceError::CreatedRecordNotFound => {
//             InternalError(formatted_error)
//         }
//     };

//     Err(graphql_error.extend())
// }
