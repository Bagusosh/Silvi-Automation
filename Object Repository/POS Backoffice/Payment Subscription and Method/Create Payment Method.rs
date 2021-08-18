<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Payment Method</name>
   <tag></tag>
   <elementGuidId>65ad5751-75eb-4bd4-aeb4-a76f82511ac7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{   \r\n    \&quot;payment_name\&quot; : \&quot;Cash\&quot;,\r\n\t\&quot;payment_fee\&quot; : 0,\r\n\t\&quot;payment_issuer\&quot; : \&quot;CASH-IT\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/payment-method</restUrl>
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
      <id>31c4ff03-040a-4408-a56f-39376c1e3492</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
