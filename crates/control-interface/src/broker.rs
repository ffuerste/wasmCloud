const DEFAULT_TOPIC_PREFIX: &str = "wasmbus.ctl";
// Copied from https://docs.rs/wasmcloud-core/0.15.0/wasmcloud_core/constant.CTL_API_VERSION_1.html
// to avoid a dependency on a crate for one constant
const CTL_API_VERSION_1: &str = "v1";

fn prefix(topic_prefix: &Option<String>, lattice: &str, version: &str) -> String {
    format!(
        "{}.{version}.{lattice}",
        topic_prefix
            .as_ref()
            .unwrap_or(&DEFAULT_TOPIC_PREFIX.to_string())
    )
}

pub mod v1 {
    use crate::broker::CTL_API_VERSION_1;

    use super::prefix;

    pub fn provider_auction_subject(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!(
            "{}.provider.auction",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn component_auction_subject(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!(
            "{}.component.auction",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn put_link(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!(
            "{}.link.put",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn delete_link(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!(
            "{}.link.del",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn publish_registries(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!(
            "{}.registry.put",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn put_config(topic_prefix: &Option<String>, lattice: &str, config_name: &str) -> String {
        format!(
            "{}.config.put.{config_name}",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn delete_config(
        topic_prefix: &Option<String>,
        lattice: &str,
        config_name: &str,
    ) -> String {
        format!(
            "{}.config.del.{config_name}",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn put_label(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!(
            "{}.label.put.{host_id}",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub fn delete_label(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!(
            "{}.label.del.{host_id}",
            prefix(topic_prefix, lattice, CTL_API_VERSION_1)
        )
    }

    pub mod commands {
        use crate::broker::CTL_API_VERSION_1;

        use super::prefix;

        pub fn scale_component(
            topic_prefix: &Option<String>,
            lattice: &str,
            host_id: &str,
        ) -> String {
            format!(
                "{}.component.scale.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn start_provider(
            topic_prefix: &Option<String>,
            lattice: &str,
            host_id: &str,
        ) -> String {
            format!(
                "{}.provider.start.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn stop_provider(
            topic_prefix: &Option<String>,
            lattice: &str,
            host_id: &str,
        ) -> String {
            format!(
                "{}.provider.stop.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn update_component(
            topic_prefix: &Option<String>,
            lattice: &str,
            host_id: &str,
        ) -> String {
            format!(
                "{}.component.update.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn stop_host(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
            format!(
                "{}.host.stop.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }
    }

    pub mod queries {
        use crate::broker::CTL_API_VERSION_1;

        use super::prefix;

        pub fn link_definitions(topic_prefix: &Option<String>, lattice: &str) -> String {
            format!(
                "{}.link.get",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn claims(topic_prefix: &Option<String>, lattice: &str) -> String {
            format!(
                "{}.claims.get",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn host_inventory(
            topic_prefix: &Option<String>,
            lattice: &str,
            host_id: &str,
        ) -> String {
            format!(
                "{}.host.get.{host_id}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn hosts(topic_prefix: &Option<String>, lattice: &str) -> String {
            format!(
                "{}.host.ping",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1)
            )
        }

        pub fn config(topic_prefix: &Option<String>, lattice: &str, config_name: &str) -> String {
            format!(
                "{}.config.get.{config_name}",
                prefix(topic_prefix, lattice, CTL_API_VERSION_1),
            )
        }
    }
}
