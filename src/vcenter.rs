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


pub use vcenterdatacenter::VCenterDataCenter;
pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;
pub use job::Job;
pub use vrsaddressrange::VRSAddressRange;
pub use vrsredeploymentpolicy::VRSRedeploymentpolicy;
pub use autodiscovereddatacenter::Autodiscovereddatacenter;


#[derive(Serialize, Deserialize)]
pub struct VCenter<'a> {
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
    
    #[serde(rename="VRSConfigurationTime")]
    vrs_configuration_time: u64,
    
    #[serde(rename="vRequireNuageMetadata")]
    v_require_nuage_metadata: bool,
    name: Option<String>,
    password: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="dataDNS1")]
    data_dns1: Option<String>,
    
    #[serde(rename="dataDNS2")]
    data_dns2: Option<String>,
    
    #[serde(rename="dataGateway")]
    data_gateway: Option<String>,
    
    #[serde(rename="dataNetworkPortgroup")]
    data_network_portgroup: Option<String>,
    
    #[serde(rename="datapathSyncTimeout")]
    datapath_sync_timeout: u64,
    
    #[serde(rename="secondaryNuageController")]
    secondary_nuage_controller: Option<String>,
    
    #[serde(rename="genericSplitActivation")]
    generic_split_activation: bool,
    
    #[serde(rename="separateDataNetwork")]
    separate_data_network: bool,
    personality: Option<String>,
    description: Option<String>,
    
    #[serde(rename="destinationMirrorPort")]
    destination_mirror_port: Option<String>,
    
    #[serde(rename="metadataServerIP")]
    metadata_server_ip: Option<String>,
    
    #[serde(rename="metadataServerListenPort")]
    metadata_server_listen_port: u64,
    
    #[serde(rename="metadataServerPort")]
    metadata_server_port: u64,
    
    #[serde(rename="metadataServiceEnabled")]
    metadata_service_enabled: bool,
    
    #[serde(rename="networkUplinkInterface")]
    network_uplink_interface: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceGateway")]
    network_uplink_interface_gateway: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceIp")]
    network_uplink_interface_ip: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceNetmask")]
    network_uplink_interface_netmask: Option<String>,
    
    #[serde(rename="nfsLogServer")]
    nfs_log_server: Option<String>,
    
    #[serde(rename="nfsMountPath")]
    nfs_mount_path: Option<String>,
    
    #[serde(rename="mgmtDNS1")]
    mgmt_dns1: Option<String>,
    
    #[serde(rename="mgmtDNS2")]
    mgmt_dns2: Option<String>,
    
    #[serde(rename="mgmtGateway")]
    mgmt_gateway: Option<String>,
    
    #[serde(rename="mgmtNetworkPortgroup")]
    mgmt_network_portgroup: Option<String>,
    
    #[serde(rename="dhcpRelayServer")]
    dhcp_relay_server: Option<String>,
    
    #[serde(rename="mirrorNetworkPortgroup")]
    mirror_network_portgroup: Option<String>,
    
    #[serde(rename="siteId")]
    site_id: Option<String>,
    
    #[serde(rename="allowDataDHCP")]
    allow_data_dhcp: bool,
    
    #[serde(rename="allowMgmtDHCP")]
    allow_mgmt_dhcp: bool,
    
    #[serde(rename="flowEvictionThreshold")]
    flow_eviction_threshold: u64,
    
    #[serde(rename="vmNetworkPortgroup")]
    vm_network_portgroup: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="connectionStatus")]
    connection_status: bool,
    
    #[serde(rename="portgroupMetadata")]
    portgroup_metadata: bool,
    
    #[serde(rename="hostLevelManagement")]
    host_level_management: bool,
    
    #[serde(rename="novaClientVersion")]
    nova_client_version: u64,
    
    #[serde(rename="novaMetadataServiceAuthUrl")]
    nova_metadata_service_auth_url: Option<String>,
    
    #[serde(rename="novaMetadataServiceEndpoint")]
    nova_metadata_service_endpoint: Option<String>,
    
    #[serde(rename="novaMetadataServicePassword")]
    nova_metadata_service_password: Option<String>,
    
    #[serde(rename="novaMetadataServiceTenant")]
    nova_metadata_service_tenant: Option<String>,
    
    #[serde(rename="novaMetadataServiceUsername")]
    nova_metadata_service_username: Option<String>,
    
    #[serde(rename="novaMetadataSharedSecret")]
    nova_metadata_shared_secret: Option<String>,
    
    #[serde(rename="novaRegionName")]
    nova_region_name: Option<String>,
    
    #[serde(rename="ipAddress")]
    ip_address: Option<String>,
    
    #[serde(rename="primaryNuageController")]
    primary_nuage_controller: Option<String>,
    
    #[serde(rename="vrsConfigID")]
    vrs_config_id: Option<String>,
    
    #[serde(rename="vrsPassword")]
    vrs_password: Option<String>,
    
    #[serde(rename="vrsUserName")]
    vrs_user_name: Option<String>,
    
    #[serde(rename="userName")]
    user_name: Option<String>,
    
    #[serde(rename="staticRoute")]
    static_route: Option<String>,
    
    #[serde(rename="staticRouteGateway")]
    static_route_gateway: Option<String>,
    
    #[serde(rename="staticRouteNetmask")]
    static_route_netmask: Option<String>,
    
    #[serde(rename="ntpServer1")]
    ntp_server1: Option<String>,
    
    #[serde(rename="ntpServer2")]
    ntp_server2: Option<String>,
    
    #[serde(rename="httpPort")]
    http_port: u64,
    
    #[serde(rename="httpsPort")]
    https_port: u64,
    mtu: u64,
    
    #[serde(rename="multiVMSsupport")]
    multi_vmssupport: bool,
    
    #[serde(rename="multicastReceiveInterface")]
    multicast_receive_interface: Option<String>,
    
    #[serde(rename="multicastReceiveInterfaceIP")]
    multicast_receive_interface_ip: Option<String>,
    
    #[serde(rename="multicastReceiveInterfaceNetmask")]
    multicast_receive_interface_netmask: Option<String>,
    
    #[serde(rename="multicastReceiveRange")]
    multicast_receive_range: Option<String>,
    
    #[serde(rename="multicastSendInterface")]
    multicast_send_interface: Option<String>,
    
    #[serde(rename="multicastSendInterfaceIP")]
    multicast_send_interface_ip: Option<String>,
    
    #[serde(rename="multicastSendInterfaceNetmask")]
    multicast_send_interface_netmask: Option<String>,
    
    #[serde(rename="multicastSourcePortgroup")]
    multicast_source_portgroup: Option<String>,
    
    #[serde(rename="customizedScriptURL")]
    customized_script_url: Option<String>,
    
    #[serde(rename="autoResolveFrequency")]
    auto_resolve_frequency: u64,
    
    #[serde(rename="ovfURL")]
    ovf_url: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for VCenter<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "vcenter"
    }

    fn group_path() -> &'static str {
        "vcenters"
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

