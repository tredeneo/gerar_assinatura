use query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = query::buscar_no_banco_pelo_nome("dan%").await?;
    // let tmp =query::buscar_no_banco_pelo_id(27987).await?;
    Ok(())
}
