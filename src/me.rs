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


pub use l2domain::L2Domain;
pub use vcentereamconfig::VCenterEAMConfig;
pub use ratelimiter::RateLimiter;
pub use gateway::Gateway;
pub use gatewaytemplate::GatewayTemplate;
pub use patmapper::PATMapper;
pub use patnatpool::PATNATPool;
pub use tca::TCA;
pub use vcenter::VCenter;
pub use vcenterhypervisor::VCenterHypervisor;
pub use redirectiontarget::RedirectionTarget;
pub use redundancygroup::RedundancyGroup;
pub use certificate::Certificate;
pub use metadata::Metadata;
pub use metadatatag::MetadataTag;
pub use networklayout::NetworkLayout;
pub use keyservermember::KeyServerMember;
pub use zfbautoassignment::ZFBAutoAssignment;
pub use zfbrequest::ZFBRequest;
pub use bgpneighbor::BGPNeighbor;
pub use bgpprofile::BGPProfile;
pub use egressaclentrytemplate::EgressACLEntryTemplate;
pub use egressacltemplate::EgressACLTemplate;
pub use domainfipacltemplate::DomainFIPAclTemplate;
pub use floatingipacltemplate::FloatingIPACLTemplate;
pub use egressqospolicy::EgressQOSPolicy;
pub use sharednetworkresource::SharedNetworkResource;
pub use license::License;
pub use licensestatus::LicenseStatus;
pub use mirrordestination::MirrorDestination;
pub use siteinfo::SiteInfo;
pub use floatingip::FloatingIp;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use vminterface::VMInterface;
pub use cloudmgmtsystem::CloudMgmtSystem;
pub use infrastructuregatewayprofile::InfrastructureGatewayProfile;
pub use infrastructurevscprofile::InfrastructureVscProfile;
pub use ingressaclentrytemplate::IngressACLEntryTemplate;
pub use ingressacltemplate::IngressACLTemplate;
pub use ingressadvfwdentrytemplate::IngressAdvFwdEntryTemplate;
pub use enterprise::Enterprise;
pub use enterpriseprofile::EnterpriseProfile;
pub use job::Job;
pub use policygroup::PolicyGroup;
pub use domain::Domain;
pub use zone::Zone;
pub use container::Container;
pub use containerinterface::ContainerInterface;
pub use hostinterface::HostInterface;
pub use routingpolicy::RoutingPolicy;
pub use uplinkrd::UplinkRD;
pub use applicationservice::ApplicationService;
pub use vcentervrsconfig::VCenterVRSConfig;
pub use user::User;
pub use nsgateway::NSGateway;
pub use nsgatewaytemplate::NSGatewayTemplate;
pub use nsredundantgatewaygroup::NSRedundantGatewayGroup;
pub use vsp::VSP;
pub use staticroute::StaticRoute;
pub use statscollectorinfo::StatsCollectorInfo;
pub use subnet::Subnet;
pub use multicastchannelmap::MultiCastChannelMap;
pub use autodiscoveredgateway::AutoDiscoveredGateway;
pub use externalappservice::ExternalAppService;
pub use externalservice::ExternalService;
pub use systemconfig::SystemConfig;


#[derive(Serialize, Deserialize)]
pub struct Me<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    _session: Option<&'a Session>,
    #[serde(rename="ID")]
    id: Option<String>,
    
    #[serde(rename="APIKey")]
    api_keypassword: Option<String>,
    
    #[serde(rename="lastName")]
    last_name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="firstName")]
    first_name: Option<String>,
    disabled: bool,
    
    #[serde(rename="elasticSearchUIAddress")]
    elastic_search_ui_address: Option<String>,
    email: Option<String>,
    
    #[serde(rename="enterpriseID")]
    enterprise_id: Option<String>,
    
    #[serde(rename="enterpriseName")]
    enterprise_name: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="mobileNumber")]
    mobile_number: Option<String>,
    role: Option<String>,
    
    #[serde(rename="userName")]
    user_name: Option<String>,
    
    #[serde(rename="avatarData")]
    avatar_data: Option<String>,
    
    #[serde(rename="avatarType")]
    avatar_type: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for Me<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "me"
    }

    fn group_path() -> &'static str {
        "me"
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

impl<'a> Me<'a> {

    fn fetch_l2domains(&self) -> Result<Vec<L2Domain>, BambouError> {
        let mut l2domains = Vec::<L2Domain>::new();
        try!(self.fetch_children(&mut l2domains));
        Ok(l2domains)
    }

    fn fetch_eamconfigs(&self) -> Result<Vec<VCenterEAMConfig>, BambouError> {
        let mut eamconfigs = Vec::<VCenterEAMConfig>::new();
        try!(self.fetch_children(&mut eamconfigs));
        Ok(eamconfigs)
    }

