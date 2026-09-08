use crate::types::{EmptyResult, OperationResult};
use serde_derive::{Deserialize, Serialize};
use zabbix_api::client::client::ZabbixApiClient;

/// Minimal web scenario representation: the `zabbix-api` crate model
/// doesn't expose `httptestid`, which is required for deletion.
#[derive(Deserialize, Debug, Clone)]
pub struct WebScenarioRef {
    #[serde(alias = "httptestid")]
    pub httptest_id: String,
    pub name: String,
}

#[derive(Serialize, Debug)]
struct GetWebScenariosByNamePrefixRequest {
    output: Vec<String>,
    search: NameFilter,
    #[serde(rename = "startSearch")]
    start_search: bool,
}

#[derive(Serialize, Debug)]
struct NameFilter {
    name: String,
}

#[derive(Deserialize, Debug)]
struct DeleteWebScenariosResponse {
    #[serde(alias = "httptestids")]
    pub httptest_ids: Vec<String>,
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
pub fn find_web_scenarios_by_name_prefix(
    zabbix_client: &impl ZabbixApiClient,
    session: &str,
    name_prefix: &str,
) -> OperationResult<Vec<WebScenarioRef>> {
    info!("find web scenarios with name prefix '{name_prefix}'..");

    let request = GetWebScenariosByNamePrefixRequest {
        output: vec!["httptestid".to_string(), "name".to_string()],
        search: NameFilter {
            name: name_prefix.to_string(),
        },
        start_search: true,
    };

    let response = zabbix_client
        .raw_api_call::<GetWebScenariosByNamePrefixRequest, Vec<WebScenarioRef>>(
            session,
            "httptest.get",
            &request,
        )?;

    let scenarios = response.result.unwrap_or_default();

    debug!("web scenarios found: {:?}", scenarios);

    Ok(scenarios)
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/delete
pub fn delete_web_scenarios(
    zabbix_client: &impl ZabbixApiClient,
    session: &str,
    httptest_ids: &[String],
) -> EmptyResult {
    if httptest_ids.is_empty() {
        return Ok(());
    }

    info!("deleting web scenarios with ids {:?}..", httptest_ids);

    let response = zabbix_client
        .raw_api_call::<&[String], DeleteWebScenariosResponse>(
            session,
            "httptest.delete",
            &httptest_ids,
        )?;

    if let Some(result) = response.result {
        debug!("web scenarios deleted: {:?}", result.httptest_ids);
    }

    Ok(())
}
