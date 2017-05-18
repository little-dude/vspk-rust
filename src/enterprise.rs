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
pub use networkmacrogroup::NetworkMacroGroup;
pub use networkperformancemeasurement::NetworkPerformanceMeasurement;
pub use keyservermonitor::KeyServerMonitor;
pub use zfbrequest::ZFBRequest;
pub use bgpprofile::BGPProfile;
pub use egressqospolicy::EgressQOSPolicy;
pub use sharednetworkresource::SharedNetworkResource;
pub use firewallacl::FirewallAcl;
pub use firewallrule::FirewallRule;
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
pub use trunk::Trunk;
pub use dscpforwardingclasstable::DSCPForwardingClassTable;
pub use user::User;
pub use nsgateway::NSGateway;
pub use nsgatewaytemplate::NSGatewayTemplate;
pub use nsggroup::NSGGroup;
pub use nsredundantgatewaygroup::NSRedundantGatewayGroup;
pub use publicnetworkmacro::PublicNetworkMacro;
pub use multicastlist::MultiCastList;
pub use avatar::Avatar;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default)]
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
    pub ldap_authorization_enabled: bool,
    
    #[serde(rename="LDAPEnabled")]
    pub ldap_enabled: bool,
    
    #[serde(rename="BGPEnabled")]
    pub bgp_enabled: bool,
    
    #[serde(rename="DHCPLeaseInterval")]
    pub dhcp_lease_interval: u64,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="receiveMultiCastListID")]
    pub receive_multi_cast_list_id: Option<String>,
    
    #[serde(rename="sendMultiCastListID")]
    pub send_multi_cast_list_id: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="dictionaryVersion")]
    pub dictionary_version: u64,
    
    #[serde(rename="allowAdvancedQOSConfiguration")]
    pub allow_advanced_qos_configuration: bool,
    
    #[serde(rename="allowGatewayManagement")]
    pub allow_gateway_management: bool,
    
    #[serde(rename="allowTrustedForwardingClass")]
    pub allow_trusted_forwarding_class: bool,
    
    #[serde(rename="allowedForwardingClasses")]
    pub allowed_forwarding_classes: Vec<Option<String>>,
    
    #[serde(rename="floatingIPsQuota")]
    pub floating_ips_quota: u64,
    
    #[serde(rename="floatingIPsUsed")]
    pub floating_ips_used: u64,
    
    #[serde(rename="enableApplicationPerformanceManagement")]
    pub enable_application_performance_management: bool,
    
    #[serde(rename="encryptionManagementMode")]
    pub encryption_management_mode: Option<String>,
    
    #[serde(rename="enterpriseProfileID")]
    pub enterprise_profile_id: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="localAS")]
    pub local_as: u64,
    
    #[serde(rename="associatedEnterpriseSecurityID")]
    pub associated_enterprise_security_id: Option<String>,
    
    #[serde(rename="associatedGroupKeyEncryptionProfileID")]
    pub associated_group_key_encryption_profile_id: Option<String>,
    
    #[serde(rename="associatedKeyServerMonitorID")]
    pub associated_key_server_monitor_id: Option<String>,
    
    #[serde(rename="customerID")]
    pub customer_id: u64,
    
    #[serde(rename="avatarData")]
    pub avatar_data: Option<String>,
    
    #[serde(rename="avatarType")]
    pub avatar_type: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for Enterprise<'a> {
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

impl<'a> Enterprise<'a> {

    pub fn fetch_l2domains(&self) -> Result<Vec<L2Domain>, Error> {
        let mut l2domains = Vec::<L2Domain>::new();
        let _ = self.fetch_children(&mut l2domains)?;
        Ok(l2domains)
    }

    pub fn fetch_l2domaintemplates(&self) -> Result<Vec<L2DomainTemplate>, Error> {
        let mut l2domaintemplates = Vec::<L2DomainTemplate>::new();
        let _ = self.fetch_children(&mut l2domaintemplates)?;
        Ok(l2domaintemplates)
    }

    pub fn fetch_l7applicationsignatures(&self) -> Result<Vec<L7applicationsignature>, Error> {
        let mut l7applicationsignatures = Vec::<L7applicationsignature>::new();
        let _ = self.fetch_children(&mut l7applicationsignatures)?;
        Ok(l7applicationsignatures)
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

    pub fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, Error> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        let _ = self.fetch_children(&mut patnatpools)?;
        Ok(patnatpools)
    }

