use std::str::FromStr;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "applications")]
pub struct Applications {
    #[xml(child="application")]
    pub applications: Vec<Application>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "application")]
pub struct Application {
    #[xml(flatten_text = "name")]
    pub name: String,
    #[xml(child = "instance")]
    pub instances: Vec<Instance>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "instance")]
pub struct Instance {
    #[xml(flatten_text = "hostName")]
    pub hostname: String,
    #[xml(flatten_text = "app")]
    pub app: String,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "port")]
pub struct Port {
    #[xml(attr = "enable")]
    pub enabled: bool,
    #[xml(text)]
    pub value: u16,
}

#[derive(PartialEq, Debug)]
struct Foo;

impl FromStr for Foo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "foo" || s == "FOO" {
            Ok(Foo)
        } else {
            Err("invalid Foo".into())
        }
    }
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "root")]
struct Root {
    #[xml(attr = "foo")]
    foo: Foo,
    #[xml(attr = "bar")]
    bar: Option<String>,
    #[xml(attr = "baz")]
    baz: Option<usize>,
}

#[test]
fn test() -> XmlResult<()> {

    assert_eq!(
        Root::from_str(r#"<root foo="foo" baz="100"/>"#)?,
        Root {
            foo: Foo,
            bar: None,
            baz: Some(100)
        }
    );

    assert_eq!(
        Root::from_str(r#"<root foo="FOO" bar="bar"/>"#)?,
        Root {
            foo: Foo,
            bar: Some("bar".into()),
            baz: None
        }
    );

    assert!(Root::from_str(r#"<root foo="bar"/>"#).is_err());

    assert!(Root::from_str(r#"<root foo="foo" baz="baz"/>"#).is_err());

    Ok(())
}

mod tests {
    use std::str::FromStr;

    use crate::xml::structures::Applications;

    #[test]
    fn test_xml() {
        let xml = r#"<applications>
  <versions__delta>1</versions__delta>
  <apps__hashcode>UP_2_</apps__hashcode>
  <application>
    <name>BENCH</name>
    <instance>
      <hostName>localhost</hostName>
      <app>BENCH</app>
      <ipAddr>127.0.0.1</ipAddr>
      <status>UP</status>
      <overriddenstatus>UP</overriddenstatus>
      <port enabled="true">8080</port>
      <securePort enabled="false">443</securePort>
      <countryId>1</countryId>
      <dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>
      <leaseInfo>
        <renewalIntervalInSecs>30</renewalIntervalInSecs>
        <durationInSecs>90</durationInSecs>
        <registrationTimestamp>1616761261538</registrationTimestamp>
        <lastRenewalTimestamp>1616761921820</lastRenewalTimestamp>
        <evictionTimestamp>0</evictionTimestamp>
        <serviceUpTimestamp>1616761261439</serviceUpTimestamp>
      </leaseInfo>
      <metadata class="java.util.Collections$EmptyMap"/>
      <homePageUrl>/eureka</homePageUrl>
      <statusPageUrl></statusPageUrl>
      <healthCheckUrl></healthCheckUrl>
      <vipAddress>bench</vipAddress>
      <secureVipAddress>bench</secureVipAddress>
      <isCoordinatingDiscoveryServer>false</isCoordinatingDiscoveryServer>
      <lastUpdatedTimestamp>1616761261538</lastUpdatedTimestamp>
      <lastDirtyTimestamp>1616761261439</lastDirtyTimestamp>
      <actionType>ADDED</actionType>
    </instance>
    <instance>
      <hostName>localhost2</hostName>
      <app>BENCH</app>
      <ipAddr>127.0.0.1</ipAddr>
      <status>UP</status>
      <overriddenstatus>UP</overriddenstatus>
      <port enabled="true">8081</port>
      <securePort enabled="false">443</securePort>
      <countryId>1</countryId>
      <dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>
      <leaseInfo>
        <renewalIntervalInSecs>30</renewalIntervalInSecs>
        <durationInSecs>90</durationInSecs>
        <registrationTimestamp>1616761233029</registrationTimestamp>
        <lastRenewalTimestamp>1616761900818</lastRenewalTimestamp>
        <evictionTimestamp>0</evictionTimestamp>
        <serviceUpTimestamp>1616761232774</serviceUpTimestamp>
      </leaseInfo>
      <metadata class="java.util.Collections$EmptyMap"/>
      <homePageUrl>/eureka</homePageUrl>
      <statusPageUrl></statusPageUrl>
      <healthCheckUrl></healthCheckUrl>
      <vipAddress>bench</vipAddress>
      <secureVipAddress>bench</secureVipAddress>
      <isCoordinatingDiscoveryServer>false</isCoordinatingDiscoveryServer>
      <lastUpdatedTimestamp>1616761233030</lastUpdatedTimestamp>
      <lastDirtyTimestamp>1616761232774</lastDirtyTimestamp>
      <actionType>ADDED</actionType>
    </instance>
  </application>
</applications>"#;
    }
}