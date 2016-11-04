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
pub use l2domaintemplate::L2DomainTemplate;
pub use l7applicationsignature::L7applicationsignature;
pub use ratelimiter::RateLimiter;
pub use gateway::Gateway;
pub use gatewaytemplate::GatewayTemplate;
pub use patnatpool::PATNATPool;
pub use ldapconfiguration::LDAPConfiguration;
pub use redundancygroup::RedundancyGroup;
pub use performancemonitor::PerformanceMonitor;
pub use metadata::Metadata;
pub use metadatatag::MetadataTag;
pub use networkmacrogroup::NetworkMacroGroup;
pub use networkperformancemeasurement::NetworkPerformanceMeasurement;
pub use keyservermonitor::KeyServerMonitor;
pub use zfbrequest::ZFBRequest;
pub use bgpprofile::BGPProfile;
pub use egressqospolicy::EgressQOSPolicy;
pub use sharednetworkresource::SharedNetworkResource;
pub use ikecertificate::IKECertificate;
pub use ikeencryptionprofile::IKEEncryptionprofile;
pub use ikegateway::IKEGateway;
pub use ikegatewayprofile::IKEGatewayProfile;
pub use ikepsk::IKEPSK;
pub use alarm::Alarm;
pub use allalarm::AllAlarm;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use enterprisenetwork::EnterpriseNetwork;
pub use enterprisesecurity::EnterpriseSecurity;
pub use job::Job;
pub use domain::Domain;
pub use domaintemplate::DomainTemplate;
pub use container::Container;
pub use routingpolicy::RoutingPolicy;
pub use application::Application;
pub use applicationperformancemanagement::Applicationperformancemanagement;
pub use applicationservice::ApplicationService;
pub use group::Group;
pub use groupkeyencryptionprofile::GroupKeyEncryptionProfile;
pub use dscpforwardingclasstable::DSCPForwardingClassTable;
pub use user::User;
pub use nsgateway::NSGateway;
pub use nsgatewaytemplate::NSGatewayTemplate;
pub use nsredundantgatewaygroup::NSRedundantGatewayGroup;
pub use publicnetworkmacro::PublicNetworkMacro;
pub use multicastlist::MultiCastList;
pub use avatar::Avatar;
pub use eventlog::EventLog;
pub use externalappservice::ExternalAppService;
pub use externalservice::ExternalService;


#[derive(Serialize, Deserialize)]
pub struct Enterprise<'a> {
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
    
    #[serde(rename="LDAPAuthorizationEnabled")]
    ldap_authorization_enabled: bool,
    
    #[serde(rename="LDAPEnabled")]
    ldap_enabled: bool,
    
    #[serde(rename="BGPEnabled")]
    bgp_enabled: bool,
    
    #[serde(rename="DHCPLeaseInterval")]
    dhcp_lease_interval: u64,
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="receiveMultiCastListID")]
    receive_multi_cast_list_id: Option<String>,
    
    #[serde(rename="sendMultiCastListID")]
    send_multi_cast_list_id: Option<String>,
    description: Option<String>,
    
    #[serde(rename="dictionaryVersion")]
    dictionary_version: u64,
    
    #[serde(rename="allowAdvancedQOSConfiguration")]
    allow_advanced_qos_configuration: bool,
    
    #[serde(rename="allowGatewayManagement")]
    allow_gateway_management: bool,
    
    #[serde(rename="allowTrustedForwardingClass")]
    allow_trusted_forwarding_class: bool,
    
    #[serde(rename="allowedForwardingClasses")]
    allowed_forwarding_classes: Vec<Option<String>>,
    
    #[serde(rename="floatingIPsQuota")]
    floating_ips_quota: u64,
    
    #[serde(rename="floatingIPsUsed")]
    floating_ips_used: u64,
    
    #[serde(rename="enableApplicationPerformanceManagement")]
    enable_application_performance_management: bool,
    
    #[serde(rename="encryptionManagementMode")]
    encryption_management_mode: Option<String>,
    
    #[serde(rename="enterpriseProfileID")]
    enterprise_profile_id: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="localAS")]
    local_as: u64,
    
    #[serde(rename="associatedEnterpriseSecurityID")]
    associated_enterprise_security_id: Option<String>,
    
    #[serde(rename="associatedGroupKeyEncryptionProfileID")]
    associated_group_key_encryption_profile_id: Option<String>,
    
    #[serde(rename="associatedKeyServerMonitorID")]
    associated_key_server_monitor_id: Option<String>,
    
    #[serde(rename="customerID")]
    customer_id: u64,
    
    #[serde(rename="avatarData")]
    avatar_data: Option<String>,
    
    #[serde(rename="avatarType")]
    avatar_type: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for Enterprise<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "enterprise"
    }

    fn group_path() -> &'static str {
        "enterprises"
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

