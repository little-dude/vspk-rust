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
pub use redirectiontarget::RedirectionTarget;
pub use permission::Permission;
pub use metadata::Metadata;
pub use egressaclentrytemplate::EgressACLEntryTemplate;
pub use egressacltemplate::EgressACLTemplate;
pub use dhcpoption::DHCPOption;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use vminterface::VMInterface;
pub use ingressaclentrytemplate::IngressACLEntryTemplate;
pub use ingressacltemplate::IngressACLTemplate;
pub use ingressadvfwdtemplate::IngressAdvFwdTemplate;
pub use ingressexternalservicetemplate::IngressExternalServiceTemplate;
pub use job::Job;
pub use policygroup::PolicyGroup;
pub use container::Container;
pub use containerinterface::ContainerInterface;
pub use qos::QOS;
pub use hostinterface::HostInterface;
pub use uplinkrd::UplinkRD;
pub use vpnconnection::VPNConnection;
pub use vport::VPort;
pub use applicationperformancemanagementbinding::Applicationperformancemanagementbinding;
pub use bridgeinterface::BridgeInterface;
pub use group::Group;
pub use staticroute::StaticRoute;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct L2Domain<'a> {
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
    
    #[serde(rename="DHCPManaged")]
    dhcp_managed: bool,
    
    #[serde(rename="DPI")]
    dpi: Option<String>,
    
    #[serde(rename="IPType")]
    ip_type: Option<String>,
    
    #[serde(rename="maintenanceMode")]
    maintenance_mode: Option<String>,
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    gateway: Option<String>,
    
    #[serde(rename="gatewayMACAddress")]
    gateway_mac_address: Option<String>,
    address: Option<String>,
    
    #[serde(rename="templateID")]
    template_id: Option<String>,
    
    #[serde(rename="serviceID")]
    service_id: u64,
    description: Option<String>,
    netmask: Option<String>,
    
    #[serde(rename="vnId")]
    vn_id: u64,
    encryption: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="policyChangeStatus")]
    policy_change_status: Option<String>,
    
    #[serde(rename="routeDistinguisher")]
    route_distinguisher: Option<String>,
    
    #[serde(rename="routeTarget")]
    route_target: Option<String>,
    
    #[serde(rename="uplinkPreference")]
    uplink_preference: Option<String>,
    
    #[serde(rename="associatedMulticastChannelMapID")]
    associated_multicast_channel_map_id: Option<String>,
    
    #[serde(rename="associatedSharedNetworkResourceID")]
    associated_shared_network_resource_id: Option<String>,
    stretched: bool,
    multicast: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for L2Domain<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "l2domain"
    }

    fn group_path() -> &'static str {
        "l2domains"
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

impl<'a> L2Domain<'a> {

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

    fn fetch_redirectiontargets(&self) -> Result<Vec<RedirectionTarget>, BambouError> {
        let mut redirectiontargets = Vec::<RedirectionTarget>::new();
        try!(self.fetch_children(&mut redirectiontargets));
        Ok(redirectiontargets)
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

    fn fetch_egressaclentrytemplates(&self) -> Result<Vec<EgressACLEntryTemplate>, BambouError> {
        let mut egressaclentrytemplates = Vec::<EgressACLEntryTemplate>::new();
        try!(self.fetch_children(&mut egressaclentrytemplates));
        Ok(egressaclentrytemplates)
    }

    fn fetch_egressacltemplates(&self) -> Result<Vec<EgressACLTemplate>, BambouError> {
        let mut egressacltemplates = Vec::<EgressACLTemplate>::new();
        try!(self.fetch_children(&mut egressacltemplates));
        Ok(egressacltemplates)
    }

    fn fetch_dhcpoptions(&self) -> Result<Vec<DHCPOption>, BambouError> {
        let mut dhcpoptions = Vec::<DHCPOption>::new();
        try!(self.fetch_children(&mut dhcpoptions));
        Ok(dhcpoptions)
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

    fn fetch_ingressaclentrytemplates(&self) -> Result<Vec<IngressACLEntryTemplate>, BambouError> {
        let mut ingressaclentrytemplates = Vec::<IngressACLEntryTemplate>::new();
        try!(self.fetch_children(&mut ingressaclentrytemplates));
        Ok(ingressaclentrytemplates)
    }

    fn fetch_ingressacltemplates(&self) -> Result<Vec<IngressACLTemplate>, BambouError> {
        let mut ingressacltemplates = Vec::<IngressACLTemplate>::new();
        try!(self.fetch_children(&mut ingressacltemplates));
        Ok(ingressacltemplates)
    }

    fn fetch_ingressadvfwdtemplates(&self) -> Result<Vec<IngressAdvFwdTemplate>, BambouError> {
        let mut ingressadvfwdtemplates = Vec::<IngressAdvFwdTemplate>::new();
        try!(self.fetch_children(&mut ingressadvfwdtemplates));
        Ok(ingressadvfwdtemplates)
    }

    fn fetch_ingressexternalservicetemplates(&self) -> Result<Vec<IngressExternalServiceTemplate>, BambouError> {
        let mut ingressexternalservicetemplates = Vec::<IngressExternalServiceTemplate>::new();
        try!(self.fetch_children(&mut ingressexternalservicetemplates));
        Ok(ingressexternalservicetemplates)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
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

    fn fetch_uplinkroutedistinguishers(&self) -> Result<Vec<UplinkRD>, BambouError> {
        let mut uplinkroutedistinguishers = Vec::<UplinkRD>::new();
        try!(self.fetch_children(&mut uplinkroutedistinguishers));
        Ok(uplinkroutedistinguishers)
    }

    fn fetch_vpnconnections(&self) -> Result<Vec<VPNConnection>, BambouError> {
        let mut vpnconnections = Vec::<VPNConnection>::new();
        try!(self.fetch_children(&mut vpnconnections));
        Ok(vpnconnections)
    }

    fn fetch_vports(&self) -> Result<Vec<VPort>, BambouError> {
        let mut vports = Vec::<VPort>::new();
        try!(self.fetch_children(&mut vports));
        Ok(vports)
    }

    fn fetch_applicationperformancemanagementbindings(&self) -> Result<Vec<Applicationperformancemanagementbinding>, BambouError> {
        let mut applicationperformancemanagementbindings = Vec::<Applicationperformancemanagementbinding>::new();
        try!(self.fetch_children(&mut applicationperformancemanagementbindings));
        Ok(applicationperformancemanagementbindings)
    }

    fn fetch_bridgeinterfaces(&self) -> Result<Vec<BridgeInterface>, BambouError> {
        let mut bridgeinterfaces = Vec::<BridgeInterface>::new();
        try!(self.fetch_children(&mut bridgeinterfaces));
        Ok(bridgeinterfaces)
    }

    fn fetch_groups(&self) -> Result<Vec<Group>, BambouError> {
        let mut groups = Vec::<Group>::new();
        try!(self.fetch_children(&mut groups));
        Ok(groups)
    }

    fn fetch_staticroutes(&self) -> Result<Vec<StaticRoute>, BambouError> {
        let mut staticroutes = Vec::<StaticRoute>::new();
        try!(self.fetch_children(&mut staticroutes));
        Ok(staticroutes)
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