impl<'a> VCenter<'a> {

    fn fetch_vcenterdatacenters(&self) -> Result<Vec<VCenterDataCenter>, BambouError> {
        let mut vcenterdatacenters = Vec::<VCenterDataCenter>::new();
        try!(self.fetch_children(&mut vcenterdatacenters));
        Ok(vcenterdatacenters)
    }

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

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_vrsaddressranges(&self) -> Result<Vec<VRSAddressRange>, BambouError> {
        let mut vrsaddressranges = Vec::<VRSAddressRange>::new();
        try!(self.fetch_children(&mut vrsaddressranges));
        Ok(vrsaddressranges)
    }

    fn fetch_vrsredeploymentpolicies(&self) -> Result<Vec<VRSRedeploymentpolicy>, BambouError> {
        let mut vrsredeploymentpolicies = Vec::<VRSRedeploymentpolicy>::new();
        try!(self.fetch_children(&mut vrsredeploymentpolicies));
        Ok(vrsredeploymentpolicies)
    }

    fn fetch_autodiscovereddatacenters(&self) -> Result<Vec<Autodiscovereddatacenter>, BambouError> {
        let mut autodiscovereddatacenters = Vec::<Autodiscovereddatacenter>::new();
        try!(self.fetch_children(&mut autodiscovereddatacenters));
        Ok(autodiscovereddatacenters)
    }
}