impl<'a> Enterprise<'a> {

    fn fetch_l2domains(&self) -> Result<Vec<L2Domain>, BambouError> {
        let mut l2domains = Vec::<L2Domain>::new();
        try!(self.fetch_children(&mut l2domains));
        Ok(l2domains)
    }

    fn fetch_l2domaintemplates(&self) -> Result<Vec<L2DomainTemplate>, BambouError> {
        let mut l2domaintemplates = Vec::<L2DomainTemplate>::new();
        try!(self.fetch_children(&mut l2domaintemplates));
        Ok(l2domaintemplates)
    }

    fn fetch_l7applicationsignatures(&self) -> Result<Vec<L7applicationsignature>, BambouError> {
        let mut l7applicationsignatures = Vec::<L7applicationsignature>::new();
        try!(self.fetch_children(&mut l7applicationsignatures));
        Ok(l7applicationsignatures)
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

    fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, BambouError> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        try!(self.fetch_children(&mut patnatpools));
        Ok(patnatpools)
    }

    fn fetch_ldapconfigurations(&self) -> Result<Vec<LDAPConfiguration>, BambouError> {
        let mut ldapconfigurations = Vec::<LDAPConfiguration>::new();
        try!(self.fetch_children(&mut ldapconfigurations));
        Ok(ldapconfigurations)
    }

    fn fetch_redundancygroups(&self) -> Result<Vec<RedundancyGroup>, BambouError> {
        let mut redundancygroups = Vec::<RedundancyGroup>::new();
        try!(self.fetch_children(&mut redundancygroups));
        Ok(redundancygroups)
    }

