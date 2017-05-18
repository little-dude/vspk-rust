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
pub use performancemonitor::PerformanceMonitor;
pub use certificate::Certificate;
pub use metadata::Metadata;
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
pub use underlay::Underlay;
pub use infrastructureaccessprofile::InfrastructureAccessProfile;
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
pub use nsggroup::NSGGroup;
pub use nsredundantgatewaygroup::NSRedundantGatewayGroup;
pub use vsp::VSP;
pub use staticroute::StaticRoute;
pub use statscollectorinfo::StatsCollectorInfo;
pub use subnet::Subnet;
pub use ducgroup::DUCGroup;
pub use multicastchannelmap::MultiCastChannelMap;
pub use autodiscoveredgateway::AutoDiscoveredGateway;
pub use systemconfig::SystemConfig;


#[derive(Serialize, Deserialize, Default)]
pub struct Me<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    _session: Option<&'a Session>,

    #[serde(rename="ID")]
    id: Option<String>,

    #[serde(rename = "APIKey")]
    api_key: Option<String>,

    
    pub password: Option<String>,
    
    #[serde(rename="lastName")]
    pub last_name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="firstName")]
    pub first_name: Option<String>,
    
    pub disabled: bool,
    
    #[serde(rename="elasticSearchAddress")]
    pub elastic_search_address: Option<String>,
    
    #[serde(rename="flowCollectionEnabled")]
    pub flow_collection_enabled: bool,
    
    pub email: Option<String>,
    
    #[serde(rename="enterpriseID")]
    pub enterprise_id: Option<String>,
    
    #[serde(rename="enterpriseName")]
    pub enterprise_name: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="mobileNumber")]
    pub mobile_number: Option<String>,
    
    pub role: Option<String>,
    
    #[serde(rename="userName")]
    pub user_name: Option<String>,
    
    #[serde(rename="statisticsEnabled")]
    pub statistics_enabled: bool,
    
    #[serde(rename="avatarData")]
    pub avatar_data: Option<String>,
    
    #[serde(rename="avatarType")]
    pub avatar_type: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for Me<'a> {
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
        "me"
    }

    fn group_path() -> &'static str {
        "me"
    }

    fn is_root(&self) -> bool {
        true
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

impl<'a> Me<'a> {

    pub fn fetch_l2domains(&self) -> Result<Vec<L2Domain>, Error> {
        let mut l2domains = Vec::<L2Domain>::new();
        let _ = self.fetch_children(&mut l2domains)?;
        Ok(l2domains)
    }

    pub fn fetch_eamconfigs(&self) -> Result<Vec<VCenterEAMConfig>, Error> {
        let mut eamconfigs = Vec::<VCenterEAMConfig>::new();
        let _ = self.fetch_children(&mut eamconfigs)?;
        Ok(eamconfigs)
    }

    pub fn fetch_ratelimiters(&self) -> Result<Vec<RateLimiter>, Error> {
        let mut ratelimiters = Vec::<RateLimiter>::new();
        let _ = self.fetch_children(&mut ratelimiters)?;
        Ok(ratelimiters)
    }

    pub fn fetch_gateways(&self) -> Result<Vec<Gateway>, Error> {
        let mut gateways = Vec::<Gateway>::new();
        let _ = self.fetch_children(&mut gateways)?;
        Ok(gateways)
    }

    pub fn fetch_gatewaytemplates(&self) -> Result<Vec<GatewayTemplate>, Error> {
        let mut gatewaytemplates = Vec::<GatewayTemplate>::new();
        let _ = self.fetch_children(&mut gatewaytemplates)?;
        Ok(gatewaytemplates)
    }

    pub fn fetch_patmappers(&self) -> Result<Vec<PATMapper>, Error> {
        let mut patmappers = Vec::<PATMapper>::new();
        let _ = self.fetch_children(&mut patmappers)?;
        Ok(patmappers)
    }

    pub fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, Error> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        let _ = self.fetch_children(&mut patnatpools)?;
        Ok(patnatpools)
    }

    pub fn fetch_tcas(&self) -> Result<Vec<TCA>, Error> {
        let mut tcas = Vec::<TCA>::new();
        let _ = self.fetch_children(&mut tcas)?;
        Ok(tcas)
    }

    pub fn fetch_vcenters(&self) -> Result<Vec<VCenter>, Error> {
        let mut vcenters = Vec::<VCenter>::new();
        let _ = self.fetch_children(&mut vcenters)?;
        Ok(vcenters)
    }

    pub fn fetch_vcenterhypervisors(&self) -> Result<Vec<VCenterHypervisor>, Error> {
        let mut vcenterhypervisors = Vec::<VCenterHypervisor>::new();
        let _ = self.fetch_children(&mut vcenterhypervisors)?;
        Ok(vcenterhypervisors)
    }

    pub fn fetch_redirectiontargets(&self) -> Result<Vec<RedirectionTarget>, Error> {
        let mut redirectiontargets = Vec::<RedirectionTarget>::new();
        let _ = self.fetch_children(&mut redirectiontargets)?;
        Ok(redirectiontargets)
    }

    pub fn fetch_redundancygroups(&self) -> Result<Vec<RedundancyGroup>, Error> {
        let mut redundancygroups = Vec::<RedundancyGroup>::new();
        let _ = self.fetch_children(&mut redundancygroups)?;
        Ok(redundancygroups)
    }

    pub fn fetch_performancemonitors(&self) -> Result<Vec<PerformanceMonitor>, Error> {
        let mut performancemonitors = Vec::<PerformanceMonitor>::new();
        let _ = self.fetch_children(&mut performancemonitors)?;
        Ok(performancemonitors)
    }

    pub fn fetch_certificates(&self) -> Result<Vec<Certificate>, Error> {
        let mut certificates = Vec::<Certificate>::new();
        let _ = self.fetch_children(&mut certificates)?;
        Ok(certificates)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_networklayout(&self) -> Result<Vec<NetworkLayout>, Error> {
        let mut networklayout = Vec::<NetworkLayout>::new();
        let _ = self.fetch_children(&mut networklayout)?;
        Ok(networklayout)
    }

    pub fn fetch_keyservermembers(&self) -> Result<Vec<KeyServerMember>, Error> {
        let mut keyservermembers = Vec::<KeyServerMember>::new();
        let _ = self.fetch_children(&mut keyservermembers)?;
        Ok(keyservermembers)
    }

    pub fn fetch_zfbautoassignments(&self) -> Result<Vec<ZFBAutoAssignment>, Error> {
        let mut zfbautoassignments = Vec::<ZFBAutoAssignment>::new();
        let _ = self.fetch_children(&mut zfbautoassignments)?;
        Ok(zfbautoassignments)
    }

    pub fn fetch_zfbrequests(&self) -> Result<Vec<ZFBRequest>, Error> {
        let mut zfbrequests = Vec::<ZFBRequest>::new();
        let _ = self.fetch_children(&mut zfbrequests)?;
        Ok(zfbrequests)
    }

    pub fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, Error> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        let _ = self.fetch_children(&mut bgpneighbors)?;
        Ok(bgpneighbors)
    }

    pub fn fetch_bgpprofiles(&self) -> Result<Vec<BGPProfile>, Error> {
        let mut bgpprofiles = Vec::<BGPProfile>::new();
        let _ = self.fetch_children(&mut bgpprofiles)?;
        Ok(bgpprofiles)
    }

    pub fn fetch_egressaclentrytemplates(&self) -> Result<Vec<EgressACLEntryTemplate>, Error> {
        let mut egressaclentrytemplates = Vec::<EgressACLEntryTemplate>::new();
        let _ = self.fetch_children(&mut egressaclentrytemplates)?;
        Ok(egressaclentrytemplates)
    }

    pub fn fetch_egressacltemplates(&self) -> Result<Vec<EgressACLTemplate>, Error> {
        let mut egressacltemplates = Vec::<EgressACLTemplate>::new();
        let _ = self.fetch_children(&mut egressacltemplates)?;
        Ok(egressacltemplates)
    }

    pub fn fetch_egressdomainfloatingipacltemplates(&self) -> Result<Vec<DomainFIPAclTemplate>, Error> {
        let mut egressdomainfloatingipacltemplates = Vec::<DomainFIPAclTemplate>::new();
        let _ = self.fetch_children(&mut egressdomainfloatingipacltemplates)?;
        Ok(egressdomainfloatingipacltemplates)
    }

    pub fn fetch_egressfloatingipacltemplates(&self) -> Result<Vec<FloatingIPACLTemplate>, Error> {
        let mut egressfloatingipacltemplates = Vec::<FloatingIPACLTemplate>::new();
        let _ = self.fetch_children(&mut egressfloatingipacltemplates)?;
        Ok(egressfloatingipacltemplates)
    }

    pub fn fetch_egressqospolicies(&self) -> Result<Vec<EgressQOSPolicy>, Error> {
        let mut egressqospolicies = Vec::<EgressQOSPolicy>::new();
        let _ = self.fetch_children(&mut egressqospolicies)?;
        Ok(egressqospolicies)
    }

    pub fn fetch_sharednetworkresources(&self) -> Result<Vec<SharedNetworkResource>, Error> {
        let mut sharednetworkresources = Vec::<SharedNetworkResource>::new();
        let _ = self.fetch_children(&mut sharednetworkresources)?;
        Ok(sharednetworkresources)
    }

    pub fn fetch_licenses(&self) -> Result<Vec<License>, Error> {
        let mut licenses = Vec::<License>::new();
        let _ = self.fetch_children(&mut licenses)?;
        Ok(licenses)
    }

    pub fn fetch_licensestatus(&self) -> Result<Vec<LicenseStatus>, Error> {
        let mut licensestatus = Vec::<LicenseStatus>::new();
        let _ = self.fetch_children(&mut licensestatus)?;
        Ok(licensestatus)
    }

    pub fn fetch_mirrordestinations(&self) -> Result<Vec<MirrorDestination>, Error> {
        let mut mirrordestinations = Vec::<MirrorDestination>::new();
        let _ = self.fetch_children(&mut mirrordestinations)?;
        Ok(mirrordestinations)
    }

    pub fn fetch_sites(&self) -> Result<Vec<SiteInfo>, Error> {
        let mut sites = Vec::<SiteInfo>::new();
        let _ = self.fetch_children(&mut sites)?;
        Ok(sites)
    }

    pub fn fetch_floatingips(&self) -> Result<Vec<FloatingIp>, Error> {
        let mut floatingips = Vec::<FloatingIp>::new();
        let _ = self.fetch_children(&mut floatingips)?;
        Ok(floatingips)
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

    pub fn fetch_cms(&self) -> Result<Vec<CloudMgmtSystem>, Error> {
        let mut cms = Vec::<CloudMgmtSystem>::new();
        let _ = self.fetch_children(&mut cms)?;
        Ok(cms)
    }

    pub fn fetch_underlays(&self) -> Result<Vec<Underlay>, Error> {
        let mut underlays = Vec::<Underlay>::new();
        let _ = self.fetch_children(&mut underlays)?;
        Ok(underlays)
    }

    pub fn fetch_infrastructureaccessprofiles(&self) -> Result<Vec<InfrastructureAccessProfile>, Error> {
        let mut infrastructureaccessprofiles = Vec::<InfrastructureAccessProfile>::new();
        let _ = self.fetch_children(&mut infrastructureaccessprofiles)?;
        Ok(infrastructureaccessprofiles)
    }

    pub fn fetch_infrastructuregatewayprofiles(&self) -> Result<Vec<InfrastructureGatewayProfile>, Error> {
        let mut infrastructuregatewayprofiles = Vec::<InfrastructureGatewayProfile>::new();
        let _ = self.fetch_children(&mut infrastructuregatewayprofiles)?;
        Ok(infrastructuregatewayprofiles)
    }

    pub fn fetch_infrastructurevscprofiles(&self) -> Result<Vec<InfrastructureVscProfile>, Error> {
        let mut infrastructurevscprofiles = Vec::<InfrastructureVscProfile>::new();
        let _ = self.fetch_children(&mut infrastructurevscprofiles)?;
        Ok(infrastructurevscprofiles)
    }

    pub fn fetch_ingressaclentrytemplates(&self) -> Result<Vec<IngressACLEntryTemplate>, Error> {
        let mut ingressaclentrytemplates = Vec::<IngressACLEntryTemplate>::new();
        let _ = self.fetch_children(&mut ingressaclentrytemplates)?;
        Ok(ingressaclentrytemplates)
    }

    pub fn fetch_ingressacltemplates(&self) -> Result<Vec<IngressACLTemplate>, Error> {
        let mut ingressacltemplates = Vec::<IngressACLTemplate>::new();
        let _ = self.fetch_children(&mut ingressacltemplates)?;
        Ok(ingressacltemplates)
    }

    pub fn fetch_ingressadvfwdentrytemplates(&self) -> Result<Vec<IngressAdvFwdEntryTemplate>, Error> {
        let mut ingressadvfwdentrytemplates = Vec::<IngressAdvFwdEntryTemplate>::new();
        let _ = self.fetch_children(&mut ingressadvfwdentrytemplates)?;
        Ok(ingressadvfwdentrytemplates)
    }

    pub fn fetch_enterprises(&self) -> Result<Vec<Enterprise>, Error> {
        let mut enterprises = Vec::<Enterprise>::new();
        let _ = self.fetch_children(&mut enterprises)?;
        Ok(enterprises)
    }

    pub fn fetch_enterpriseprofiles(&self) -> Result<Vec<EnterpriseProfile>, Error> {
        let mut enterpriseprofiles = Vec::<EnterpriseProfile>::new();
        let _ = self.fetch_children(&mut enterpriseprofiles)?;
        Ok(enterpriseprofiles)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_policygroups(&self) -> Result<Vec<PolicyGroup>, Error> {
        let mut policygroups = Vec::<PolicyGroup>::new();
        let _ = self.fetch_children(&mut policygroups)?;
        Ok(policygroups)
    }

    pub fn fetch_domains(&self) -> Result<Vec<Domain>, Error> {
        let mut domains = Vec::<Domain>::new();
        let _ = self.fetch_children(&mut domains)?;
        Ok(domains)
    }

    pub fn fetch_zones(&self) -> Result<Vec<Zone>, Error> {
        let mut zones = Vec::<Zone>::new();
        let _ = self.fetch_children(&mut zones)?;
        Ok(zones)
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

    pub fn fetch_hostinterfaces(&self) -> Result<Vec<HostInterface>, Error> {
        let mut hostinterfaces = Vec::<HostInterface>::new();
        let _ = self.fetch_children(&mut hostinterfaces)?;
        Ok(hostinterfaces)
    }

    pub fn fetch_routingpolicies(&self) -> Result<Vec<RoutingPolicy>, Error> {
        let mut routingpolicies = Vec::<RoutingPolicy>::new();
        let _ = self.fetch_children(&mut routingpolicies)?;
        Ok(routingpolicies)
    }

    pub fn fetch_uplinkroutedistinguishers(&self) -> Result<Vec<UplinkRD>, Error> {
        let mut uplinkroutedistinguishers = Vec::<UplinkRD>::new();
        let _ = self.fetch_children(&mut uplinkroutedistinguishers)?;
        Ok(uplinkroutedistinguishers)
    }

    pub fn fetch_applicationservices(&self) -> Result<Vec<ApplicationService>, Error> {
        let mut applicationservices = Vec::<ApplicationService>::new();
        let _ = self.fetch_children(&mut applicationservices)?;
        Ok(applicationservices)
    }

    pub fn fetch_vrsconfigs(&self) -> Result<Vec<VCenterVRSConfig>, Error> {
        let mut vrsconfigs = Vec::<VCenterVRSConfig>::new();
        let _ = self.fetch_children(&mut vrsconfigs)?;
        Ok(vrsconfigs)
    }

    pub fn fetch_users(&self) -> Result<Vec<User>, Error> {
        let mut users = Vec::<User>::new();
        let _ = self.fetch_children(&mut users)?;
        Ok(users)
    }

    pub fn fetch_nsgateways(&self) -> Result<Vec<NSGateway>, Error> {
        let mut nsgateways = Vec::<NSGateway>::new();
        let _ = self.fetch_children(&mut nsgateways)?;
        Ok(nsgateways)
    }

    pub fn fetch_nsgatewaytemplates(&self) -> Result<Vec<NSGatewayTemplate>, Error> {
        let mut nsgatewaytemplates = Vec::<NSGatewayTemplate>::new();
        let _ = self.fetch_children(&mut nsgatewaytemplates)?;
        Ok(nsgatewaytemplates)
    }

    pub fn fetch_nsggroups(&self) -> Result<Vec<NSGGroup>, Error> {
        let mut nsggroups = Vec::<NSGGroup>::new();
        let _ = self.fetch_children(&mut nsggroups)?;
        Ok(nsggroups)
    }

    pub fn fetch_nsgredundancygroups(&self) -> Result<Vec<NSRedundantGatewayGroup>, Error> {
        let mut nsgredundancygroups = Vec::<NSRedundantGatewayGroup>::new();
        let _ = self.fetch_children(&mut nsgredundancygroups)?;
        Ok(nsgredundancygroups)
    }

    pub fn fetch_vsps(&self) -> Result<Vec<VSP>, Error> {
        let mut vsps = Vec::<VSP>::new();
        let _ = self.fetch_children(&mut vsps)?;
        Ok(vsps)
    }

    pub fn fetch_staticroutes(&self) -> Result<Vec<StaticRoute>, Error> {
        let mut staticroutes = Vec::<StaticRoute>::new();
        let _ = self.fetch_children(&mut staticroutes)?;
        Ok(staticroutes)
    }

    pub fn fetch_statisticscollector(&self) -> Result<Vec<StatsCollectorInfo>, Error> {
        let mut statisticscollector = Vec::<StatsCollectorInfo>::new();
        let _ = self.fetch_children(&mut statisticscollector)?;
        Ok(statisticscollector)
    }

    pub fn fetch_subnets(&self) -> Result<Vec<Subnet>, Error> {
        let mut subnets = Vec::<Subnet>::new();
        let _ = self.fetch_children(&mut subnets)?;
        Ok(subnets)
    }

    pub fn fetch_ducgroups(&self) -> Result<Vec<DUCGroup>, Error> {
        let mut ducgroups = Vec::<DUCGroup>::new();
        let _ = self.fetch_children(&mut ducgroups)?;
        Ok(ducgroups)
    }

    pub fn fetch_multicastchannelmaps(&self) -> Result<Vec<MultiCastChannelMap>, Error> {
        let mut multicastchannelmaps = Vec::<MultiCastChannelMap>::new();
        let _ = self.fetch_children(&mut multicastchannelmaps)?;
        Ok(multicastchannelmaps)
    }

    pub fn fetch_autodiscoveredgateways(&self) -> Result<Vec<AutoDiscoveredGateway>, Error> {
        let mut autodiscoveredgateways = Vec::<AutoDiscoveredGateway>::new();
        let _ = self.fetch_children(&mut autodiscoveredgateways)?;
        Ok(autodiscoveredgateways)
    }

    pub fn fetch_systemconfigs(&self) -> Result<Vec<SystemConfig>, Error> {
        let mut systemconfigs = Vec::<SystemConfig>::new();
        let _ = self.fetch_children(&mut systemconfigs)?;
        Ok(systemconfigs)
    }
}