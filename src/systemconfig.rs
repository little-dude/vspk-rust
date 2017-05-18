// Copyright (c) 2015 Alcatel-Lucent, (c) 2016 Nokia
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the copyright holder nor the names of its contributors
//       may be used to endorse or promote products derived from this software without
//       specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


use bambou::{Error, RestEntity, Session};
use reqwest::Response;
use std::collections::BTreeMap;
use serde_json;


pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SystemConfig<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    _session: Option<&'a Session>,

    #[serde(rename="ID")]
    id: Option<String>,

    #[serde(rename="parentID")]
    parent_id: Option<String>,

    #[serde(rename="parentType")]
    parent_type: Option<String>,

    owner: Option<String>,

    
    #[serde(rename="ACLAllowOrigin")]
    pub acl_allow_origin: Option<String>,
    
    #[serde(rename="ECMPCount")]
    pub ecmp_count: u64,
    
    #[serde(rename="LDAPSyncInterval")]
    pub ldap_sync_interval: u64,
    
    #[serde(rename="LDAPTrustStoreCertifcate")]
    pub ldap_trust_store_certifcate: Option<String>,
    
    #[serde(rename="LDAPTrustStorePassword")]
    pub ldap_trust_store_password: Option<String>,
    
    #[serde(rename="ADGatewayPurgeTime")]
    pub ad_gateway_purge_time: u64,
    
    #[serde(rename="RDLowerLimit")]
    pub rd_lower_limit: u64,
    
    #[serde(rename="RDPublicNetworkLowerLimit")]
    pub rd_public_network_lower_limit: u64,
    
    #[serde(rename="RDPublicNetworkUpperLimit")]
    pub rd_public_network_upper_limit: u64,
    
    #[serde(rename="RDUpperLimit")]
    pub rd_upper_limit: u64,
    
    #[serde(rename="ZFBBootstrapEnabled")]
    pub zfb_bootstrap_enabled: bool,
    
    #[serde(rename="ZFBRequestRetryTimer")]
    pub zfb_request_retry_timer: u64,
    
    #[serde(rename="ZFBSchedulerStaleRequestTimeout")]
    pub zfb_scheduler_stale_request_timeout: u64,
    
    #[serde(rename="DHCPOptionSize")]
    pub dhcp_option_size: u64,
    
    #[serde(rename="VLANIDLowerLimit")]
    pub vlanid_lower_limit: u64,
    
    #[serde(rename="VLANIDUpperLimit")]
    pub vlanid_upper_limit: u64,
    
    #[serde(rename="VMCacheSize")]
    pub vm_cache_size: u64,
    
    #[serde(rename="VMPurgeTime")]
    pub vm_purge_time: u64,
    
    #[serde(rename="VMResyncDeletionWaitTime")]
    pub vm_resync_deletion_wait_time: u64,
    
    #[serde(rename="VMResyncOutstandingInterval")]
    pub vm_resync_outstanding_interval: u64,
    
    #[serde(rename="VMUnreachableCleanupTime")]
    pub vm_unreachable_cleanup_time: u64,
    
    #[serde(rename="VMUnreachableTime")]
    pub vm_unreachable_time: u64,
    
    #[serde(rename="VNIDLowerLimit")]
    pub vnid_lower_limit: u64,
    
    #[serde(rename="VNIDPublicNetworkLowerLimit")]
    pub vnid_public_network_lower_limit: u64,
    
    #[serde(rename="VNIDPublicNetworkUpperLimit")]
    pub vnid_public_network_upper_limit: u64,
    
    #[serde(rename="VNIDUpperLimit")]
    pub vnid_upper_limit: u64,
    
    #[serde(rename="APIKeyRenewalInterval")]
    pub api_key_renewal_interval: u64,
    
    #[serde(rename="APIKeyValidity")]
    pub api_key_validity: u64,
    
    #[serde(rename="VPortInitStatefulTimer")]
    pub vport_init_stateful_timer: u64,
    
    #[serde(rename="LRUCacheSizePerSubnet")]
    pub lru_cache_size_per_subnet: u64,
    
    #[serde(rename="VSCOnSameVersionAsVSD")]
    pub vsc_on_same_version_as_vsd: bool,
    
    #[serde(rename="VSDReadOnlyMode")]
    pub vsd_read_only_mode: bool,
    
    #[serde(rename="VSDUpgradeIsComplete")]
    pub vsd_upgrade_is_complete: bool,
    
    #[serde(rename="ASNumber")]
    pub as_number: u64,
    
    #[serde(rename="RTLowerLimit")]
    pub rt_lower_limit: u64,
    
    #[serde(rename="RTPublicNetworkLowerLimit")]
    pub rt_public_network_lower_limit: u64,
    
    #[serde(rename="RTPublicNetworkUpperLimit")]
    pub rt_public_network_upper_limit: u64,
    
    #[serde(rename="RTUpperLimit")]
    pub rt_upper_limit: u64,
    
    #[serde(rename="EVPNBGPCommunityTagASNumber")]
    pub evpnbgp_community_tag_as_number: u64,
    
    #[serde(rename="EVPNBGPCommunityTagLowerLimit")]
    pub evpnbgp_community_tag_lower_limit: u64,
    
    #[serde(rename="EVPNBGPCommunityTagUpperLimit")]
    pub evpnbgp_community_tag_upper_limit: u64,
    
    #[serde(rename="pageMaxSize")]
    pub page_max_size: u64,
    
    #[serde(rename="pageSize")]
    pub page_size: u64,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="maxFailedLogins")]
    pub max_failed_logins: u64,
    
    #[serde(rename="maxResponse")]
    pub max_response: u64,
    
    #[serde(rename="accumulateLicensesEnabled")]
    pub accumulate_licenses_enabled: bool,
    
    #[serde(rename="perDomainVlanIdEnabled")]
    pub per_domain_vlan_id_enabled: bool,
    
    #[serde(rename="performancePathSelectionVNID")]
    pub performance_path_selection_vnid: u64,
    
    #[serde(rename="serviceIDUpperLimit")]
    pub service_id_upper_limit: u64,
    
    #[serde(rename="keyServerMonitorEnabled")]
    pub key_server_monitor_enabled: bool,
    
    #[serde(rename="keyServerVSDDataSynchronizationInterval")]
    pub key_server_vsd_data_synchronization_interval: u64,
    
    #[serde(rename="offsetCustomerID")]
    pub offset_customer_id: u64,
    
    #[serde(rename="offsetServiceID")]
    pub offset_service_id: u64,
    
    #[serde(rename="ejbcaNSGCertificateProfile")]
    pub ejbca_nsg_certificate_profile: Option<String>,
    
    #[serde(rename="ejbcaNSGEndEntityProfile")]
    pub ejbca_nsg_end_entity_profile: Option<String>,
    
    #[serde(rename="ejbcaOCSPResponderCN")]
    pub ejbca_ocsp_responder_cn: Option<String>,
    
    #[serde(rename="ejbcaOCSPResponderURI")]
    pub ejbca_ocsp_responder_uri: Option<String>,
    
    #[serde(rename="ejbcaVspRootCa")]
    pub ejbca_vsp_root_ca: Option<String>,
    
    #[serde(rename="alarmsMaxPerObject")]
    pub alarms_max_per_object: u64,
    
    #[serde(rename="elasticClusterName")]
    pub elastic_cluster_name: Option<String>,
    
    #[serde(rename="allowEnterpriseAvatarOnNSG")]
    pub allow_enterprise_avatar_on_nsg: bool,
    
    #[serde(rename="globalMACAddress")]
    pub global_mac_address: Option<String>,
    
    #[serde(rename="flowCollectionEnabled")]
    pub flow_collection_enabled: bool,
    
    #[serde(rename="inactiveTimeout")]
    pub inactive_timeout: u64,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="domainTunnelType")]
    pub domain_tunnel_type: Option<String>,
    
    #[serde(rename="postProcessorThreadsCount")]
    pub post_processor_threads_count: u64,
    
    #[serde(rename="groupKeyDefaultSEKGenerationInterval")]
    pub group_key_default_sek_generation_interval: u64,
    
    #[serde(rename="groupKeyDefaultSEKLifetime")]
    pub group_key_default_sek_lifetime: u64,
    
    #[serde(rename="groupKeyDefaultSEKPayloadEncryptionAlgorithm")]
    pub group_key_default_sek_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSEKPayloadSigningAlgorithm")]
    pub group_key_default_sek_payload_signing_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedGenerationInterval")]
    pub group_key_default_seed_generation_interval: u64,
    
    #[serde(rename="groupKeyDefaultSeedLifetime")]
    pub group_key_default_seed_lifetime: u64,
    
    #[serde(rename="groupKeyDefaultSeedPayloadAuthenticationAlgorithm")]
    pub group_key_default_seed_payload_authentication_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedPayloadEncryptionAlgorithm")]
    pub group_key_default_seed_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedPayloadSigningAlgorithm")]
    pub group_key_default_seed_payload_signing_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficAuthenticationAlgorithm")]
    pub group_key_default_traffic_authentication_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficEncryptionAlgorithm")]
    pub group_key_default_traffic_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficEncryptionKeyLifetime")]
    pub group_key_default_traffic_encryption_key_lifetime: u64,
    
    #[serde(rename="groupKeyGenerationIntervalOnForcedReKey")]
    pub group_key_generation_interval_on_forced_re_key: u64,
    
    #[serde(rename="groupKeyGenerationIntervalOnRevoke")]
    pub group_key_generation_interval_on_revoke: u64,
    
    #[serde(rename="groupKeyMinimumSEKGenerationInterval")]
    pub group_key_minimum_sek_generation_interval: u64,
    
    #[serde(rename="groupKeyMinimumSEKLifetime")]
    pub group_key_minimum_sek_lifetime: u64,
    
    #[serde(rename="groupKeyMinimumSeedGenerationInterval")]
    pub group_key_minimum_seed_generation_interval: u64,
    
    #[serde(rename="groupKeyMinimumSeedLifetime")]
    pub group_key_minimum_seed_lifetime: u64,
    
    #[serde(rename="groupKeyMinimumTrafficEncryptionKeyLifetime")]
    pub group_key_minimum_traffic_encryption_key_lifetime: u64,
    
    #[serde(rename="nsgBootstrapEndpoint")]
    pub nsg_bootstrap_endpoint: Option<String>,
    
    #[serde(rename="nsgConfigEndpoint")]
    pub nsg_config_endpoint: Option<String>,
    
    #[serde(rename="nsgLocalUiUrl")]
    pub nsg_local_ui_url: Option<String>,
    
    #[serde(rename="esiID")]
    pub esi_id: u64,
    
    #[serde(rename="csprootAuthenticationMethod")]
    pub csproot_authentication_method: Option<String>,
    
    #[serde(rename="stackTraceEnabled")]
    pub stack_trace_enabled: bool,
    
    #[serde(rename="statefulACLNonTCPTimeout")]
    pub stateful_acl_non_tcp_timeout: u64,
    
    #[serde(rename="statefulACLTCPTimeout")]
    pub stateful_acltcp_timeout: u64,
    
    #[serde(rename="staticWANServicePurgeTime")]
    pub static_wan_service_purge_time: u64,
    
    #[serde(rename="statisticsEnabled")]
    pub statistics_enabled: bool,
    
    #[serde(rename="statsCollectorAddress")]
    pub stats_collector_address: Option<String>,
    
    #[serde(rename="statsCollectorPort")]
    pub stats_collector_port: Option<String>,
    
    #[serde(rename="statsCollectorProtoBufPort")]
    pub stats_collector_proto_buf_port: Option<String>,
    
    #[serde(rename="statsMaxDataPoints")]
    pub stats_max_data_points: u64,
    
    #[serde(rename="statsMinDuration")]
    pub stats_min_duration: u64,
    
    #[serde(rename="statsNumberOfDataPoints")]
    pub stats_number_of_data_points: u64,
    
    #[serde(rename="statsTSDBServerAddress")]
    pub stats_tsdb_server_address: Option<String>,
    
    #[serde(rename="stickyECMPIdleTimeout")]
    pub sticky_ecmp_idle_timeout: u64,
    
    #[serde(rename="subnetResyncInterval")]
    pub subnet_resync_interval: u64,
    
    #[serde(rename="subnetResyncOutstandingInterval")]
    pub subnet_resync_outstanding_interval: u64,
    
    #[serde(rename="customerIDUpperLimit")]
    pub customer_id_upper_limit: u64,
    
    #[serde(rename="customerKey")]
    pub customer_key: Option<String>,
    
    #[serde(rename="avatarBasePath")]
    pub avatar_base_path: Option<String>,
    
    #[serde(rename="avatarBaseURL")]
    pub avatar_base_url: Option<String>,
    
    #[serde(rename="eventLogCleanupInterval")]
    pub event_log_cleanup_interval: u64,
    
    #[serde(rename="eventLogEntryMaxAge")]
    pub event_log_entry_max_age: u64,
    
    #[serde(rename="eventProcessorInterval")]
    pub event_processor_interval: u64,
    
    #[serde(rename="eventProcessorMaxEventsCount")]
    pub event_processor_max_events_count: u64,
    
    #[serde(rename="eventProcessorTimeout")]
    pub event_processor_timeout: u64,
    
    #[serde(rename="twoFactorCodeExpiry")]
    pub two_factor_code_expiry: u64,
    
    #[serde(rename="twoFactorCodeLength")]
    pub two_factor_code_length: u64,
    
    #[serde(rename="twoFactorCodeSeedLength")]
    pub two_factor_code_seed_length: u64,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="dynamicWANServiceDiffTime")]
    pub dynamic_wan_service_diff_time: u64,
    
    #[serde(rename="syslogDestinationHost")]
    pub syslog_destination_host: Option<String>,
    
    #[serde(rename="syslogDestinationPort")]
    pub syslog_destination_port: u64,
    
    #[serde(rename="sysmonCleanupTaskInterval")]
    pub sysmon_cleanup_task_interval: u64,
    
    #[serde(rename="sysmonNodePresenceTimeout")]
    pub sysmon_node_presence_timeout: u64,
    
    #[serde(rename="sysmonProbeResponseTimeout")]
    pub sysmon_probe_response_timeout: u64,
    
    #[serde(rename="systemAvatarData")]
    pub system_avatar_data: Option<String>,
    
    #[serde(rename="systemAvatarType")]
    pub system_avatar_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for SystemConfig<'a> {
    fn fetch(&mut self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.fetch_entity(self),
            None => Err(Error::NoSession),
        }
    }

    fn save(&mut self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.save(self),
            None => Err(Error::NoSession),
        }
    }

    fn delete(self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.delete(self),
            None => Err(Error::NoSession),
        }
    }

    fn create_child<C>(&self, child: &mut C) -> Result<Response, Error>
        where C: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.create_child(self, child),
            None => Err(Error::NoSession),
        }
    }

    fn path() -> &'static str {
        "systemconfig"
    }

    fn group_path() -> &'static str {
        "systemconfigs"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn id(&self) -> Option<&str> {
        self.id.as_ref().and_then(|id| Some(id.as_str()))
    }

    fn fetch_children<R>(&self, children: &mut Vec<R>) -> Result<Response, Error>
        where R: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.fetch_children(self, children),
            None => Err(Error::NoSession),
        }
    }

    fn get_session(&self) -> Option<&Session> {
        self._session
    }

    fn set_session(&mut self, session: &'a Session) {
        self._session = Some(session);
    }
}

impl<'a> SystemConfig<'a> {

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, Error> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        let _ = self.fetch_children(&mut globalmetadatas)?;
        Ok(globalmetadatas)
    }
}