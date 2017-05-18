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


pub use gatewaysecurity::GatewaySecurity;
pub use patnatpool::PATNATPool;
pub use permission::Permission;
pub use metadata::Metadata;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use infrastructureconfig::InfrastructureConfig;
pub use enterprisepermission::EnterprisePermission;
pub use job::Job;
pub use location::Location;
pub use monitorscope::Monitorscope;
pub use bootstrap::Bootstrap;
pub use bootstrapactivation::BootstrapActivation;
pub use uplinkconnection::UplinkConnection;
pub use nsginfo::NSGInfo;
pub use nsport::NSPort;
pub use subnet::Subnet;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default)]
pub struct NSGateway<'a> {
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

    
    #[serde(rename="MACAddress")]
    pub mac_address: Option<String>,
    
    #[serde(rename="NATTraversalEnabled")]
    pub nat_traversal_enabled: bool,
    
    #[serde(rename="TCPMSSEnabled")]
    pub tcpmss_enabled: bool,
    
    #[serde(rename="TCPMaximumSegmentSize")]
    pub tcp_maximum_segment_size: u64,
    
    #[serde(rename="SKU")]
    pub sku: Option<String>,
    
    #[serde(rename="TPMStatus")]
    pub tpm_status: Option<String>,
    
    #[serde(rename="CPUType")]
    pub cpu_type: Option<String>,
    
    #[serde(rename="NSGVersion")]
    pub nsg_version: Option<String>,
    
    #[serde(rename="SSHService")]
    pub ssh_service: Option<String>,
    
    #[serde(rename="UUID")]
    pub uuid: Option<String>,
    
    pub name: Option<String>,
    
    pub family: Option<String>,
    
    #[serde(rename="lastConfigurationReloadTimestamp")]
    pub last_configuration_reload_timestamp: u64,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="datapathID")]
    pub datapath_id: Option<String>,
    
    #[serde(rename="redundancyGroupID")]
    pub redundancy_group_id: Option<String>,
    
    #[serde(rename="templateID")]
    pub template_id: Option<String>,
    
    pub pending: bool,
    
    #[serde(rename="serialNumber")]
    pub serial_number: Option<String>,
    
    #[serde(rename="derivedSSHServiceState")]
    pub derived_ssh_service_state: Option<String>,
    
    #[serde(rename="permittedAction")]
    pub permitted_action: Option<String>,
    
    pub personality: Option<String>,
    
    pub description: Option<String>,
    
    pub libraries: Option<String>,
    
    #[serde(rename="inheritedSSHServiceState")]
    pub inherited_ssh_service_state: Option<String>,
    
    #[serde(rename="enterpriseID")]
    pub enterprise_id: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="locationID")]
    pub location_id: Option<String>,
    
    #[serde(rename="configurationReloadState")]
    pub configuration_reload_state: Option<String>,
    
    #[serde(rename="configurationStatus")]
    pub configuration_status: Option<String>,
    
    #[serde(rename="bootstrapID")]
    pub bootstrap_id: Option<String>,
    
    #[serde(rename="bootstrapStatus")]
    pub bootstrap_status: Option<String>,
    
    #[serde(rename="operationMode")]
    pub operation_mode: Option<String>,
    
    #[serde(rename="operationStatus")]
    pub operation_status: Option<String>,
    
    #[serde(rename="associatedGatewaySecurityID")]
    pub associated_gateway_security_id: Option<String>,
    
    #[serde(rename="associatedGatewaySecurityProfileID")]
    pub associated_gateway_security_profile_id: Option<String>,
    
    #[serde(rename="associatedNSGInfoID")]
    pub associated_nsg_info_id: Option<String>,
    
    #[serde(rename="autoDiscGatewayID")]
    pub auto_disc_gateway_id: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="systemID")]
    pub system_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for NSGateway<'a> {
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
        "nsgateway"
    }

    fn group_path() -> &'static str {
        "nsgateways"
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

impl<'a> NSGateway<'a> {

    pub fn fetch_gatewaysecurities(&self) -> Result<Vec<GatewaySecurity>, Error> {
        let mut gatewaysecurities = Vec::<GatewaySecurity>::new();
        let _ = self.fetch_children(&mut gatewaysecurities)?;
        Ok(gatewaysecurities)
    }

    pub fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, Error> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        let _ = self.fetch_children(&mut patnatpools)?;
        Ok(patnatpools)
    }

    pub fn fetch_permissions(&self) -> Result<Vec<Permission>, Error> {
        let mut permissions = Vec::<Permission>::new();
        let _ = self.fetch_children(&mut permissions)?;
        Ok(permissions)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_alarms(&self) -> Result<Vec<Alarm>, Error> {
        let mut alarms = Vec::<Alarm>::new();
        let _ = self.fetch_children(&mut alarms)?;
        Ok(alarms)
    }

    pub fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, Error> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        let _ = self.fetch_children(&mut globalmetadatas)?;
        Ok(globalmetadatas)
    }

    pub fn fetch_infraconfig(&self) -> Result<Vec<InfrastructureConfig>, Error> {
        let mut infraconfig = Vec::<InfrastructureConfig>::new();
        let _ = self.fetch_children(&mut infraconfig)?;
        Ok(infraconfig)
    }

    pub fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, Error> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        let _ = self.fetch_children(&mut enterprisepermissions)?;
        Ok(enterprisepermissions)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_locations(&self) -> Result<Vec<Location>, Error> {
        let mut locations = Vec::<Location>::new();
        let _ = self.fetch_children(&mut locations)?;
        Ok(locations)
    }

    pub fn fetch_monitorscopes(&self) -> Result<Vec<Monitorscope>, Error> {
        let mut monitorscopes = Vec::<Monitorscope>::new();
        let _ = self.fetch_children(&mut monitorscopes)?;
        Ok(monitorscopes)
    }

    pub fn fetch_bootstraps(&self) -> Result<Vec<Bootstrap>, Error> {
        let mut bootstraps = Vec::<Bootstrap>::new();
        let _ = self.fetch_children(&mut bootstraps)?;
        Ok(bootstraps)
    }

    pub fn fetch_bootstrapactivations(&self) -> Result<Vec<BootstrapActivation>, Error> {
        let mut bootstrapactivations = Vec::<BootstrapActivation>::new();
        let _ = self.fetch_children(&mut bootstrapactivations)?;
        Ok(bootstrapactivations)
    }

    pub fn fetch_uplinkconnections(&self) -> Result<Vec<UplinkConnection>, Error> {
        let mut uplinkconnections = Vec::<UplinkConnection>::new();
        let _ = self.fetch_children(&mut uplinkconnections)?;
        Ok(uplinkconnections)
    }

    pub fn fetch_nsginfos(&self) -> Result<Vec<NSGInfo>, Error> {
        let mut nsginfos = Vec::<NSGInfo>::new();
        let _ = self.fetch_children(&mut nsginfos)?;
        Ok(nsginfos)
    }

    pub fn fetch_nsports(&self) -> Result<Vec<NSPort>, Error> {
        let mut nsports = Vec::<NSPort>::new();
        let _ = self.fetch_children(&mut nsports)?;
        Ok(nsports)
    }

    pub fn fetch_subnets(&self) -> Result<Vec<Subnet>, Error> {
        let mut subnets = Vec::<Subnet>::new();
        let _ = self.fetch_children(&mut subnets)?;
        Ok(subnets)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}