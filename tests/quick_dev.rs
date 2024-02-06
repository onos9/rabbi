use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello/Godsgrace").await?.print().await?;
    hc.do_get("/index.html").await?.print().await?;
    let req = hc.do_post(
        "/api/login",
        serde_json::json!({
            "username": "Godsgrace",
            "password": "password"
        }),
    );
    req.await?.print().await?;

    let req = hc.do_post(
        "/api/tickets",
        serde_json::json!({
            "title": "Ticket AAA"
        }),
    );
    req.await?.print().await?;

    let req = hc.do_get("/api/tickets");
    req.await?.print().await?;

    Ok(())
}
