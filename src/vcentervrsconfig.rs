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
pub use vrsaddressrange::VRSAddressRange;
pub use vrsredeploymentpolicy::VRSRedeploymentpolicy;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VCenterVRSConfig<'a> {
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

    
    #[serde(rename="vRequireNuageMetadata")]
    pub v_require_nuage_metadata: bool,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="dataDNS1")]
    pub data_dns1: Option<String>,
    
    #[serde(rename="dataDNS2")]
    pub data_dns2: Option<String>,
    
    #[serde(rename="dataGateway")]
    pub data_gateway: Option<String>,
    
    #[serde(rename="dataNetworkPortgroup")]
    pub data_network_portgroup: Option<String>,
    
    #[serde(rename="datapathSyncTimeout")]
    pub datapath_sync_timeout: u64,
    
    #[serde(rename="secondaryNuageController")]
    pub secondary_nuage_controller: Option<String>,
    
    #[serde(rename="genericSplitActivation")]
    pub generic_split_activation: bool,
    
    #[serde(rename="separateDataNetwork")]
    pub separate_data_network: bool,
    
    pub personality: Option<String>,
    
    #[serde(rename="metadataServerIP")]
    pub metadata_server_ip: Option<String>,
    
    #[serde(rename="metadataServerListenPort")]
    pub metadata_server_listen_port: u64,
    
    #[serde(rename="metadataServerPort")]
    pub metadata_server_port: u64,
    
    #[serde(rename="metadataServiceEnabled")]
    pub metadata_service_enabled: bool,
    
    #[serde(rename="networkUplinkInterface")]
    pub network_uplink_interface: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceGateway")]
    pub network_uplink_interface_gateway: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceIp")]
    pub network_uplink_interface_ip: Option<String>,
    
    #[serde(rename="networkUplinkInterfaceNetmask")]
    pub network_uplink_interface_netmask: Option<String>,
    
    #[serde(rename="nfsLogServer")]
    pub nfs_log_server: Option<String>,
    
    #[serde(rename="nfsMountPath")]
    pub nfs_mount_path: Option<String>,
    
    #[serde(rename="mgmtDNS1")]
    pub mgmt_dns1: Option<String>,
    
    #[serde(rename="mgmtDNS2")]
    pub mgmt_dns2: Option<String>,
    
    #[serde(rename="mgmtGateway")]
    pub mgmt_gateway: Option<String>,
    
    #[serde(rename="mgmtNetworkPortgroup")]
    pub mgmt_network_portgroup: Option<String>,
    
    #[serde(rename="dhcpRelayServer")]
    pub dhcp_relay_server: Option<String>,
    
    #[serde(rename="siteId")]
    pub site_id: Option<String>,
    
    #[serde(rename="allowDataDHCP")]
    pub allow_data_dhcp: bool,
    
    #[serde(rename="allowMgmtDHCP")]
    pub allow_mgmt_dhcp: bool,
    
    #[serde(rename="flowEvictionThreshold")]
    pub flow_eviction_threshold: u64,
    
    #[serde(rename="vmNetworkPortgroup")]
    pub vm_network_portgroup: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="portgroupMetadata")]
    pub portgroup_metadata: bool,
    
    #[serde(rename="novaClientVersion")]
    pub nova_client_version: u64,
    
    #[serde(rename="novaMetadataServiceAuthUrl")]
    pub nova_metadata_service_auth_url: Option<String>,
    
    #[serde(rename="novaMetadataServiceEndpoint")]
    pub nova_metadata_service_endpoint: Option<String>,
    
    #[serde(rename="novaMetadataServicePassword")]
    pub nova_metadata_service_password: Option<String>,
    
    #[serde(rename="novaMetadataServiceTenant")]
    pub nova_metadata_service_tenant: Option<String>,
    
    #[serde(rename="novaMetadataServiceUsername")]
    pub nova_metadata_service_username: Option<String>,
    
    #[serde(rename="novaMetadataSharedSecret")]
    pub nova_metadata_shared_secret: Option<String>,
    
    #[serde(rename="novaRegionName")]
    pub nova_region_name: Option<String>,
    
    #[serde(rename="primaryNuageController")]
    pub primary_nuage_controller: Option<String>,
    
    #[serde(rename="vrsPassword")]
    pub vrs_password: Option<String>,
    
    #[serde(rename="vrsUserName")]
    pub vrs_user_name: Option<String>,
    
    #[serde(rename="staticRoute")]
    pub static_route: Option<String>,
    
    #[serde(rename="staticRouteGateway")]
    pub static_route_gateway: Option<String>,
    
    #[serde(rename="staticRouteNetmask")]
    pub static_route_netmask: Option<String>,
    
    #[serde(rename="ntpServer1")]
    pub ntp_server1: Option<String>,
    
    #[serde(rename="ntpServer2")]
    pub ntp_server2: Option<String>,
    
    pub mtu: u64,
    
    #[serde(rename="multiVMSsupport")]
    pub multi_vmssupport: bool,
    
    #[serde(rename="multicastReceiveInterface")]
    pub multicast_receive_interface: Option<String>,
    
    #[serde(rename="multicastReceiveInterfaceIP")]
    pub multicast_receive_interface_ip: Option<String>,
    
    #[serde(rename="multicastReceiveInterfaceNetmask")]
    pub multicast_receive_interface_netmask: Option<String>,
    
    #[serde(rename="multicastReceiveRange")]
    pub multicast_receive_range: Option<String>,
    
    #[serde(rename="multicastSendInterface")]
    pub multicast_send_interface: Option<String>,
    
    #[serde(rename="multicastSendInterfaceIP")]
    pub multicast_send_interface_ip: Option<String>,
    
    #[serde(rename="multicastSendInterfaceNetmask")]
    pub multicast_send_interface_netmask: Option<String>,
    
    #[serde(rename="multicastSourcePortgroup")]
    pub multicast_source_portgroup: Option<String>,
    
    #[serde(rename="customizedScriptURL")]
    pub customized_script_url: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for VCenterVRSConfig<'a> {
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
        "vrsconfig"
    }

    fn group_path() -> &'static str {
        "vrsconfigs"
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

impl<'a> VCenterVRSConfig<'a> {

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

    pub fn fetch_vrsaddressranges(&self) -> Result<Vec<VRSAddressRange>, Error> {
        let mut vrsaddressranges = Vec::<VRSAddressRange>::new();
        let _ = self.fetch_children(&mut vrsaddressranges)?;
        Ok(vrsaddressranges)
    }

    pub fn fetch_vrsredeploymentpolicies(&self) -> Result<Vec<VRSRedeploymentpolicy>, Error> {
        let mut vrsredeploymentpolicies = Vec::<VRSRedeploymentpolicy>::new();
        let _ = self.fetch_children(&mut vrsredeploymentpolicies)?;
        Ok(vrsredeploymentpolicies)
    }
}