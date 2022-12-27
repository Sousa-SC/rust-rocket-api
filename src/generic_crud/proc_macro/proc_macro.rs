extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote, TokenStreamExt};
use syn::{DeriveInput, Ident, Lit, Meta, parse_macro_input, parse_quote, Path, PathArguments, PathSegment, Token, TypePath};
use syn::__private::Span;

fn get_field_by_name<'a>(name: &'a str, data_struct: &'a syn::DataStruct) -> &'a syn::Field {
    data_struct.fields.iter().find(|field| {
        field.attrs.iter().any(|attr| {
            attr.path.is_ident(name)
        })
    }).expect(format!("#[{}] field not found", name).as_str())
}

fn get_attribute_value(input: &DeriveInput, attr_name: &str) -> Option<String> {
    // Iterate over the attributes of the input struct
    for attr in &input.attrs {
        // Check if the attribute is the one with the specified name
        if attr.path.is_ident(attr_name) {
            // Extract the attribute's value
            let meta = attr.parse_meta().unwrap();
            if let Meta::NameValue(nv) = meta {
                if let Lit::Str(ref s) = nv.lit {
                    // Return the attribute value as a string
                    return Some(s.value());
                }
            }
        }
    }

    // Attribute not found
    None
}

fn build_type_path(path: &str) -> TypePath {
    let mut segments: syn::punctuated::Punctuated<_, Token![::]> = syn::punctuated::Punctuated::new();

    for segment in path.split("::") {
        segments.push(PathSegment {
            ident: Ident::new(segment, Span::call_site()),
            arguments: PathArguments::None,
        });
    }

    TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    }
}


#[proc_macro_derive(CRUDModel, attributes(module, idField))]
pub fn crud_model(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let module = get_attribute_value(&input, "module").expect("#[module = \"foo\"] attribute not found");

    let entity_module = format!("crate::entity::{}", module);

    let active_model = build_type_path(&format!("{}::ActiveModel", entity_module));
    let model = build_type_path(&format!("{}::Model", entity_module));


    let data_struct = match &input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => unimplemented!()
    };
    let id_field = get_field_by_name("idField", data_struct);

    let post_name_ident = format_ident!("Post{}", name);
    let fields = data_struct.fields.iter().filter(|field| field != &id_field);

    let partial_name_ident = format_ident!("Partial{}", name);
    let partial_fields = data_struct.fields.iter()
        .filter(|field| field != &id_field)
        .map(|field| {
            let mut field = field.clone();
            let ty = field.ty.clone();
            field.ty = parse_quote! { Option<#ty> };
            field
        });

    let mut from_entity_fields = quote! {};
    // Get the fields of the struct
    if let syn::Fields::Named(ref fields) = data_struct.fields {
        // Create a match expression that converts each field in the input model
        for field in fields.named.iter() {
            let ident = &field.ident;
            from_entity_fields.append_all(quote! {
                #ident: obj.#ident,
            });
        }
    }

    let mut to_active_model_fields = quote! {};
    if let syn::Fields::Named(ref fields) = data_struct.fields {
        // Create a match expression that converts each field in the input model
        for field in fields.named.iter() {
            let ident = &field.ident;
            // if ident is idField, then use id instead
            if ident == &id_field.ident {
                to_active_model_fields.append_all(quote! {
                    id: ActiveValue::set(placeholder.id),
                });
            } else {
                to_active_model_fields.append_all(quote! {
                    #ident: ActiveValue::set(self.#ident.unwrap_or(placeholder.#ident)),
                });
            }
        }
    }

    // Hand the output tokens back to the compiler
    let expanded = quote! {
        use sea_orm::DeriveIntoActiveModel;
        use generic_crud_trait::{FromEntity, ToActiveModel};
        use sea_orm::ActiveValue;
        use #active_model;

        impl FromEntity<#model> for #name {
            fn from_entity(obj: #model) -> Self {
                Self {
                    #from_entity_fields
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, DeriveIntoActiveModel)]
        pub struct #post_name_ident {
            #(#fields),*
        }

        #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
        pub struct #partial_name_ident {
            #(#partial_fields),*
        }

        impl ToActiveModel<#active_model, #model> for #partial_name_ident {
            fn into_active_model(self, placeholder: #model) -> #active_model {
                #active_model {
                    #to_active_model_fields
                }
            }
        }

    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CRUDControllerImpl, attributes(module, service, model))]
