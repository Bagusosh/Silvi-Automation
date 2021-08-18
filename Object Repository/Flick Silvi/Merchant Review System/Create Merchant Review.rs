<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Merchant Review</name>
   <tag></tag>
   <elementGuidId>8c14cc4f-a014-408e-aaa0-d19f6f9f84d8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;review_score\&quot;: 4.5,\r\n    \&quot;review_topic\&quot;: [\r\n        \&quot;PRICE\&quot;\r\n    ],\r\n    \&quot;review_comment\&quot;: \&quot;Waktu pengirimannya cepat!\&quot;,\r\n    \&quot;merchant_id\&quot;: \&quot;d9951416-2da0-4f37-b830-22af9f2ff5a5\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Flick-Silvi-Staging}/users/${userId}/review</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Flick-Silvi-Staging</defaultValue>
      <description></description>
      <id>4e1f3cab-a21e-4e1d-91e0-8c4f72653c64</id>
      <masked>false</masked>
      <name>Flick-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>f9f2277b-6e53-4b39-bcd9-f0acf7c1027d</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
