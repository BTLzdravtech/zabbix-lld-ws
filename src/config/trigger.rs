use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixTriggerConfig {
    pub name: String,
    pub priority: u8,
    pub problem_expression: String,
    pub recovery_mode: u8,
    pub recovery_expression: String,
    pub event_name: String,
    pub url: String,

    /// Tags assigned to created triggers.
    /// Template variables are supported in both `tag` and `value`.
    #[serde(default = "get_empty_tag_vec")]
    pub tags: Vec<ZabbixTriggerTagConfig>,
}

impl Display for ZabbixTriggerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name '{}', priority: {}, problem-expression: '{}', recovery-mode: {}, recovery-expression: '{}', event-name: '{}', url: '{}', tags: {:?}",
               self.name, self.priority, self.problem_expression,
               self.recovery_mode, self.recovery_expression,
               self.event_name, self.url, self.tags
        )
    }
}

/// Zabbix API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/object#trigger-tag
#[derive(PartialEq, Deserialize, Clone, Debug)]
pub struct ZabbixTriggerTagConfig {
    pub tag: String,
    pub value: String,
}

fn get_empty_tag_vec() -> Vec<ZabbixTriggerTagConfig> {
    Vec::new()
}
