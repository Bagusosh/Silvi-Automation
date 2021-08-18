<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Merchant Operational Hour</name>
   <tag></tag>
   <elementGuidId>028caf85-692c-40e8-9f6f-42d1f78057a6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;start_time\&quot; : \&quot;09:00\&quot;,\r\n    \&quot;end_time\&quot; : \&quot;17:00\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}/merchant/${userId}/operational-hour/${operationHourId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-Staging</defaultValue>
      <description></description>
      <id>cfc47aa9-04e0-45d4-974b-70cac9033a63</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>abc4d3aa-de97-4ba6-b3b6-b004f88028d3</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.operationHourId</defaultValue>
      <description></description>
      <id>5f91bca2-b289-4d41-b448-5b22bab7469c</id>
      <masked>false</masked>
      <name>operationHourId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
