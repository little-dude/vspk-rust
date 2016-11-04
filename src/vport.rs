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


pub use tca::TCA;
pub use redirectiontarget::RedirectionTarget;
pub use metadata::Metadata;
pub use aggregatemetadata::AggregateMetadata;
pub use dhcpoption::DHCPOption;
pub use virtualip::VirtualIP;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use vminterface::VMInterface;
pub use policygroup::PolicyGroup;
pub use container::Container;
pub use containerinterface::ContainerInterface;
pub use portmapping::PortMapping;
pub use qos::QOS;
pub use hostinterface::HostInterface;
pub use vportmirror::VPortMirror;
pub use applicationperformancemanagement::Applicationperformancemanagement;
pub use bridgeinterface::BridgeInterface;
pub use vrs::VRS;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct VPort<'a> {
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
    
    #[serde(rename="VLANID")]
    vlanid: Option<String>,
    
    #[serde(rename="DPI")]
    dpi: Option<String>,
    name: Option<String>,
    
    #[serde(rename="hasAttachedInterfaces")]
    has_attached_interfaces: bool,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    active: bool,
    
    #[serde(rename="addressSpoofing")]
    address_spoofing: Option<String>,
    description: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="domainID")]
    domain_id: Option<String>,
    
    #[serde(rename="zoneID")]
    zone_id: Option<String>,
    
    #[serde(rename="operationalState")]
    operational_state: Option<String>,
    
    #[serde(rename="associatedFloatingIPID")]
    associated_floating_ip_id: Option<String>,
    
    #[serde(rename="associatedMulticastChannelMapID")]
    associated_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="associatedSendMulticastChannelMapID")]
    associated_send_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="multiNICVPortID")]
    multi_nic_vport_id: Option<String>,
    multicast: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
    #[serde(rename="type")]
    type_: Option<String>,
    
    #[serde(rename="systemType")]
    system_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for VPort<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "vport"
    }

    fn group_path() -> &'static str {
        "vports"
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

impl<'a> VPort<'a> {

    fn fetch_tcas(&self) -> Result<Vec<TCA>, BambouError> {
        let mut tcas = Vec::<TCA>::new();
        try!(self.fetch_children(&mut tcas));
        Ok(tcas)
    }

    fn fetch_redirectiontargets(&self) -> Result<Vec<RedirectionTarget>, BambouError> {
        let mut redirectiontargets = Vec::<RedirectionTarget>::new();
        try!(self.fetch_children(&mut redirectiontargets));
        Ok(redirectiontargets)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_aggregatemetadatas(&self) -> Result<Vec<AggregateMetadata>, BambouError> {
        let mut aggregatemetadatas = Vec::<AggregateMetadata>::new();
        try!(self.fetch_children(&mut aggregatemetadatas));
        Ok(aggregatemetadatas)
    }

    fn fetch_dhcpoptions(&self) -> Result<Vec<DHCPOption>, BambouError> {
        let mut dhcpoptions = Vec::<DHCPOption>::new();
        try!(self.fetch_children(&mut dhcpoptions));
        Ok(dhcpoptions)
    }

    fn fetch_virtualips(&self) -> Result<Vec<VirtualIP>, BambouError> {
        let mut virtualips = Vec::<VirtualIP>::new();
        try!(self.fetch_children(&mut virtualips));
        Ok(virtualips)
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

    fn fetch_vms(&self) -> Result<Vec<VM>, BambouError> {
        let mut vms = Vec::<VM>::new();
        try!(self.fetch_children(&mut vms));
        Ok(vms)
    }

    fn fetch_vminterfaces(&self) -> Result<Vec<VMInterface>, BambouError> {
        let mut vminterfaces = Vec::<VMInterface>::new();
        try!(self.fetch_children(&mut vminterfaces));
        Ok(vminterfaces)
    }

    fn fetch_policygroups(&self) -> Result<Vec<PolicyGroup>, BambouError> {
        let mut policygroups = Vec::<PolicyGroup>::new();
        try!(self.fetch_children(&mut policygroups));
        Ok(policygroups)
    }

    fn fetch_containers(&self) -> Result<Vec<Container>, BambouError> {
        let mut containers = Vec::<Container>::new();
        try!(self.fetch_children(&mut containers));
        Ok(containers)
    }

    fn fetch_containerinterfaces(&self) -> Result<Vec<ContainerInterface>, BambouError> {
        let mut containerinterfaces = Vec::<ContainerInterface>::new();
        try!(self.fetch_children(&mut containerinterfaces));
        Ok(containerinterfaces)
    }

    fn fetch_portmappings(&self) -> Result<Vec<PortMapping>, BambouError> {
        let mut portmappings = Vec::<PortMapping>::new();
        try!(self.fetch_children(&mut portmappings));
        Ok(portmappings)
    }

    fn fetch_qos(&self) -> Result<Vec<QOS>, BambouError> {
        let mut qos = Vec::<QOS>::new();
        try!(self.fetch_children(&mut qos));
        Ok(qos)
    }

    fn fetch_hostinterfaces(&self) -> Result<Vec<HostInterface>, BambouError> {
        let mut hostinterfaces = Vec::<HostInterface>::new();
        try!(self.fetch_children(&mut hostinterfaces));
        Ok(hostinterfaces)
    }

    fn fetch_vportmirrors(&self) -> Result<Vec<VPortMirror>, BambouError> {
        let mut vportmirrors = Vec::<VPortMirror>::new();
        try!(self.fetch_children(&mut vportmirrors));
        Ok(vportmirrors)
    }

    fn fetch_applicationperformancemanagements(&self) -> Result<Vec<Applicationperformancemanagement>, BambouError> {
        let mut applicationperformancemanagements = Vec::<Applicationperformancemanagement>::new();
        try!(self.fetch_children(&mut applicationperformancemanagements));
        Ok(applicationperformancemanagements)
    }

    fn fetch_bridgeinterfaces(&self) -> Result<Vec<BridgeInterface>, BambouError> {
        let mut bridgeinterfaces = Vec::<BridgeInterface>::new();
        try!(self.fetch_children(&mut bridgeinterfaces));
        Ok(bridgeinterfaces)
    }

    fn fetch_vrss(&self) -> Result<Vec<VRS>, BambouError> {
        let mut vrss = Vec::<VRS>::new();
        try!(self.fetch_children(&mut vrss));
        Ok(vrss)
    }

    fn fetch_statistics(&self) -> Result<Vec<Statistics>, BambouError> {
        let mut statistics = Vec::<Statistics>::new();
        try!(self.fetch_children(&mut statistics));
        Ok(statistics)
    }

    fn fetch_statisticspolicies(&self) -> Result<Vec<StatisticsPolicy>, BambouError> {
        let mut statisticspolicies = Vec::<StatisticsPolicy>::new();
        try!(self.fetch_children(&mut statisticspolicies));
        Ok(statisticspolicies)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