    pub fn fetch_ldapconfigurations(&self) -> Result<Vec<LDAPConfiguration>, Error> {
        let mut ldapconfigurations = Vec::<LDAPConfiguration>::new();
        let _ = self.fetch_children(&mut ldapconfigurations)?;
        Ok(ldapconfigurations)
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

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_networkmacrogroups(&self) -> Result<Vec<NetworkMacroGroup>, Error> {
        let mut networkmacrogroups = Vec::<NetworkMacroGroup>::new();
        let _ = self.fetch_children(&mut networkmacrogroups)?;
        Ok(networkmacrogroups)
    }

    pub fn fetch_networkperformancemeasurements(&self) -> Result<Vec<NetworkPerformanceMeasurement>, Error> {
        let mut networkperformancemeasurements = Vec::<NetworkPerformanceMeasurement>::new();
        let _ = self.fetch_children(&mut networkperformancemeasurements)?;
        Ok(networkperformancemeasurements)
    }

    pub fn fetch_keyservermonitors(&self) -> Result<Vec<KeyServerMonitor>, Error> {
        let mut keyservermonitors = Vec::<KeyServerMonitor>::new();
        let _ = self.fetch_children(&mut keyservermonitors)?;
        Ok(keyservermonitors)
    }

    pub fn fetch_zfbrequests(&self) -> Result<Vec<ZFBRequest>, Error> {
        let mut zfbrequests = Vec::<ZFBRequest>::new();
        let _ = self.fetch_children(&mut zfbrequests)?;
        Ok(zfbrequests)
    }

    pub fn fetch_bgpprofiles(&self) -> Result<Vec<BGPProfile>, Error> {
        let mut bgpprofiles = Vec::<BGPProfile>::new();
        let _ = self.fetch_children(&mut bgpprofiles)?;
        Ok(bgpprofiles)
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

    pub fn fetch_firewallacls(&self) -> Result<Vec<FirewallAcl>, Error> {
        let mut firewallacls = Vec::<FirewallAcl>::new();
        let _ = self.fetch_children(&mut firewallacls)?;
        Ok(firewallacls)
    }

    pub fn fetch_firewallrules(&self) -> Result<Vec<FirewallRule>, Error> {
        let mut firewallrules = Vec::<FirewallRule>::new();
        let _ = self.fetch_children(&mut firewallrules)?;
        Ok(firewallrules)
    }

    pub fn fetch_ikecertificates(&self) -> Result<Vec<IKECertificate>, Error> {
        let mut ikecertificates = Vec::<IKECertificate>::new();
        let _ = self.fetch_children(&mut ikecertificates)?;
        Ok(ikecertificates)
    }

    pub fn fetch_ikeencryptionprofiles(&self) -> Result<Vec<IKEEncryptionprofile>, Error> {
        let mut ikeencryptionprofiles = Vec::<IKEEncryptionprofile>::new();
        let _ = self.fetch_children(&mut ikeencryptionprofiles)?;
        Ok(ikeencryptionprofiles)
    }

    pub fn fetch_ikegateways(&self) -> Result<Vec<IKEGateway>, Error> {
        let mut ikegateways = Vec::<IKEGateway>::new();
        let _ = self.fetch_children(&mut ikegateways)?;
        Ok(ikegateways)
    }

    pub fn fetch_ikegatewayprofiles(&self) -> Result<Vec<IKEGatewayProfile>, Error> {
        let mut ikegatewayprofiles = Vec::<IKEGatewayProfile>::new();
        let _ = self.fetch_children(&mut ikegatewayprofiles)?;
        Ok(ikegatewayprofiles)
    }

    pub fn fetch_ikepsks(&self) -> Result<Vec<IKEPSK>, Error> {
        let mut ikepsks = Vec::<IKEPSK>::new();
        let _ = self.fetch_children(&mut ikepsks)?;
        Ok(ikepsks)
    }

    pub fn fetch_alarms(&self) -> Result<Vec<Alarm>, Error> {
        let mut alarms = Vec::<Alarm>::new();
        let _ = self.fetch_children(&mut alarms)?;
        Ok(alarms)
    }

    pub fn fetch_allalarms(&self) -> Result<Vec<AllAlarm>, Error> {
        let mut allalarms = Vec::<AllAlarm>::new();
        let _ = self.fetch_children(&mut allalarms)?;
        Ok(allalarms)
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

    pub fn fetch_enterprisenetworks(&self) -> Result<Vec<EnterpriseNetwork>, Error> {
        let mut enterprisenetworks = Vec::<EnterpriseNetwork>::new();
        let _ = self.fetch_children(&mut enterprisenetworks)?;
        Ok(enterprisenetworks)
    }

    pub fn fetch_enterprisesecurities(&self) -> Result<Vec<EnterpriseSecurity>, Error> {
        let mut enterprisesecurities = Vec::<EnterpriseSecurity>::new();
        let _ = self.fetch_children(&mut enterprisesecurities)?;
        Ok(enterprisesecurities)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_domains(&self) -> Result<Vec<Domain>, Error> {
        let mut domains = Vec::<Domain>::new();
        let _ = self.fetch_children(&mut domains)?;
        Ok(domains)
    }

    pub fn fetch_domaintemplates(&self) -> Result<Vec<DomainTemplate>, Error> {
        let mut domaintemplates = Vec::<DomainTemplate>::new();
        let _ = self.fetch_children(&mut domaintemplates)?;
        Ok(domaintemplates)
    }

    pub fn fetch_containers(&self) -> Result<Vec<Container>, Error> {
        let mut containers = Vec::<Container>::new();
        let _ = self.fetch_children(&mut containers)?;
        Ok(containers)
    }

    pub fn fetch_routingpolicies(&self) -> Result<Vec<RoutingPolicy>, Error> {
        let mut routingpolicies = Vec::<RoutingPolicy>::new();
        let _ = self.fetch_children(&mut routingpolicies)?;
        Ok(routingpolicies)
    }

    pub fn fetch_applications(&self) -> Result<Vec<Application>, Error> {
        let mut applications = Vec::<Application>::new();
        let _ = self.fetch_children(&mut applications)?;
        Ok(applications)
    }

    pub fn fetch_applicationperformancemanagements(&self) -> Result<Vec<Applicationperformancemanagement>, Error> {
        let mut applicationperformancemanagements = Vec::<Applicationperformancemanagement>::new();
        let _ = self.fetch_children(&mut applicationperformancemanagements)?;
        Ok(applicationperformancemanagements)
    }

    pub fn fetch_applicationservices(&self) -> Result<Vec<ApplicationService>, Error> {
        let mut applicationservices = Vec::<ApplicationService>::new();
        let _ = self.fetch_children(&mut applicationservices)?;
        Ok(applicationservices)
    }

    pub fn fetch_groups(&self) -> Result<Vec<Group>, Error> {
        let mut groups = Vec::<Group>::new();
        let _ = self.fetch_children(&mut groups)?;
        Ok(groups)
    }

    pub fn fetch_groupkeyencryptionprofiles(&self) -> Result<Vec<GroupKeyEncryptionProfile>, Error> {
        let mut groupkeyencryptionprofiles = Vec::<GroupKeyEncryptionProfile>::new();
        let _ = self.fetch_children(&mut groupkeyencryptionprofiles)?;
        Ok(groupkeyencryptionprofiles)
    }

    pub fn fetch_trunks(&self) -> Result<Vec<Trunk>, Error> {
        let mut trunks = Vec::<Trunk>::new();
        let _ = self.fetch_children(&mut trunks)?;
        Ok(trunks)
    }

    pub fn fetch_dscpforwardingclasstables(&self) -> Result<Vec<DSCPForwardingClassTable>, Error> {
        let mut dscpforwardingclasstables = Vec::<DSCPForwardingClassTable>::new();
        let _ = self.fetch_children(&mut dscpforwardingclasstables)?;
        Ok(dscpforwardingclasstables)
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

    pub fn fetch_publicnetworks(&self) -> Result<Vec<PublicNetworkMacro>, Error> {
        let mut publicnetworks = Vec::<PublicNetworkMacro>::new();
        let _ = self.fetch_children(&mut publicnetworks)?;
        Ok(publicnetworks)
    }

    pub fn fetch_multicastlists(&self) -> Result<Vec<MultiCastList>, Error> {
        let mut multicastlists = Vec::<MultiCastList>::new();
        let _ = self.fetch_children(&mut multicastlists)?;
        Ok(multicastlists)
    }

    pub fn fetch_avatars(&self) -> Result<Vec<Avatar>, Error> {
        let mut avatars = Vec::<Avatar>::new();
        let _ = self.fetch_children(&mut avatars)?;
        Ok(avatars)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}