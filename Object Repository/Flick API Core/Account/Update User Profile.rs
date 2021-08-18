<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update User Profile</name>
   <tag></tag>
   <elementGuidId>8133dc67-afaf-41d9-9042-5d7bc7f702a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;nama&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;alamat&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;openTime&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;tanggalLahir&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;tempatLahir&quot;,
      &quot;value&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>/v1/users/${userId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>4d7c8de7-bd3a-47d3-8f2f-4665d8b02835</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
