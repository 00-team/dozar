use utoipa::{
    openapi::{
        security::{Http, HttpAuthScheme, SecurityScheme},
        Components, SecurityRequirement,
    },
    Modify, OpenApi,
};

pub struct AddSecurity;

impl Modify for AddSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if openapi.components.is_none() {
            openapi.components = Some(Components::new());
        }

        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "user_token",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }

        openapi.security =
            Some(vec![SecurityRequirement::new("user_token", [] as [&str; 0])]);
    }
}

pub struct UpdatePaths;

impl Modify for UpdatePaths {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let base_path = if let Some(s) = openapi.servers.as_mut() {
            if !s.is_empty() {
                s.remove(0).url
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        openapi.paths.paths = openapi
            .paths
            .paths
            .iter()
            .map(|(path, value)| {
                let path = base_path.clone() + path;
                let mut value = value.to_owned();
                value.operations.iter_mut().for_each(|(_, op)| {
                    if let Some(tags) = &openapi.tags {
                        op.tags =
                            Some(tags.iter().map(|t| t.name.clone()).collect());
                    }
                });

                (path, value)
            })
            .collect();
    }
}

#[derive(OpenApi)]
#[openapi(
    servers((url = "/api")),
    modifiers(&AddSecurity)
)]
pub struct ApiDoc;
