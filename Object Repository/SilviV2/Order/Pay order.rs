<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Pay order</name>
   <tag></tag>
   <elementGuidId>a5c8cba6-63d6-44ed-b78e-86402104ca2d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customer_id\&quot;: \&quot;78dcfc94-2b61-41e6-af3a-0831c18a4a92\&quot;,\n    \&quot;merchant_id\&quot; : \&quot;c010c7ed-527f-4514-8ff3-0aeb7acf37c0\&quot;,\n    \&quot;payment_method_id\&quot;: \&quot;679bf1d7-590b-46bf-877a-6285b787f661\&quot;,\n    \&quot;money_received\&quot;: 1300000,\n    \&quot;discount_merchant_id\&quot;: \&quot;37b82dd7-bc2f-4524-9ba9-5fa290e35507\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-V2-Local}/order-summaries/pay</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Local</defaultValue>
      <description></description>
      <id>cb30ab2c-fe45-4c9c-a930-cac8b47179ea</id>
      <masked>false</masked>
      <name>Silvi-V2-Local</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
