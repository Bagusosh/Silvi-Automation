<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PLN Prepaid Payment</name>
   <tag></tag>
   <elementGuidId>d4a5a2b2-4fce-49b2-a4a9-fb257770ba7d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;nomor\&quot; : \&quot;\&quot;,\r\n    \&quot;idProduk\&quot; : \&quot;\&quot;,\r\n    \&quot;total\&quot; : \&quot;\&quot;,\r\n    \&quot;trackingId\&quot; : \&quot;\&quot;,\r\n    \&quot;inquiryId\&quot; : \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v1/users/${userId}/payment/pln-prepaid</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseFlick-Staging</defaultValue>
      <description></description>
      <id>2f826e0b-fd97-48c6-bfdd-083abd7b6605</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>6af329be-4613-498d-86af-159613ffce2a</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
