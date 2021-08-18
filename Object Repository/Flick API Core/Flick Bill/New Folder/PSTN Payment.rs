<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PSTN Payment</name>
   <tag></tag>
   <elementGuidId>5c92e4a7-af62-4396-a154-ee214535544d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;nomor\&quot; : \&quot;\&quot;,\r\n    \&quot;billId\&quot; : \&quot;\&quot;,\r\n    \&quot;total\&quot; : \&quot;\&quot;,\r\n    \&quot;trackingId\&quot; : \&quot;\&quot;,\r\n    \&quot;inquiryId\&quot; : \&quot;\&quot;,\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v1/users/${userId}}/payment/indihome</restUrl>
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
      <id>63be8bb1-412a-4e00-84f7-cca99b935467</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId}</defaultValue>
      <description></description>
      <id>5ffbb89a-57fd-4720-9910-ca007934994b</id>
      <masked>false</masked>
      <name>userId}</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
