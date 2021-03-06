pub mod hosts {
    use serde::Deserialize;
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::http::http::send_post_request;
    use crate::types::types::OperationResult;
    use crate::zabbix::zabbix;
    use crate::zabbix::zabbix::{log_zabbix_error, ZabbixError, ZabbixRequest};

    #[derive(Serialize)]
    struct SearchRequestParams {
        hostids: Vec<String>
    }

    #[derive(Deserialize)]
    struct SearchResponse {
        result: Option<Vec<ZabbixHost>>,
        error: Option<ZabbixError>
    }

    #[derive(Deserialize)]
    pub struct ZabbixHost {
        pub hostid: String,
        pub host: String
    }

    pub fn find_hosts(client: &reqwest::blocking::Client,
                      api_endpoint: &str, api_token: &str,
                      ids: Vec<String>) -> OperationResult<Vec<ZabbixHost>> {
        info!("find hosts by ids..");

        let params = SearchRequestParams { hostids: ids };

        let request: ZabbixRequest<SearchRequestParams> = ZabbixRequest::new(
            "host.get", params, api_token
        );

        match send_post_request(client, api_endpoint, request) {
            Ok(response) => {
                let search_response: SearchResponse = serde_json::from_str(&response)
                                                .expect(zabbix::UNSUPPORTED_RESPONSE_MESSAGE);

                match search_response.result {
                    Some(hosts) => Ok(hosts),
                    None => {
                        log_zabbix_error(&search_response.error);
                        error!("unable to find zabbix hosts");
                        Err(OperationError::Error)
                    }
                }
            }
            Err(_) => {
                error!("unable to find zabbix hosts");
                Err(OperationError::Error)
            }
        }
    }
}