    fn fetch_performancemonitors(&self) -> Result<Vec<PerformanceMonitor>, BambouError> {
        let mut performancemonitors = Vec::<PerformanceMonitor>::new();
        try!(self.fetch_children(&mut performancemonitors));
        Ok(performancemonitors)
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

    fn fetch_networkmacrogroups(&self) -> Result<Vec<NetworkMacroGroup>, BambouError> {
        let mut networkmacrogroups = Vec::<NetworkMacroGroup>::new();
        try!(self.fetch_children(&mut networkmacrogroups));
        Ok(networkmacrogroups)
    }

    fn fetch_networkperformancemeasurements(&self) -> Result<Vec<NetworkPerformanceMeasurement>, BambouError> {
        let mut networkperformancemeasurements = Vec::<NetworkPerformanceMeasurement>::new();
        try!(self.fetch_children(&mut networkperformancemeasurements));
        Ok(networkperformancemeasurements)
    }

    fn fetch_keyservermonitors(&self) -> Result<Vec<KeyServerMonitor>, BambouError> {
        let mut keyservermonitors = Vec::<KeyServerMonitor>::new();
        try!(self.fetch_children(&mut keyservermonitors));
        Ok(keyservermonitors)
    }

    fn fetch_zfbrequests(&self) -> Result<Vec<ZFBRequest>, BambouError> {
        let mut zfbrequests = Vec::<ZFBRequest>::new();
        try!(self.fetch_children(&mut zfbrequests));
        Ok(zfbrequests)
    }

    fn fetch_bgpprofiles(&self) -> Result<Vec<BGPProfile>, BambouError> {
        let mut bgpprofiles = Vec::<BGPProfile>::new();
        try!(self.fetch_children(&mut bgpprofiles));
        Ok(bgpprofiles)
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

    fn fetch_ikecertificates(&self) -> Result<Vec<IKECertificate>, BambouError> {
        let mut ikecertificates = Vec::<IKECertificate>::new();
        try!(self.fetch_children(&mut ikecertificates));
        Ok(ikecertificates)
    }

    fn fetch_ikeencryptionprofiles(&self) -> Result<Vec<IKEEncryptionprofile>, BambouError> {
        let mut ikeencryptionprofiles = Vec::<IKEEncryptionprofile>::new();
        try!(self.fetch_children(&mut ikeencryptionprofiles));
        Ok(ikeencryptionprofiles)
    }

    fn fetch_ikegateways(&self) -> Result<Vec<IKEGateway>, BambouError> {
        let mut ikegateways = Vec::<IKEGateway>::new();
        try!(self.fetch_children(&mut ikegateways));
        Ok(ikegateways)
    }

    fn fetch_ikegatewayprofiles(&self) -> Result<Vec<IKEGatewayProfile>, BambouError> {
        let mut ikegatewayprofiles = Vec::<IKEGatewayProfile>::new();
        try!(self.fetch_children(&mut ikegatewayprofiles));
        Ok(ikegatewayprofiles)
    }

    fn fetch_ikepsks(&self) -> Result<Vec<IKEPSK>, BambouError> {
        let mut ikepsks = Vec::<IKEPSK>::new();
        try!(self.fetch_children(&mut ikepsks));
        Ok(ikepsks)
    }

    fn fetch_alarms(&self) -> Result<Vec<Alarm>, BambouError> {
        let mut alarms = Vec::<Alarm>::new();
        try!(self.fetch_children(&mut alarms));
        Ok(alarms)
    }

    fn fetch_allalarms(&self) -> Result<Vec<AllAlarm>, BambouError> {
        let mut allalarms = Vec::<AllAlarm>::new();
        try!(self.fetch_children(&mut allalarms));
        Ok(allalarms)
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

    fn fetch_enterprisenetworks(&self) -> Result<Vec<EnterpriseNetwork>, BambouError> {
        let mut enterprisenetworks = Vec::<EnterpriseNetwork>::new();
        try!(self.fetch_children(&mut enterprisenetworks));
        Ok(enterprisenetworks)
    }

    fn fetch_enterprisesecurities(&self) -> Result<Vec<EnterpriseSecurity>, BambouError> {
        let mut enterprisesecurities = Vec::<EnterpriseSecurity>::new();
        try!(self.fetch_children(&mut enterprisesecurities));
        Ok(enterprisesecurities)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_domains(&self) -> Result<Vec<Domain>, BambouError> {
        let mut domains = Vec::<Domain>::new();
        try!(self.fetch_children(&mut domains));
        Ok(domains)
    }

    fn fetch_domaintemplates(&self) -> Result<Vec<DomainTemplate>, BambouError> {
        let mut domaintemplates = Vec::<DomainTemplate>::new();
        try!(self.fetch_children(&mut domaintemplates));
        Ok(domaintemplates)
    }

    fn fetch_containers(&self) -> Result<Vec<Container>, BambouError> {
        let mut containers = Vec::<Container>::new();
        try!(self.fetch_children(&mut containers));
        Ok(containers)
    }

    fn fetch_routingpolicies(&self) -> Result<Vec<RoutingPolicy>, BambouError> {
        let mut routingpolicies = Vec::<RoutingPolicy>::new();
        try!(self.fetch_children(&mut routingpolicies));
        Ok(routingpolicies)
    }

    fn fetch_applications(&self) -> Result<Vec<Application>, BambouError> {
        let mut applications = Vec::<Application>::new();
        try!(self.fetch_children(&mut applications));
        Ok(applications)
    }

    fn fetch_applicationperformancemanagements(&self) -> Result<Vec<Applicationperformancemanagement>, BambouError> {
        let mut applicationperformancemanagements = Vec::<Applicationperformancemanagement>::new();
        try!(self.fetch_children(&mut applicationperformancemanagements));
        Ok(applicationperformancemanagements)
    }

    fn fetch_applicationservices(&self) -> Result<Vec<ApplicationService>, BambouError> {
        let mut applicationservices = Vec::<ApplicationService>::new();
        try!(self.fetch_children(&mut applicationservices));
        Ok(applicationservices)
    }

    fn fetch_groups(&self) -> Result<Vec<Group>, BambouError> {
        let mut groups = Vec::<Group>::new();
        try!(self.fetch_children(&mut groups));
        Ok(groups)
    }

    fn fetch_groupkeyencryptionprofiles(&self) -> Result<Vec<GroupKeyEncryptionProfile>, BambouError> {
        let mut groupkeyencryptionprofiles = Vec::<GroupKeyEncryptionProfile>::new();
        try!(self.fetch_children(&mut groupkeyencryptionprofiles));
        Ok(groupkeyencryptionprofiles)
    }

    fn fetch_dscpforwardingclasstables(&self) -> Result<Vec<DSCPForwardingClassTable>, BambouError> {
        let mut dscpforwardingclasstables = Vec::<DSCPForwardingClassTable>::new();
        try!(self.fetch_children(&mut dscpforwardingclasstables));
        Ok(dscpforwardingclasstables)
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

    fn fetch_publicnetworks(&self) -> Result<Vec<PublicNetworkMacro>, BambouError> {
        let mut publicnetworks = Vec::<PublicNetworkMacro>::new();
        try!(self.fetch_children(&mut publicnetworks));
        Ok(publicnetworks)
    }

    fn fetch_multicastlists(&self) -> Result<Vec<MultiCastList>, BambouError> {
        let mut multicastlists = Vec::<MultiCastList>::new();
        try!(self.fetch_children(&mut multicastlists));
        Ok(multicastlists)
    }

    fn fetch_avatars(&self) -> Result<Vec<Avatar>, BambouError> {
        let mut avatars = Vec::<Avatar>::new();
        try!(self.fetch_children(&mut avatars));
        Ok(avatars)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
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
}