pub fn crud_controller(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let module = get_attribute_value(&input, "module").expect("#[module = \"foo\"] attribute not found");
    let module_capitalized = heck::AsUpperCamelCase(&module);
    let models_module = format!("crate::{}::models", module);
    let service_module = format!("crate::{}::service", module);

    let model = build_type_path(&format!("{}::{}", models_module, module_capitalized));
    let post_model = build_type_path(&format!("{}::Post{}", models_module, module_capitalized));
    let partial_model = build_type_path(&format!("{}::Partial{}", models_module, module_capitalized));
    let service = build_type_path(&format!("{}::{}Service", service_module, module_capitalized));


    let impl_block = quote! {

        use rocket::http::uri::Origin;
        use rocket::response::status::{Created, NoContent};
        use rocket::serde::json::Json;
        use sea_orm_rocket::Connection;

        use generic_crud_trait::CRUDControllerTrait;
        use generic_crud_trait::CRUDServiceTrait;
        use db::Db;

        #[async_trait]
        impl CRUDControllerTrait<#model, #post_model, #partial_model> for #name {

            async fn reads(conn: Connection<'_, Db>) -> Json<Vec<#model>> {
                let db = conn.into_inner();
                let obj = #service::get_all(&db).await;
                Json(obj)
            }

            async fn read(obj_id: i32, conn: Connection<'_, Db>) -> Option<Json<#model>> {
                let db = conn.into_inner();
                let obj = #service::get_by_id(obj_id, &db).await;
                obj.map(|obj| Json(obj))
            }

            async fn post(obj: Json<#post_model>, conn: Connection<'_, Db>, uri: &Origin<'_>) -> Created<Json<#model>> {
                let db = conn.into_inner();
                let obj = #service::create(obj.into_inner(), &db).await;
                Created::new(format!("{}/{}", uri.to_string(), obj.id)).body(Json(obj))
            }

            async fn patch(obj_id: i32, obj: Json<#partial_model>, conn: Connection<'_, Db>) -> Option<Json<#model>> {
                let db = conn.into_inner();
                let obj = #service::update(obj_id, obj.into_inner(), &db).await;
                obj.map(|obj| Json(obj))
            }

            async fn delete(obj_id: i32, conn: Connection<'_, Db>) -> Option<NoContent> {
                let db = conn.into_inner();
                let res = #service::delete(obj_id, &db).await;
                res.map(|_res| NoContent)
            }

        }

    };

    TokenStream::from(impl_block)
}

#[proc_macro_derive(CRUDServiceImpl, attributes(module))]
pub fn crud_service(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let module = get_attribute_value(&input, "module").expect("#[module = \"foo\"] attribute not found");
    let module_capitalized = heck::AsUpperCamelCase(&module);

    let models_module = format!("crate::{}::models", module);
    let entity_module = format!("crate::entity::{}", module);

    let model = build_type_path(&format!("{}::{}", models_module, module_capitalized));
    let active_model = build_type_path(&format!("{}::ActiveModel", entity_module));
    let model_from_entity = build_type_path(&format!("{}::{}::from_entity", models_module, module_capitalized));
    let post_model = build_type_path(&format!("{}::Post{}", models_module, module_capitalized));
    let partial_model = build_type_path(&format!("{}::Partial{}", models_module, module_capitalized));
    let entity = build_type_path(&format!("{}::Entity", entity_module));

    let impl_block = quote! {

        use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel};

        use generic_crud_trait::FromEntity;
        use generic_crud_trait::CRUDServiceTrait;
        use generic_crud_trait::ToActiveModel;
        use db::Db;
        use #entity as CrudEntity;

        #[async_trait]
        impl CRUDServiceTrait<#model, #post_model, #partial_model> for #name {

           async fn get_all(db: &DatabaseConnection) -> Vec<#model> {
                let objs = CrudEntity::find()
                    .all(db)
                    .await
                    .unwrap();

                objs.into_iter().map(|obj| #model_from_entity(obj)).collect()
            }

            async fn get_by_id(obj_id: i32, db: &DatabaseConnection) -> Option<#model> {
                let res = CrudEntity::find_by_id(obj_id).one(db).await;

                match res {
                    Ok(res) => res.map(|obj| #model_from_entity(obj)),
                    Err(_err) => None,
                }
            }

            async fn create(form: #post_model, db: &DatabaseConnection) -> #model {
                let obj = form.into_active_model();

                let obj = obj.insert(db).await.unwrap();

                #model_from_entity(obj)
            }

            async fn update(obj_id: i32, form: #partial_model, db: &DatabaseConnection) -> Option<#model> {
                let obj = CrudEntity::find_by_id(obj_id).one(db).await.unwrap()?;

                let obj = form.into_active_model(obj);

                let obj = obj.update(db).await.unwrap();

                Some(#model_from_entity(obj))
            }

            async fn delete(obj_id: i32, db: &DatabaseConnection) -> Option<()> {
                let obj: #active_model = CrudEntity::find_by_id(obj_id)
                    .one(db)
                    .await
                    .expect(&format!("Cannot find {} with the specified ID.", #module))
                    .map(Into::into)?;

                obj.delete(db).await
                    .map(|_| ())
                    .map_err(|err|
                        rocket::response::status::NotFound(format!("Cannot delete {}: {}", #module, err))
                    )
                    .ok()
            }

        }

    };

    TokenStream::from(impl_block)
}
