<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Subscription </name>
   <tag></tag>
   <elementGuidId>960cd729-6b8d-4cd2-8772-34967477bdbc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot; {       \r\n    \&quot;subscription_name\&quot;: \&quot;30 Day\&quot;,\r\n    \&quot;subscription_image\&quot;: \&quot;\&quot;,\r\n    \&quot;subscription_price\&quot;: 200000,\r\n    \&quot;subscription_description\&quot;: \&quot;\&quot;,\r\n    \&quot;subscription_duration\&quot;: 720\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/subscription/${subscriptionId}</restUrl>
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
      <id>0120420b-5b04-465b-9a63-03db353f4cc3</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.subscriptionId</defaultValue>
      <description></description>
      <id>e057491c-745e-4ee7-a06e-488f4dfd86e1</id>
      <masked>false</masked>
      <name>subscriptionId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
