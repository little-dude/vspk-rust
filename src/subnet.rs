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
pub use addressrange::AddressRange;
pub use vmresync::VMResync;
pub use metadata::Metadata;
pub use bgpneighbor::BGPNeighbor;
pub use dhcpoption::DHCPOption;
pub use virtualip::VirtualIP;
pub use ikegatewayconnection::IKEGatewayConnection;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use vminterface::VMInterface;
pub use container::Container;
pub use containerinterface::ContainerInterface;
pub use containerresync::ContainerResync;
pub use qos::QOS;
pub use vport::VPort;
pub use ipreservation::IPReservation;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct Subnet<'a> {
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
    
    #[serde(rename="PATEnabled")]
    pat_enabled: Option<String>,
    
    #[serde(rename="DPI")]
    dpi: Option<String>,
    
    #[serde(rename="IPType")]
    ip_type: Option<String>,
    
    #[serde(rename="IPv6Address")]
    ipv6_address: Option<String>,
    
    #[serde(rename="IPv6Gateway")]
    ipv6_gateway: Option<String>,
    
    #[serde(rename="maintenanceMode")]
    maintenance_mode: Option<String>,
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    gateway: Option<String>,
    
    #[serde(rename="gatewayMACAddress")]
    gateway_mac_address: Option<String>,
    address: Option<String>,
    
    #[serde(rename="defaultAction")]
    default_action: Option<String>,
    
    #[serde(rename="templateID")]
    template_id: Option<String>,
    
    #[serde(rename="serviceID")]
    service_id: u64,
    description: Option<String>,
    netmask: Option<String>,
    
    #[serde(rename="vnId")]
    vn_id: u64,
    encryption: Option<String>,
    underlay: bool,
    
    #[serde(rename="underlayEnabled")]
    underlay_enabled: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="policyGroupID")]
    policy_group_id: u64,
    
    #[serde(rename="routeDistinguisher")]
    route_distinguisher: Option<String>,
    
    #[serde(rename="routeTarget")]
    route_target: Option<String>,
    
    #[serde(rename="splitSubnet")]
    split_subnet: bool,
    
    #[serde(rename="proxyARP")]
    proxy_arp: bool,
    
    #[serde(rename="useGlobalMAC")]
    use_global_mac: Option<String>,
    
    #[serde(rename="associatedApplicationID")]
    associated_application_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectID")]
    associated_application_object_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectType")]
    associated_application_object_type: Option<String>,
    
    #[serde(rename="associatedMulticastChannelMapID")]
    associated_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="associatedSharedNetworkResourceID")]
    associated_shared_network_resource_id: Option<String>,
    public: bool,
    multicast: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for Subnet<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "subnet"
    }

    fn group_path() -> &'static str {
        "subnets"
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

impl<'a> Subnet<'a> {

    fn fetch_tcas(&self) -> Result<Vec<TCA>, BambouError> {
        let mut tcas = Vec::<TCA>::new();
        try!(self.fetch_children(&mut tcas));
        Ok(tcas)
    }

    fn fetch_addressranges(&self) -> Result<Vec<AddressRange>, BambouError> {
        let mut addressranges = Vec::<AddressRange>::new();
        try!(self.fetch_children(&mut addressranges));
        Ok(addressranges)
    }

    fn fetch_resync(&self) -> Result<Vec<VMResync>, BambouError> {
        let mut resync = Vec::<VMResync>::new();
        try!(self.fetch_children(&mut resync));
        Ok(resync)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, BambouError> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        try!(self.fetch_children(&mut bgpneighbors));
        Ok(bgpneighbors)
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

    fn fetch_ikegatewayconnections(&self) -> Result<Vec<IKEGatewayConnection>, BambouError> {
        let mut ikegatewayconnections = Vec::<IKEGatewayConnection>::new();
        try!(self.fetch_children(&mut ikegatewayconnections));
        Ok(ikegatewayconnections)
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

    fn fetch_containerresync(&self) -> Result<Vec<ContainerResync>, BambouError> {
        let mut containerresync = Vec::<ContainerResync>::new();
        try!(self.fetch_children(&mut containerresync));
        Ok(containerresync)
    }

    fn fetch_qos(&self) -> Result<Vec<QOS>, BambouError> {
        let mut qos = Vec::<QOS>::new();
        try!(self.fetch_children(&mut qos));
        Ok(qos)
    }

    fn fetch_vports(&self) -> Result<Vec<VPort>, BambouError> {
        let mut vports = Vec::<VPort>::new();
        try!(self.fetch_children(&mut vports));
        Ok(vports)
    }

    fn fetch_ipreservations(&self) -> Result<Vec<IPReservation>, BambouError> {
        let mut ipreservations = Vec::<IPReservation>::new();
        try!(self.fetch_children(&mut ipreservations));
        Ok(ipreservations)
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
