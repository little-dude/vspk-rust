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


pub use tca::TCA;
pub use redirectiontarget::RedirectionTarget;
pub use metadata::Metadata;
pub use aggregatemetadata::AggregateMetadata;
pub use bgpneighbor::BGPNeighbor;
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
pub use trunk::Trunk;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
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
    pub vlanid: Option<String>,
    
    #[serde(rename="DPI")]
    pub dpi: Option<String>,
    
    pub name: Option<String>,
    
    #[serde(rename="hasAttachedInterfaces")]
    pub has_attached_interfaces: bool,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub active: bool,
    
    #[serde(rename="addressSpoofing")]
    pub address_spoofing: Option<String>,
    
    #[serde(rename="segmentationID")]
    pub segmentation_id: u64,
    
    #[serde(rename="segmentationType")]
    pub segmentation_type: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="domainID")]
    pub domain_id: Option<String>,
    
    #[serde(rename="zoneID")]
    pub zone_id: Option<String>,
    
    #[serde(rename="operationalState")]
    pub operational_state: Option<String>,
    
    #[serde(rename="trunkRole")]
    pub trunk_role: Option<String>,
    
    #[serde(rename="associatedFloatingIPID")]
    pub associated_floating_ip_id: Option<String>,
    
    #[serde(rename="associatedMulticastChannelMapID")]
    pub associated_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="associatedSendMulticastChannelMapID")]
    pub associated_send_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="associatedTrunkID")]
    pub associated_trunk_id: Option<String>,
    
    #[serde(rename="multiNICVPortID")]
    pub multi_nic_vport_id: Option<String>,
    
    pub multicast: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="type")]
    pub type_: Option<String>,
    
    #[serde(rename="systemType")]
    pub system_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for VPort<'a> {
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

impl<'a> VPort<'a> {

    pub fn fetch_tcas(&self) -> Result<Vec<TCA>, Error> {
        let mut tcas = Vec::<TCA>::new();
        let _ = self.fetch_children(&mut tcas)?;
        Ok(tcas)
    }

    pub fn fetch_redirectiontargets(&self) -> Result<Vec<RedirectionTarget>, Error> {
        let mut redirectiontargets = Vec::<RedirectionTarget>::new();
        let _ = self.fetch_children(&mut redirectiontargets)?;
        Ok(redirectiontargets)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_aggregatemetadatas(&self) -> Result<Vec<AggregateMetadata>, Error> {
        let mut aggregatemetadatas = Vec::<AggregateMetadata>::new();
        let _ = self.fetch_children(&mut aggregatemetadatas)?;
        Ok(aggregatemetadatas)
    }

    pub fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, Error> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        let _ = self.fetch_children(&mut bgpneighbors)?;
        Ok(bgpneighbors)
    }

    pub fn fetch_dhcpoptions(&self) -> Result<Vec<DHCPOption>, Error> {
        let mut dhcpoptions = Vec::<DHCPOption>::new();
        let _ = self.fetch_children(&mut dhcpoptions)?;
        Ok(dhcpoptions)
    }

    pub fn fetch_virtualips(&self) -> Result<Vec<VirtualIP>, Error> {
        let mut virtualips = Vec::<VirtualIP>::new();
        let _ = self.fetch_children(&mut virtualips)?;
        Ok(virtualips)
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

    pub fn fetch_vms(&self) -> Result<Vec<VM>, Error> {
        let mut vms = Vec::<VM>::new();
        let _ = self.fetch_children(&mut vms)?;
        Ok(vms)
    }

    pub fn fetch_vminterfaces(&self) -> Result<Vec<VMInterface>, Error> {
        let mut vminterfaces = Vec::<VMInterface>::new();
        let _ = self.fetch_children(&mut vminterfaces)?;
        Ok(vminterfaces)
    }

    pub fn fetch_policygroups(&self) -> Result<Vec<PolicyGroup>, Error> {
        let mut policygroups = Vec::<PolicyGroup>::new();
        let _ = self.fetch_children(&mut policygroups)?;
        Ok(policygroups)
    }

    pub fn fetch_containers(&self) -> Result<Vec<Container>, Error> {
        let mut containers = Vec::<Container>::new();
        let _ = self.fetch_children(&mut containers)?;
        Ok(containers)
    }

    pub fn fetch_containerinterfaces(&self) -> Result<Vec<ContainerInterface>, Error> {
        let mut containerinterfaces = Vec::<ContainerInterface>::new();
        let _ = self.fetch_children(&mut containerinterfaces)?;
        Ok(containerinterfaces)
    }

    pub fn fetch_portmappings(&self) -> Result<Vec<PortMapping>, Error> {
        let mut portmappings = Vec::<PortMapping>::new();
        let _ = self.fetch_children(&mut portmappings)?;
        Ok(portmappings)
    }

    pub fn fetch_qos(&self) -> Result<Vec<QOS>, Error> {
        let mut qos = Vec::<QOS>::new();
        let _ = self.fetch_children(&mut qos)?;
        Ok(qos)
    }

    pub fn fetch_hostinterfaces(&self) -> Result<Vec<HostInterface>, Error> {
        let mut hostinterfaces = Vec::<HostInterface>::new();
        let _ = self.fetch_children(&mut hostinterfaces)?;
        Ok(hostinterfaces)
    }

    pub fn fetch_vportmirrors(&self) -> Result<Vec<VPortMirror>, Error> {
        let mut vportmirrors = Vec::<VPortMirror>::new();
        let _ = self.fetch_children(&mut vportmirrors)?;
        Ok(vportmirrors)
    }

    pub fn fetch_applicationperformancemanagements(&self) -> Result<Vec<Applicationperformancemanagement>, Error> {
        let mut applicationperformancemanagements = Vec::<Applicationperformancemanagement>::new();
        let _ = self.fetch_children(&mut applicationperformancemanagements)?;
        Ok(applicationperformancemanagements)
    }

    pub fn fetch_bridgeinterfaces(&self) -> Result<Vec<BridgeInterface>, Error> {
        let mut bridgeinterfaces = Vec::<BridgeInterface>::new();
        let _ = self.fetch_children(&mut bridgeinterfaces)?;
        Ok(bridgeinterfaces)
    }

    pub fn fetch_vrss(&self) -> Result<Vec<VRS>, Error> {
        let mut vrss = Vec::<VRS>::new();
        let _ = self.fetch_children(&mut vrss)?;
        Ok(vrss)
    }

    pub fn fetch_trunks(&self) -> Result<Vec<Trunk>, Error> {
        let mut trunks = Vec::<Trunk>::new();
        let _ = self.fetch_children(&mut trunks)?;
        Ok(trunks)
    }

    pub fn fetch_statistics(&self) -> Result<Vec<Statistics>, Error> {
        let mut statistics = Vec::<Statistics>::new();
        let _ = self.fetch_children(&mut statistics)?;
        Ok(statistics)
    }

    pub fn fetch_statisticspolicies(&self) -> Result<Vec<StatisticsPolicy>, Error> {
        let mut statisticspolicies = Vec::<StatisticsPolicy>::new();
        let _ = self.fetch_children(&mut statisticspolicies)?;
        Ok(statisticspolicies)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}