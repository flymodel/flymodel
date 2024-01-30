use flymodel_client::client::Client;
use flymodel_graphql::gql::create_model::{CreateModel, CreateModelVariables, Model};

async fn create_model_test_base() -> anyhow::Result<()> {
    let cli = Client::new("http://localhost:9009")?;
    let req = CreateModelVariables {
        name: "test.model.lg".into(),
        namespace: 1,
    };
    let model = cli.create_model(req.clone()).await?;

    let expect = flymodel_dev::regional::Regional::new(
        CreateModel {
            create_model: Model {
                id: 2,
                name: req.name.clone(),
                namespace_id: req.namespace,
            },
        },
        CreateModel {
            create_model: Model {
                id: 3,
                name: req.name.clone(),
                namespace_id: req.namespace,
            },
        },
    );

    let ref_e = expect.as_ref();
    assert_eq!(model.create_model.name, ref_e.create_model.name);
    assert_eq!(
        model.create_model.namespace_id,
        ref_e.create_model.namespace_id
    );
    Ok(())
}

#[cfg(not(feature = "wasm"))]
mod test {
    #[tokio::test]
    async fn create_model_test() -> anyhow::Result<()> {
        super::create_model_test_base().await
    }
}

#[cfg(feature = "wasm")]
mod wasm_tests {
    cfg_if::cfg_if! {
        if #[cfg(feature = "wasm-web")] {
            wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        }
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_wasm_create_model() -> anyhow::Result<()> {
        super::create_model_test_base().await
    }
}
