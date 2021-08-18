<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Merchant Inventory</name>
   <tag></tag>
   <elementGuidId>0eda834d-9b92-4a52-a055-c7517960b756</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/merchant/${merchantId}/inventory</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BO-Silvi-Staging</defaultValue>
      <description></description>
      <id>e7671f80-82ca-4725-9220-1cccc8449d6d</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantId</defaultValue>
      <description></description>
      <id>3c106c48-4ec2-40a3-b6de-33147a9dea3c</id>
      <masked>false</masked>
      <name>merchantId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