    fn fetch_ratelimiters(&self) -> Result<Vec<RateLimiter>, BambouError> {
        let mut ratelimiters = Vec::<RateLimiter>::new();
        try!(self.fetch_children(&mut ratelimiters));
        Ok(ratelimiters)
    }

    fn fetch_gateways(&self) -> Result<Vec<Gateway>, BambouError> {
        let mut gateways = Vec::<Gateway>::new();
        try!(self.fetch_children(&mut gateways));
        Ok(gateways)
    }

    fn fetch_gatewaytemplates(&self) -> Result<Vec<GatewayTemplate>, BambouError> {
        let mut gatewaytemplates = Vec::<GatewayTemplate>::new();
        try!(self.fetch_children(&mut gatewaytemplates));
        Ok(gatewaytemplates)
    }

    fn fetch_patmappers(&self) -> Result<Vec<PATMapper>, BambouError> {
        let mut patmappers = Vec::<PATMapper>::new();
        try!(self.fetch_children(&mut patmappers));
        Ok(patmappers)
    }

    fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, BambouError> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        try!(self.fetch_children(&mut patnatpools));
        Ok(patnatpools)
    }

    fn fetch_tcas(&self) -> Result<Vec<TCA>, BambouError> {
        let mut tcas = Vec::<TCA>::new();
        try!(self.fetch_children(&mut tcas));
        Ok(tcas)
    }

    fn fetch_vcenters(&self) -> Result<Vec<VCenter>, BambouError> {
        let mut vcenters = Vec::<VCenter>::new();
        try!(self.fetch_children(&mut vcenters));
        Ok(vcenters)
    }

    fn fetch_vcenterhypervisors(&self) -> Result<Vec<VCenterHypervisor>, BambouError> {
        let mut vcenterhypervisors = Vec::<VCenterHypervisor>::new();
        try!(self.fetch_children(&mut vcenterhypervisors));
        Ok(vcenterhypervisors)
    }

    fn fetch_redirectiontargets(&self) -> Result<Vec<RedirectionTarget>, BambouError> {
        let mut redirectiontargets = Vec::<RedirectionTarget>::new();
        try!(self.fetch_children(&mut redirectiontargets));
        Ok(redirectiontargets)
    }

    fn fetch_redundancygroups(&self) -> Result<Vec<RedundancyGroup>, BambouError> {
        let mut redundancygroups = Vec::<RedundancyGroup>::new();
        try!(self.fetch_children(&mut redundancygroups));
        Ok(redundancygroups)
    }

    fn fetch_certificates(&self) -> Result<Vec<Certificate>, BambouError> {
        let mut certificates = Vec::<Certificate>::new();
        try!(self.fetch_children(&mut certificates));
        Ok(certificates)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_metadatatags(&self) -> Result<Vec<MetadataTag>, BambouError> {
        let mut metadatatags = Vec::<MetadataTag>::new();
        try!(self.fetch_children(&mut metadatatags));
        Ok(metadatatags)
    }

    fn fetch_networklayout(&self) -> Result<Vec<NetworkLayout>, BambouError> {
        let mut networklayout = Vec::<NetworkLayout>::new();
        try!(self.fetch_children(&mut networklayout));
        Ok(networklayout)
    }

    fn fetch_keyservermembers(&self) -> Result<Vec<KeyServerMember>, BambouError> {
        let mut keyservermembers = Vec::<KeyServerMember>::new();
        try!(self.fetch_children(&mut keyservermembers));
        Ok(keyservermembers)
    }

    fn fetch_zfbautoassignments(&self) -> Result<Vec<ZFBAutoAssignment>, BambouError> {
        let mut zfbautoassignments = Vec::<ZFBAutoAssignment>::new();
        try!(self.fetch_children(&mut zfbautoassignments));
        Ok(zfbautoassignments)
    }

    fn fetch_zfbrequests(&self) -> Result<Vec<ZFBRequest>, BambouError> {
        let mut zfbrequests = Vec::<ZFBRequest>::new();
        try!(self.fetch_children(&mut zfbrequests));
        Ok(zfbrequests)
    }

    fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, BambouError> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        try!(self.fetch_children(&mut bgpneighbors));
        Ok(bgpneighbors)
    }

    fn fetch_bgpprofiles(&self) -> Result<Vec<BGPProfile>, BambouError> {
        let mut bgpprofiles = Vec::<BGPProfile>::new();
        try!(self.fetch_children(&mut bgpprofiles));
        Ok(bgpprofiles)
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

    fn fetch_egressdomainfloatingipacltemplates(&self) -> Result<Vec<DomainFIPAclTemplate>, BambouError> {
        let mut egressdomainfloatingipacltemplates = Vec::<DomainFIPAclTemplate>::new();
        try!(self.fetch_children(&mut egressdomainfloatingipacltemplates));
        Ok(egressdomainfloatingipacltemplates)
    }

    fn fetch_egressfloatingipacltemplates(&self) -> Result<Vec<FloatingIPACLTemplate>, BambouError> {
        let mut egressfloatingipacltemplates = Vec::<FloatingIPACLTemplate>::new();
        try!(self.fetch_children(&mut egressfloatingipacltemplates));
        Ok(egressfloatingipacltemplates)
    }

    fn fetch_egressqospolicies(&self) -> Result<Vec<EgressQOSPolicy>, BambouError> {
        let mut egressqospolicies = Vec::<EgressQOSPolicy>::new();
        try!(self.fetch_children(&mut egressqospolicies));
        Ok(egressqospolicies)
    }

    fn fetch_sharednetworkresources(&self) -> Result<Vec<SharedNetworkResource>, BambouError> {
        let mut sharednetworkresources = Vec::<SharedNetworkResource>::new();
        try!(self.fetch_children(&mut sharednetworkresources));
        Ok(sharednetworkresources)
    }

    fn fetch_licenses(&self) -> Result<Vec<License>, BambouError> {
        let mut licenses = Vec::<License>::new();
        try!(self.fetch_children(&mut licenses));
        Ok(licenses)
    }

    fn fetch_licensestatus(&self) -> Result<Vec<LicenseStatus>, BambouError> {
        let mut licensestatus = Vec::<LicenseStatus>::new();
        try!(self.fetch_children(&mut licensestatus));
        Ok(licensestatus)
    }

    fn fetch_mirrordestinations(&self) -> Result<Vec<MirrorDestination>, BambouError> {
        let mut mirrordestinations = Vec::<MirrorDestination>::new();
        try!(self.fetch_children(&mut mirrordestinations));
        Ok(mirrordestinations)
    }

    fn fetch_sites(&self) -> Result<Vec<SiteInfo>, BambouError> {
        let mut sites = Vec::<SiteInfo>::new();
        try!(self.fetch_children(&mut sites));
        Ok(sites)
    }

    fn fetch_floatingips(&self) -> Result<Vec<FloatingIp>, BambouError> {
        let mut floatingips = Vec::<FloatingIp>::new();
        try!(self.fetch_children(&mut floatingips));
        Ok(floatingips)
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

    fn fetch_cms(&self) -> Result<Vec<CloudMgmtSystem>, BambouError> {
        let mut cms = Vec::<CloudMgmtSystem>::new();
        try!(self.fetch_children(&mut cms));
        Ok(cms)
    }

    fn fetch_infrastructuregatewayprofiles(&self) -> Result<Vec<InfrastructureGatewayProfile>, BambouError> {
        let mut infrastructuregatewayprofiles = Vec::<InfrastructureGatewayProfile>::new();
        try!(self.fetch_children(&mut infrastructuregatewayprofiles));
        Ok(infrastructuregatewayprofiles)
    }

    fn fetch_infrastructurevscprofiles(&self) -> Result<Vec<InfrastructureVscProfile>, BambouError> {
        let mut infrastructurevscprofiles = Vec::<InfrastructureVscProfile>::new();
        try!(self.fetch_children(&mut infrastructurevscprofiles));
        Ok(infrastructurevscprofiles)
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

    fn fetch_ingressadvfwdentrytemplates(&self) -> Result<Vec<IngressAdvFwdEntryTemplate>, BambouError> {
        let mut ingressadvfwdentrytemplates = Vec::<IngressAdvFwdEntryTemplate>::new();
        try!(self.fetch_children(&mut ingressadvfwdentrytemplates));
        Ok(ingressadvfwdentrytemplates)
    }

    fn fetch_enterprises(&self) -> Result<Vec<Enterprise>, BambouError> {
        let mut enterprises = Vec::<Enterprise>::new();
        try!(self.fetch_children(&mut enterprises));
        Ok(enterprises)
    }

    fn fetch_enterpriseprofiles(&self) -> Result<Vec<EnterpriseProfile>, BambouError> {
        let mut enterpriseprofiles = Vec::<EnterpriseProfile>::new();
        try!(self.fetch_children(&mut enterpriseprofiles));
        Ok(enterpriseprofiles)
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

    fn fetch_domains(&self) -> Result<Vec<Domain>, BambouError> {
        let mut domains = Vec::<Domain>::new();
        try!(self.fetch_children(&mut domains));
        Ok(domains)
    }

    fn fetch_zones(&self) -> Result<Vec<Zone>, BambouError> {
        let mut zones = Vec::<Zone>::new();
        try!(self.fetch_children(&mut zones));
        Ok(zones)
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

    fn fetch_hostinterfaces(&self) -> Result<Vec<HostInterface>, BambouError> {
        let mut hostinterfaces = Vec::<HostInterface>::new();
        try!(self.fetch_children(&mut hostinterfaces));
        Ok(hostinterfaces)
    }

    fn fetch_routingpolicies(&self) -> Result<Vec<RoutingPolicy>, BambouError> {
        let mut routingpolicies = Vec::<RoutingPolicy>::new();
        try!(self.fetch_children(&mut routingpolicies));
        Ok(routingpolicies)
    }

    fn fetch_uplinkroutedistinguishers(&self) -> Result<Vec<UplinkRD>, BambouError> {
        let mut uplinkroutedistinguishers = Vec::<UplinkRD>::new();
        try!(self.fetch_children(&mut uplinkroutedistinguishers));
        Ok(uplinkroutedistinguishers)
    }

    fn fetch_applicationservices(&self) -> Result<Vec<ApplicationService>, BambouError> {
        let mut applicationservices = Vec::<ApplicationService>::new();
        try!(self.fetch_children(&mut applicationservices));
        Ok(applicationservices)
    }

    fn fetch_vrsconfigs(&self) -> Result<Vec<VCenterVRSConfig>, BambouError> {
        let mut vrsconfigs = Vec::<VCenterVRSConfig>::new();
        try!(self.fetch_children(&mut vrsconfigs));
        Ok(vrsconfigs)
    }

    fn fetch_users(&self) -> Result<Vec<User>, BambouError> {
        let mut users = Vec::<User>::new();
        try!(self.fetch_children(&mut users));
        Ok(users)
    }

    fn fetch_nsgateways(&self) -> Result<Vec<NSGateway>, BambouError> {
        let mut nsgateways = Vec::<NSGateway>::new();
        try!(self.fetch_children(&mut nsgateways));
        Ok(nsgateways)
    }

    fn fetch_nsgatewaytemplates(&self) -> Result<Vec<NSGatewayTemplate>, BambouError> {
        let mut nsgatewaytemplates = Vec::<NSGatewayTemplate>::new();
        try!(self.fetch_children(&mut nsgatewaytemplates));
        Ok(nsgatewaytemplates)
    }

    fn fetch_nsgredundancygroups(&self) -> Result<Vec<NSRedundantGatewayGroup>, BambouError> {
        let mut nsgredundancygroups = Vec::<NSRedundantGatewayGroup>::new();
        try!(self.fetch_children(&mut nsgredundancygroups));
        Ok(nsgredundancygroups)
    }

    fn fetch_vsps(&self) -> Result<Vec<VSP>, BambouError> {
        let mut vsps = Vec::<VSP>::new();
        try!(self.fetch_children(&mut vsps));
        Ok(vsps)
    }

    fn fetch_staticroutes(&self) -> Result<Vec<StaticRoute>, BambouError> {
        let mut staticroutes = Vec::<StaticRoute>::new();
        try!(self.fetch_children(&mut staticroutes));
        Ok(staticroutes)
    }

    fn fetch_statisticscollector(&self) -> Result<Vec<StatsCollectorInfo>, BambouError> {
        let mut statisticscollector = Vec::<StatsCollectorInfo>::new();
        try!(self.fetch_children(&mut statisticscollector));
        Ok(statisticscollector)
    }

    fn fetch_subnets(&self) -> Result<Vec<Subnet>, BambouError> {
        let mut subnets = Vec::<Subnet>::new();
        try!(self.fetch_children(&mut subnets));
        Ok(subnets)
    }

    fn fetch_multicastchannelmaps(&self) -> Result<Vec<MultiCastChannelMap>, BambouError> {
        let mut multicastchannelmaps = Vec::<MultiCastChannelMap>::new();
        try!(self.fetch_children(&mut multicastchannelmaps));
        Ok(multicastchannelmaps)
    }

    fn fetch_autodiscoveredgateways(&self) -> Result<Vec<AutoDiscoveredGateway>, BambouError> {
        let mut autodiscoveredgateways = Vec::<AutoDiscoveredGateway>::new();
        try!(self.fetch_children(&mut autodiscoveredgateways));
        Ok(autodiscoveredgateways)
    }

    fn fetch_externalappservices(&self) -> Result<Vec<ExternalAppService>, BambouError> {
        let mut externalappservices = Vec::<ExternalAppService>::new();
        try!(self.fetch_children(&mut externalappservices));
        Ok(externalappservices)
    }

    fn fetch_externalservices(&self) -> Result<Vec<ExternalService>, BambouError> {
        let mut externalservices = Vec::<ExternalService>::new();
        try!(self.fetch_children(&mut externalservices));
        Ok(externalservices)
    }

    fn fetch_systemconfigs(&self) -> Result<Vec<SystemConfig>, BambouError> {
        let mut systemconfigs = Vec::<SystemConfig>::new();
        try!(self.fetch_children(&mut systemconfigs));
        Ok(systemconfigs)
    }
}
