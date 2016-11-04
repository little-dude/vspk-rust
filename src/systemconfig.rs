// Copyright (c) 2015-2016, Nokia Inc
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


use bambou::{BambouError, RestEntity, Session, SessionConfig};
use hyper::client::{Response};
use std::collections::BTreeMap;
use serde_json;


pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;


#[derive(Serialize, Deserialize)]
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
    acl_allow_origin: Option<String>,
    
    #[serde(rename="ECMPCount")]
    ecmp_count: u64,
    
    #[serde(rename="LDAPSyncInterval")]
    ldap_sync_interval: u64,
    
    #[serde(rename="LDAPTrustStoreCertifcate")]
    ldap_trust_store_certifcate: Option<String>,
    
    #[serde(rename="LDAPTrustStorePassword")]
    ldap_trust_store_password: Option<String>,
    
    #[serde(rename="ADGatewayPurgeTime")]
    ad_gateway_purge_time: u64,
    
    #[serde(rename="RDLowerLimit")]
    rd_lower_limit: u64,
    
    #[serde(rename="RDPublicNetworkLowerLimit")]
    rd_public_network_lower_limit: u64,
    
    #[serde(rename="RDPublicNetworkUpperLimit")]
    rd_public_network_upper_limit: u64,
    
    #[serde(rename="RDUpperLimit")]
    rd_upper_limit: u64,
    
    #[serde(rename="ZFBBootstrapEnabled")]
    zfb_bootstrap_enabled: bool,
    
    #[serde(rename="ZFBRequestRetryTimer")]
    zfb_request_retry_timer: u64,
    
    #[serde(rename="ZFBSchedulerStaleRequestTimeout")]
    zfb_scheduler_stale_request_timeout: u64,
    
    #[serde(rename="DHCPOptionSize")]
    dhcp_option_size: u64,
    
    #[serde(rename="VMCacheSize")]
    vm_cache_size: u64,
    
    #[serde(rename="VMPurgeTime")]
    vm_purge_time: u64,
    
    #[serde(rename="VMResyncDeletionWaitTime")]
    vm_resync_deletion_wait_time: u64,
    
    #[serde(rename="VMResyncOutstandingInterval")]
    vm_resync_outstanding_interval: u64,
    
    #[serde(rename="VMUnreachableCleanupTime")]
    vm_unreachable_cleanup_time: u64,
    
    #[serde(rename="VMUnreachableTime")]
    vm_unreachable_time: u64,
    
    #[serde(rename="VNIDLowerLimit")]
    vnid_lower_limit: u64,
    
    #[serde(rename="VNIDPublicNetworkLowerLimit")]
    vnid_public_network_lower_limit: u64,
    
    #[serde(rename="VNIDPublicNetworkUpperLimit")]
    vnid_public_network_upper_limit: u64,
    
    #[serde(rename="VNIDUpperLimit")]
    vnid_upper_limit: u64,
    
    #[serde(rename="APIKeyRenewalInterval")]
    api_key_renewal_interval: u64,
    
    #[serde(rename="APIKeyValidity")]
    api_key_validity: u64,
    
    #[serde(rename="VPortInitStatefulTimer")]
    vport_init_stateful_timer: u64,
    
    #[serde(rename="LRUCacheSizePerSubnet")]
    lru_cache_size_per_subnet: u64,
    
    #[serde(rename="VSCOnSameVersionAsVSD")]
    vsc_on_same_version_as_vsd: bool,
    
    #[serde(rename="VSDReadOnlyMode")]
    vsd_read_only_mode: bool,
    
    #[serde(rename="VSDUpgradeIsComplete")]
    vsd_upgrade_is_complete: bool,
    
    #[serde(rename="ASNumber")]
    as_number: u64,
    
    #[serde(rename="RTLowerLimit")]
    rt_lower_limit: u64,
    
    #[serde(rename="RTPublicNetworkLowerLimit")]
    rt_public_network_lower_limit: u64,
    
    #[serde(rename="RTPublicNetworkUpperLimit")]
    rt_public_network_upper_limit: u64,
    
    #[serde(rename="RTUpperLimit")]
    rt_upper_limit: u64,
    
    #[serde(rename="EVPNBGPCommunityTagASNumber")]
    evpnbgp_community_tag_as_number: u64,
    
    #[serde(rename="EVPNBGPCommunityTagLowerLimit")]
    evpnbgp_community_tag_lower_limit: u64,
    
    #[serde(rename="EVPNBGPCommunityTagUpperLimit")]
    evpnbgp_community_tag_upper_limit: u64,
    
    #[serde(rename="pageMaxSize")]
    page_max_size: u64,
    
    #[serde(rename="pageSize")]
    page_size: u64,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="maxFailedLogins")]
    max_failed_logins: u64,
    
    #[serde(rename="maxResponse")]
    max_response: u64,
    
    #[serde(rename="performancePathSelectionVNID")]
    performance_path_selection_vnid: u64,
    
    #[serde(rename="serviceIDUpperLimit")]
    service_id_upper_limit: u64,
    
    #[serde(rename="keyServerMonitorEnabled")]
    key_server_monitor_enabled: bool,
    
    #[serde(rename="keyServerVSDDataSynchronizationInterval")]
    key_server_vsd_data_synchronization_interval: u64,
    
    #[serde(rename="offsetCustomerID")]
    offset_customer_id: u64,
    
    #[serde(rename="offsetServiceID")]
    offset_service_id: u64,
    
    #[serde(rename="ejbcaNSGCertificateProfile")]
    ejbca_nsg_certificate_profile: Option<String>,
    
    #[serde(rename="ejbcaNSGEndEntityProfile")]
    ejbca_nsg_end_entity_profile: Option<String>,
    
    #[serde(rename="ejbcaOCSPResponderCN")]
    ejbca_ocsp_responder_cn: Option<String>,
    
    #[serde(rename="ejbcaOCSPResponderURI")]
    ejbca_ocsp_responder_uri: Option<String>,
    
    #[serde(rename="ejbcaVspRootCa")]
    ejbca_vsp_root_ca: Option<String>,
    
    #[serde(rename="alarmsMaxPerObject")]
    alarms_max_per_object: u64,
    
    #[serde(rename="elasticClusterName")]
    elastic_cluster_name: Option<String>,
    
    #[serde(rename="elasticSearchUIAddress")]
    elastic_search_ui_address: Option<String>,
    
    #[serde(rename="allowEnterpriseAvatarOnNSG")]
    allow_enterprise_avatar_on_nsg: bool,
    
    #[serde(rename="globalMACAddress")]
    global_mac_address: Option<String>,
    
    #[serde(rename="inactiveTimeout")]
    inactive_timeout: u64,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="domainTunnelType")]
    domain_tunnel_type: Option<String>,
    
    #[serde(rename="postProcessorThreadsCount")]
    post_processor_threads_count: u64,
    
    #[serde(rename="groupKeyDefaultSEKGenerationInterval")]
    group_key_default_sek_generation_interval: u64,
    
    #[serde(rename="groupKeyDefaultSEKLifetime")]
    group_key_default_sek_lifetime: u64,
    
    #[serde(rename="groupKeyDefaultSEKPayloadEncryptionAlgorithm")]
    group_key_default_sek_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSEKPayloadSigningAlgorithm")]
    group_key_default_sek_payload_signing_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedGenerationInterval")]
    group_key_default_seed_generation_interval: u64,
    
    #[serde(rename="groupKeyDefaultSeedLifetime")]
    group_key_default_seed_lifetime: u64,
    
    #[serde(rename="groupKeyDefaultSeedPayloadAuthenticationAlgorithm")]
    group_key_default_seed_payload_authentication_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedPayloadEncryptionAlgorithm")]
    group_key_default_seed_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultSeedPayloadSigningAlgorithm")]
    group_key_default_seed_payload_signing_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficAuthenticationAlgorithm")]
    group_key_default_traffic_authentication_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficEncryptionAlgorithm")]
    group_key_default_traffic_encryption_algorithm: Option<String>,
    
    #[serde(rename="groupKeyDefaultTrafficEncryptionKeyLifetime")]
    group_key_default_traffic_encryption_key_lifetime: u64,
    
    #[serde(rename="groupKeyGenerationIntervalOnForcedReKey")]
    group_key_generation_interval_on_forced_re_key: u64,
    
    #[serde(rename="groupKeyGenerationIntervalOnRevoke")]
    group_key_generation_interval_on_revoke: u64,
    
    #[serde(rename="groupKeyMinimumSEKGenerationInterval")]
    group_key_minimum_sek_generation_interval: u64,
    
    #[serde(rename="groupKeyMinimumSEKLifetime")]
    group_key_minimum_sek_lifetime: u64,
    
    #[serde(rename="groupKeyMinimumSeedGenerationInterval")]
    group_key_minimum_seed_generation_interval: u64,
    
    #[serde(rename="groupKeyMinimumSeedLifetime")]
    group_key_minimum_seed_lifetime: u64,
    
    #[serde(rename="groupKeyMinimumTrafficEncryptionKeyLifetime")]
    group_key_minimum_traffic_encryption_key_lifetime: u64,
    
    #[serde(rename="nsgBootstrapEndpoint")]
    nsg_bootstrap_endpoint: Option<String>,
    
    #[serde(rename="nsgConfigEndpoint")]
    nsg_config_endpoint: Option<String>,
    
    #[serde(rename="nsgLocalUiUrl")]
    nsg_local_ui_url: Option<String>,
    
    #[serde(rename="esiID")]
    esi_id: u64,
    
    #[serde(rename="csprootAuthenticationMethod")]
    csproot_authentication_method: Option<String>,
    
    #[serde(rename="stackTraceEnabled")]
    stack_trace_enabled: bool,
    
    #[serde(rename="statefulACLNonTCPTimeout")]
    stateful_acl_non_tcp_timeout: u64,
    
    #[serde(rename="statefulACLTCPTimeout")]
    stateful_acltcp_timeout: u64,
    
    #[serde(rename="staticWANServicePurgeTime")]
    static_wan_service_purge_time: u64,
    
    #[serde(rename="statisticsEnabled")]
    statistics_enabled: bool,
    
    #[serde(rename="statsCollectorAddress")]
    stats_collector_address: Option<String>,
    
    #[serde(rename="statsCollectorPort")]
    stats_collector_port: Option<String>,
    
    #[serde(rename="statsCollectorProtoBufPort")]
    stats_collector_proto_buf_port: Option<String>,
    
    #[serde(rename="statsMaxDataPoints")]
    stats_max_data_points: u64,
    
    #[serde(rename="statsMinDuration")]
    stats_min_duration: u64,
    
    #[serde(rename="statsNumberOfDataPoints")]
    stats_number_of_data_points: u64,
    
    #[serde(rename="statsTSDBServerAddress")]
    stats_tsdb_server_address: Option<String>,
    
    #[serde(rename="stickyECMPIdleTimeout")]
    sticky_ecmp_idle_timeout: u64,
    
    #[serde(rename="subnetResyncInterval")]
    subnet_resync_interval: u64,
    
    #[serde(rename="subnetResyncOutstandingInterval")]
    subnet_resync_outstanding_interval: u64,
    
    #[serde(rename="customerIDUpperLimit")]
    customer_id_upper_limit: u64,
    
    #[serde(rename="customerKey")]
    customer_key: Option<String>,
    
    #[serde(rename="avatarBasePath")]
    avatar_base_path: Option<String>,
    
    #[serde(rename="avatarBaseURL")]
    avatar_base_url: Option<String>,
    
    #[serde(rename="eventLogCleanupInterval")]
    event_log_cleanup_interval: u64,
    
    #[serde(rename="eventLogEntryMaxAge")]
    event_log_entry_max_age: u64,
    
    #[serde(rename="eventProcessorInterval")]
    event_processor_interval: u64,
    
    #[serde(rename="eventProcessorMaxEventsCount")]
    event_processor_max_events_count: u64,
    
    #[serde(rename="eventProcessorTimeout")]
    event_processor_timeout: u64,
    
    #[serde(rename="twoFactorCodeExpiry")]
    two_factor_code_expiry: u64,
    
    #[serde(rename="twoFactorCodeLength")]
    two_factor_code_length: u64,
    
    #[serde(rename="twoFactorCodeSeedLength")]
    two_factor_code_seed_length: u64,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
    #[serde(rename="dynamicWANServiceDiffTime")]
    dynamic_wan_service_diff_time: u64,
    
    #[serde(rename="syslogDestinationHost")]
    syslog_destination_host: Option<String>,
    
    #[serde(rename="syslogDestinationPort")]
    syslog_destination_port: u64,
    
    #[serde(rename="sysmonCleanupTaskInterval")]
    sysmon_cleanup_task_interval: u64,
    
    #[serde(rename="sysmonNodePresenceTimeout")]
    sysmon_node_presence_timeout: u64,
    
    #[serde(rename="sysmonProbeResponseTimeout")]
    sysmon_probe_response_timeout: u64,
    
    #[serde(rename="systemAvatarData")]
    system_avatar_data: Option<String>,
    
    #[serde(rename="systemAvatarType")]
    system_avatar_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for SystemConfig<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
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

    fn fetch_children<R>(&self, children: &mut Vec<R>) -> Result<Response, BambouError>
        where R: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.fetch_children(self, children),
            None => Err(BambouError::NoSession),
        }
    }

    fn get_session(&self) -> Option<&Session> {
        self._session
    }

    fn set_session(&mut self, session: &'a Session) {
        self._session = Some(session);
    }

    fn save(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.save(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn delete(self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.delete(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn create_child<C>(&self, child: &mut C) -> Result<Response, BambouError>
        where C: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.create_child(self, child),
            None => Err(BambouError::NoSession),
        }
    }

}

impl<'a> SystemConfig<'a> {

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, BambouError> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        try!(self.fetch_children(&mut globalmetadatas));
        Ok(globalmetadatas)
    }
}
