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
pub use nsginfo::NSGInfo;
pub use nsport::NSPort;
pub use subnet::Subnet;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
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
    mac_address: Option<String>,
    
    #[serde(rename="NATTraversalEnabled")]
    nat_traversal_enabled: bool,
    
    #[serde(rename="SKU")]
    sku: Option<String>,
    
    #[serde(rename="TPMStatus")]
    tpm_status: Option<String>,
    
    #[serde(rename="CPUType")]
    cpu_type: Option<String>,
    
    #[serde(rename="NSGVersion")]
    nsg_version: Option<String>,
    
    #[serde(rename="SSHService")]
    ssh_service: Option<String>,
    
    #[serde(rename="UUID")]
    uuid: Option<String>,
    name: Option<String>,
    family: Option<String>,
    
    #[serde(rename="lastConfigurationReloadTimestamp")]
    last_configuration_reload_timestamp: u64,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="datapathID")]
    datapath_id: Option<String>,
    
    #[serde(rename="redundancyGroupID")]
    redundancy_group_id: Option<String>,
    
    #[serde(rename="templateID")]
    template_id: Option<String>,
    pending: bool,
    
    #[serde(rename="serialNumber")]
    serial_number: Option<String>,
    
    #[serde(rename="permittedAction")]
    permitted_action: Option<String>,
    personality: Option<String>,
    description: Option<String>,
    libraries: Option<String>,
    
    #[serde(rename="inheritedSSHServiceState")]
    inherited_ssh_service_state: Option<String>,
    
    #[serde(rename="enterpriseID")]
    enterprise_id: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="locationID")]
    location_id: Option<String>,
    
    #[serde(rename="configurationReloadState")]
    configuration_reload_state: Option<String>,
    
    #[serde(rename="configurationStatus")]
    configuration_status: Option<String>,
    
    #[serde(rename="bootstrapID")]
    bootstrap_id: Option<String>,
    
    #[serde(rename="bootstrapStatus")]
    bootstrap_status: Option<String>,
    
    #[serde(rename="associatedGatewaySecurityID")]
    associated_gateway_security_id: Option<String>,
    
    #[serde(rename="associatedGatewaySecurityProfileID")]
    associated_gateway_security_profile_id: Option<String>,
    
    #[serde(rename="associatedNSGInfoID")]
    associated_nsg_info_id: Option<String>,
    
    #[serde(rename="autoDiscGatewayID")]
    auto_disc_gateway_id: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
    #[serde(rename="systemID")]
    system_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for NSGateway<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
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

impl<'a> NSGateway<'a> {

    fn fetch_gatewaysecurities(&self) -> Result<Vec<GatewaySecurity>, BambouError> {
        let mut gatewaysecurities = Vec::<GatewaySecurity>::new();
        try!(self.fetch_children(&mut gatewaysecurities));
        Ok(gatewaysecurities)
    }

    fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, BambouError> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        try!(self.fetch_children(&mut patnatpools));
        Ok(patnatpools)
    }

    fn fetch_permissions(&self) -> Result<Vec<Permission>, BambouError> {
        let mut permissions = Vec::<Permission>::new();
        try!(self.fetch_children(&mut permissions));
        Ok(permissions)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_alarms(&self) -> Result<Vec<Alarm>, BambouError> {
        let mut alarms = Vec::<Alarm>::new();
        try!(self.fetch_children(&mut alarms));
        Ok(alarms)
    }

    fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, BambouError> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        try!(self.fetch_children(&mut globalmetadatas));
        Ok(globalmetadatas)
    }

    fn fetch_infraconfig(&self) -> Result<Vec<InfrastructureConfig>, BambouError> {
        let mut infraconfig = Vec::<InfrastructureConfig>::new();
        try!(self.fetch_children(&mut infraconfig));
        Ok(infraconfig)
    }

    fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, BambouError> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        try!(self.fetch_children(&mut enterprisepermissions));
        Ok(enterprisepermissions)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_locations(&self) -> Result<Vec<Location>, BambouError> {
        let mut locations = Vec::<Location>::new();
        try!(self.fetch_children(&mut locations));
        Ok(locations)
    }

    fn fetch_monitorscopes(&self) -> Result<Vec<Monitorscope>, BambouError> {
        let mut monitorscopes = Vec::<Monitorscope>::new();
        try!(self.fetch_children(&mut monitorscopes));
        Ok(monitorscopes)
    }

    fn fetch_bootstraps(&self) -> Result<Vec<Bootstrap>, BambouError> {
        let mut bootstraps = Vec::<Bootstrap>::new();
        try!(self.fetch_children(&mut bootstraps));
        Ok(bootstraps)
    }

    fn fetch_bootstrapactivations(&self) -> Result<Vec<BootstrapActivation>, BambouError> {
        let mut bootstrapactivations = Vec::<BootstrapActivation>::new();
        try!(self.fetch_children(&mut bootstrapactivations));
        Ok(bootstrapactivations)
    }

    fn fetch_nsginfos(&self) -> Result<Vec<NSGInfo>, BambouError> {
        let mut nsginfos = Vec::<NSGInfo>::new();
        try!(self.fetch_children(&mut nsginfos));
        Ok(nsginfos)
    }

    fn fetch_nsports(&self) -> Result<Vec<NSPort>, BambouError> {
        let mut nsports = Vec::<NSPort>::new();
        try!(self.fetch_children(&mut nsports));
        Ok(nsports)
    }

    fn fetch_subnets(&self) -> Result<Vec<Subnet>, BambouError> {
        let mut subnets = Vec::<Subnet>::new();
        try!(self.fetch_children(&mut subnets));
        Ok(subnets)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
