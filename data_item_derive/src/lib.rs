#[macro_use] extern crate quote;
use syn::{Data, Fields, Type};
use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(DataItem)]
pub fn data_item_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_data_item(&ast)
}


fn impl_data_item(ast: &syn::DeriveInput) -> TokenStream {
    let name    = &ast.ident;
    let object  = struct_from_row(&ast.data);
    let table   = format!("{}s", name.to_string().to_lowercase());
    let sname   = format_ident!("{}", name.to_string().to_lowercase());
    let data_m  = format!("<{}>", sname);

    let get_all_name          = format_ident!("get_all_{}", table);
    let get_name_by_index     = format_ident!("get_{}_by_index", sname);
    let get_name_paginated    = format_ident!("get_{}_paginated", table);

    let create_name           = format_ident!("create_{}", sname);
    let create_name_by_index  = format_ident!("create_{}_by_index", sname);
    let put_name_by_index     = format_ident!("put_{}_by_index", sname);

    let remove_all_name       = format_ident!("remove_all_{}", table);
    let remove_name_by_index  = format_ident!("remove_{}_by_index", sname);

    let (all_fields_sql, all_dollars) = generate_fields_sql(&ast.data, false);
    let (fields_sql, dollars)         = generate_fields_sql(&ast.data, true);

    let all_fields_vec                = vector_fields_to_sql(&ast.data, false);
    let fields_vec                    = vector_fields_to_sql(&ast.data, true);

    let excluded_sql                  = generate_excluded_sql(&ast.data, true);

    let gen = quote! {
        // Endpoints
        #[get("/")]
        pub async fn #get_all_name(conn: KittyBox) -> JsonValue {
            json!({
                "msg_code": "no_message",
                "data": conn.run(|c| #name::get_all(c)).await,
            })
        }

        #[get("/<id>")]
        pub async fn #get_name_by_index(id: u32, conn: KittyBox) -> JsonValue {
            conn.run(
                move |c| {
                    if let Ok(#sname) = #name::from_id(c, id) {
                        json!({
                            "msg_code": "no_message",
                            "item_id": &#sname.id,
                            "data": &#sname,
                        })
                    } else {
                        json!({
                            "msg_code": "err_item_not_exist",
                            // "message": ,
                            "item_id": &id,
                        })
                    }
                }
            ).await
        }

        #[get("/?<page>")]
        pub async fn #get_name_paginated(page: u32, page_size: PageSize, conn: KittyBox) -> JsonValue {
            json!({
                "msg_code": "no_message",
                "page_number": page.clone(),
                "page_size": page_size.0.clone(),
                "data": conn.run(
                    move |c| #name::get_page(c, page, page_size.0)
                ).await,
            })
        }

        #[post("/", format = "application/json", data = #data_m)]
        pub async fn #create_name(#sname: Json<#name>, conn: KittyBox) -> JsonValue {
            json!({
                "msg_code": "info_new_item_ok",
                // "message": context.get_message("info_new_item_ok"),
                "item_id": conn.run(|c| #sname.into_inner().insert(c)).await,
            })
        }

        #[post("/<id>", format = "application/json", data = "<item>")]
        pub async fn #create_name_by_index(id: u32, item: Json<#name>, conn: KittyBox) -> JsonValue {
            conn.run(
                move |c| {
                    let mut #sname = item.into_inner();
                    #sname.id = id;

                    if let Ok(_) = #sname.insert_with_id(c) {
                        json!({
                            "msg_code": "info_new_item_ok",
                            // "message": ,
                            "item_id": &#sname.id,
                        })
                    } else {
                        json!({
                            "msg_code": "err_item_exists",
                            // "message": ,
                            "item_id": &#sname.id,
                        })
                    }
                }
            ).await
        }

        #[put("/<id>", format = "application/json", data = "<item>")]
        pub async fn #put_name_by_index(id: u32, item: Json<User>, conn: KittyBox) -> JsonValue {
            json!({
                "msg_code": "info_item_put_ok",
                // "message": context.get_message("info_new_item_ok"),
                "item_id": conn.run(move |c| {
                    let mut #sname = item.into_inner();
                    #sname.id = id;
                    #sname.put(c)
                }).await,
            })
        }

        #[delete("/")]
        pub async fn #remove_all_name(conn: KittyBox) -> JsonValue {
            json!({
                "msg_code": "info_items_removed",
                // "message": context.format_usize("info_items_removed", &vec![size])
                "items_removed": conn.run(|c| #name::delete_all(c)).await,
            })
        }

        #[delete("/<id>")]
        pub async fn #remove_name_by_index(id: u32, conn: KittyBox) -> JsonValue {
            match conn.run(
                move |c| #name::delete_with_id(c, id)
            ).await {
                Ok(#sname) => json!({
                    "msg_code": "info_remove_item_ok",
                    // "message": ,
                    "data": #sname,
                }),
                Err(_) => json!({
                    "msg_code": "err_item_not_exist",
                    // "message": ,
                    "item_id": id,
                }),
            }
        }

        #[async_trait]
        impl DataItem for #name {

            fn from_row(row: &postgres::Row) -> Self {
                #object
            }

            fn from_id(c: &mut postgres::Client, id: u32) -> Result<Self, postgres::Error> {
                match c.query_one(concat!("SELECT * FROM ", #table, " WHERE id = $1"), &[&(id as i32)]) {
                    Ok(row) => Ok(Self::from_row(&row)),
                    Err(e) => Err(e),
                }
            }

            fn get_all(c: &mut postgres::Client) -> Vec<Self> {
                c.query(concat!("SELECT * FROM ", #table), &[])
                .unwrap()
                .iter()
                .map(|row| Self::from_row(row))
                .collect()
            }

            fn get_page(c: &mut postgres::Client, page: u32, page_size: u32) -> Vec<Self> {
                c.query(concat!(
                        "SELECT * FROM ", #table, " ORDER BY id ASC LIMIT $1 OFFSET $2"
                    ),
                    &[&(page_size as i64), &((page * page_size) as i64)]
                )
                .unwrap()
                .iter()
                .map(|row| Self::from_row(row))
                .collect()
            }

            fn insert(&self, c: &mut postgres::Client) -> u32 {
                c.query_one(concat!(
                        "INSERT INTO ", #table, " (", stringify!(#fields_sql),
                        ")", "VALUES (", #dollars, ") RETURNING id"
                    ),
                    #fields_vec
                ).expect("Failed to insert item!").get::<_, i32>("id") as u32
            }

            fn insert_with_id(&self, c: &mut postgres::Client) -> Result<u32, postgres::Error> {
                match c.query_one(concat!(
                        "INSERT INTO ", #table, " (", stringify!(#all_fields_sql),
                        ")", "VALUES (", #all_dollars, ") RETURNING id"
                    ),
                    #all_fields_vec
                ) {
                    Ok(item) => Ok(item.get::<_, i32>("id") as u32),
                    Err(e) => Err(e),
                }
            }

            fn put(&self, c: &mut postgres::Client) -> u32 {
                c.query_one(concat!(
                        "INSERT INTO ", #table, " (", stringify!(#all_fields_sql), ")",
                        "VALUES (", #all_dollars, ") ON CONFLICT (id) DO UPDATE SET ",
                        stringify!(#excluded_sql), " RETURNING id"
                    ),
                    #all_fields_vec
                ).unwrap().get::<_, i32>("id") as u32
            }

            fn delete_all(c: &mut postgres::Client) -> i64 {
                let count: i64 = c.query_one(concat!("SELECT count(*) FROM ", #table), &[])
                    .unwrap()
                    .get("count");
                // @UseCase: do we want reset identity here? Probably yes.
                c.execute(concat!("TRUNCATE TABLE ", #table, " RESTART IDENTITY"), &[])
                    .expect("Fatal error when cleaning users table!");
                count
            }

            fn delete_with_id(c: &mut postgres::Client, id: u32) -> Result<Self, postgres::Error> {
                match c.query_one(concat!(
                        "DELETE FROM ", #table, " WHERE id = $1 RETURNING *"
                    ),
                    &[&(id as i32)]
                ) {
                    Ok(row) => Ok(Self::from_row(&row)),
                    Err(e)  => Err(e),
                }
            }

            fn get_api_endpoints() -> Vec<Route> {
                routes![
                    #get_all_name,
                    #get_name_by_index,
                    #get_name_paginated,

                    #create_name,
                    #create_name_by_index,
                    #put_name_by_index,

                    #remove_all_name,
                    #remove_name_by_index,
                ]
            }
        }
    };
    gen.into()
}


fn struct_from_row(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_values = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        match ty {
                            Type::Path(ref p) => {
                                match &p.path {
                                    path if path.is_ident("String") => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("bool")   => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("f32")    => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("f64")    => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("i32")    => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("i64")    => quote! { #name: row.get(stringify!(#name)) },
                                    path if path.is_ident("u32")    => quote! { #name: row.get::<_, i32>(stringify!(#name)) as u32 },
                                    path if path.is_ident("u64")    => quote! { #name: row.get::<_, i64>(stringify!(#name)) as u64 },
                                    _ => unimplemented!(),
                                }
                            }
                            _ => unimplemented!(),
                        }
                    });
                    quote! {
                        Self {
                            #(#field_values,)*
                        }
                    }
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn vector_fields_to_sql(data: &Data, count_id: bool) -> proc_macro2::TokenStream {
    let mut id = String::new();
    if count_id == true { id = "id".to_string() };

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_values = fields.named.iter().filter(|&item| &item.ident.as_ref().unwrap().to_string() != &id).map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        match ty {
                            Type::Path(ref p) => {
                                match &p.path {
                                    path if path.is_ident("String") => quote! { &self.#name },
                                    path if path.is_ident("bool")   => quote! { &self.#name },
                                    path if path.is_ident("i32")    => quote! { &self.#name },
                                    path if path.is_ident("i64")    => quote! { &self.#name },
                                    path if path.is_ident("f32")    => quote! { &self.#name },
                                    path if path.is_ident("f64")    => quote! { &self.#name },
                                    path if path.is_ident("u32")    => quote! { &(self.#name as i32) },
                                    path if path.is_ident("u64")    => quote! { &(self.#name as i64) },
                                    _ => unimplemented!(),
                                }
                            }
                            _ => unimplemented!(),
                        }
                    });
                    quote! {
                        &[
                            #(#field_values),*
                        ]
                    }
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn generate_fields_sql(data: &Data, count_id: bool) -> (proc_macro2::TokenStream, String) {
    let mut id = String::new();
    if count_id == true { id = "id".to_string() };

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let mut dollars = String::new();
                    let mut index = 1;
                    let field_values = fields.named.iter().filter(|&item| &item.ident.as_ref().unwrap().to_string() != &id).map(|f| {
                        let name = &f.ident;
                        if index > 1 {
                            dollars.push_str(", ");
                        }
                        dollars.push_str(format!("${}", index).as_str());
                        index += 1;
                        quote! { #name }
                    });

                    (quote! { #(#field_values),* }, dollars)
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn generate_excluded_sql(data: &Data, count_id: bool) -> proc_macro2::TokenStream {
    let mut id = String::new();
    if count_id == true { id = "id".to_string() };

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_values = fields.named.iter().filter(|&item| &item.ident.as_ref().unwrap().to_string() != &id).map(|f| {
                        let name = &f.ident;
                        quote! { #name = EXCLUDED.#name }
                    });

                    quote! { #(#field_values),* }
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
