<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Acquisition</name>
   <tag></tag>
   <elementGuidId>f0c657e8-1690-4d3e-ab7d-df5687b6f3e7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;status\&quot; : \&quot;ACCEPT\&quot;,\r\n    \&quot;table_amount\&quot; : 10\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-V2-Staging}/acquisition/${acquisitionId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Staging</defaultValue>
      <description></description>
      <id>0025563e-5591-4efa-96de-92af9963a85a</id>
      <masked>false</masked>
      <name>Silvi-V2-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.acquisitionId</defaultValue>
      <description></description>
      <id>17abd296-3150-4574-9809-619615fde463</id>
      <masked>false</masked>
      <name>acquisitionId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
