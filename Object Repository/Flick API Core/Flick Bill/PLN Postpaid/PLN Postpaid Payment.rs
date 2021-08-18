<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PLN Postpaid Payment</name>
   <tag></tag>
   <elementGuidId>2d5d7bc7-0f0e-4b6e-abab-ab2fa59c3b94</elementGuidId>
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
      <id>5e641ed9-886e-4602-be92-21ce3fe0700e</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>58ffd11f-d6f2-493f-a31f-39e90f3e5ad0</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
