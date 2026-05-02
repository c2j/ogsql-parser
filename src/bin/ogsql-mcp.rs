use ogsql_parser::mcp::OgsqlServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("ogsql-mcp: starting MCP server on stdio");
    let service = OgsqlServer
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| {
            eprintln!("ogsql-mcp: server init failed: {:?}", e);
            e
        })?;
    service.waiting().await?;
    Ok(())
}
